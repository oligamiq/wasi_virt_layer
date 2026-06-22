# Own Memory アーキテクチャ

`--own-memory` は、VFS とターゲット Wasm を同じ線形メモリに結合しつつ、各 Wasm が「自分専用のメモリを持っている」ように見せるための実行モデルです。主な目的は、WASI 呼び出しやスレッド実行時にターゲットのポインタをコピーせず扱いながら、ターゲット同士のメモリ領域を分離することです。

このページでは、現在の最終アーキテクチャを説明します。重要な不変条件は次の 1 つです。

`memory_reserve::<T>(pages)` は物理的な容量と配置だけを変え、ターゲットの論理メモリサイズは変えません。ターゲット内の生の `memory.grow` は、その予約済み物理領域の中で論理サイズだけを増やします。

## 全体像

結合前の各 Wasm は、自分の `memory 0` を持っています。`--own-memory` では、結合後にそれらを 1 つの物理メモリへ配置し直します。

```text
結合前

VFS memory 0        target A memory 0       target B memory 0
+-------------+    +------------------+    +------------------+
| VFS data    |    | A data/heap      |    | B data/heap      |
+-------------+    +------------------+    +------------------+

結合後の物理 memory 0

+-------------+------------------+------------------+-------------+
| VFS region  | target A region  | target B region  | free/reserve|
+-------------+------------------+------------------+-------------+
              ^                  ^
              |                  |
       A physical offset   B physical offset
```

ターゲットのロード、ストア、`memory.copy` などは、元のアドレスにターゲットごとの物理オフセットを足してから、結合後の `memory 0` に対して実行されます。ターゲット自身から見ると、アドレス `0` は今まで通り自分のメモリ先頭です。VFS や lowering 後の実行系から見ると、同じアドレスは物理メモリ内の `physical_offset + 0` です。

## 論理サイズと物理配置

Own Memory では、メモリの「大きさ」を 2 種類に分けて扱います。

| 種類 | 意味 | 変更する操作 |
| --- | --- | --- |
| 論理サイズ | ターゲット内の `memory.size` やアロケータが見るページ数 | ターゲットの生 `memory.grow` |
| 物理配置 | 結合後の `memory 0` 上で、そのターゲット用に確保された領域と開始オフセット | `memory_reserve::<T>(pages)` |

この分離により、VFS は実行前に十分な物理領域を確保できます。一方で、ターゲットのアロケータは通常の Wasm と同じように `memory.grow` を使い、必要になった時点で論理サイズを増やします。

## `memory_reserve::<T>(pages)`

`memory_reserve::<T>(pages)` は、型 `T` で指定した Wasm の own-memory 領域に、追加の物理ページを予約します。`memory_reserve_self(pages)` は VFS 自身の領域を予約します。

この API は次の性質を持ちます。

- 対象 Wasm の物理領域を拡張する。
- 後続のターゲット領域を必要に応じて後方へシフトする。
- 対象 Wasm の論理 `memory.size` は増やさない。
- ターゲット内の後続 `memory.grow` が成功できるだけの headroom を作る。
- 物理メモリの再配置が起きるため、スレッド開始前、またはメモリアクセスが安全に停止しているタイミングで呼ぶ必要がある。

典型的な VFS 初期化では、ターゲット実行やスレッド起動の前に必要な予約を済ませます。

```rust
memory_reserve_self(1024)?;
memory_reserve::<target_wasm>(3200)?;

target_wasm::_reset();
target_wasm::_start();
target_wasm::_main();
```

この設計で重要なのは、予約と grow の責務を混ぜないことです。予約が論理サイズまで増やしてしまうと、ターゲットの生 `memory.grow` から見ると既にメモリが増えたことになり、実際のアロケータ要求に使える余地がなくなります。

## 生の `memory.grow` の lowering

Own Memory モードでは、ターゲット内の生 `memory.grow` は物理 `memory.grow` としては実行されません。代わりに、ターゲットごとの論理 grow helper への呼び出しに置換されます。

```text
target memory.grow(delta)
  ↓ lowering
logical_grow(target, delta)
  ↓
予約済み物理領域の範囲内なら logical_size += delta
範囲外なら -1 を返す
```

この helper は、現在の論理サイズと要求ページ数を足し、予約済み物理領域の上限を超えないか確認します。成功時は Wasm の `memory.grow` と同じく、grow 前の論理ページ数を返します。失敗時は `-1` を返します。

つまり、物理メモリを実際に広げたり、後続ターゲットをシフトしたりするのは `memory_reserve` の役割です。ターゲットの `memory.grow` は、その物理 headroom を消費して論理的に見える範囲を広げるだけです。

## ターゲットメモリ操作の remap

結合直後、ターゲット由来の命令は一度ターゲット専用のメモリ index に割り当てられます。その後、単一メモリへの lowering で、実際のアドレス計算に物理オフセットが加えられます。

対象になる操作はロード、ストア、`memory.size`、`memory.grow` だけではありません。bulk-memory 命令も同じくターゲットメモリへ remap する必要があります。

- `memory.copy`
- `memory.fill`
- `memory.init`
- `memory.discard`

特に `memory.copy` は、コピー元とコピー先の memory index を別々に持ちます。そのため、コピー元だけ、またはコピー先だけを誤って VFS memory のままにすると、ターゲットの `println!` やバッファ操作が別領域の NUL バイトを読んでしまいます。

## Memory director と物理オフセット

VFS がターゲットのポインタを扱う場合、ターゲット相対アドレスを結合後の物理アドレスへ変換する必要があります。この変換を担うのが `*_memory_director` です。

```text
target pointer p
  ↓ memory_director
physical_offset_global + p
  ↓
combined memory 0 上の実アドレス
```

post-combine 段階では、前にあるメモリの `memory.size` を合計してオフセットを求める形の director が生成されます。しかし `memory_reserve` によって物理配置が変わると、初期の `memory.size` 合計では現在の物理開始位置を表せません。

そのため lowering 後の director は、現在の物理開始位置を保持する offset global を読む形へ置換されます。この global は、`memory_reserve` によってターゲット領域がシフトされた後の位置を表します。

## スレッドと stdout

Own Memory とスレッドを組み合わせる場合、物理再配置と並行メモリアクセスの衝突を避ける必要があります。

- 通常のターゲットメモリ操作は、threads 有効時に共有メモリの読込側ガードを通る。
- 物理再配置を伴う `memory_reserve` wrapper は、書込側の排他が必要になる。
- `memory_director` で得た物理ポインタは、ガードの有効期間を超えて保持してはならない。

stdout などの WASI 呼び出しでは、VFS がターゲットの iovec やバッファポインタを解決して host 側の `fd_write` へ渡します。このとき director が古い物理オフセットを使うと、ターゲット文字列ではなく別領域を読んでしまいます。bulk-memory remap と director の offset global 化は、ターゲットの `println!` を正しい領域から読ませるためにも必要です。

## 実行時の流れ

典型的な own-memory 実行は次の順序になります。

```text
1. VFS が own_memory! で対象 Wasm を宣言する
2. VFS 初期化時に memory_reserve_self / memory_reserve::<T> を呼ぶ
3. reserve wrapper が物理領域を確保し、必要なら後続領域をシフトする
4. target _reset / _start / _main を呼ぶ
5. target の memory.grow は logical grow helper に入る
6. target のメモリ操作は physical offset を加えて memory 0 にアクセスする
7. WASI 呼び出しでは memory_director が target pointer を物理ポインタへ変換する
```

## 回帰テストで守るべき点

Own Memory の修正では、少なくとも次の観点を確認します。

- `memory_reserve` が対象 Wasm の論理サイズを増やさないこと。
- ターゲットの生 `memory.grow` が logical grow helper に置換されること。
- reserve 後の `*_memory_director` が現在の physical offset global を使うこと。
- ターゲット定義の bulk-memory ops がターゲットメモリへ remap されること。
- own-memory smoke test でスレッド内 stdout が NUL にならず、期待文字列として出力されること。
- own-memory pool integration が成功すること。

## 既知の注意点

現在の lowering は、post-combine が生成する `*_memory_director` の命令形を検出して置換します。そのため、post-combine 側の director 生成形を変える場合は、lowering 側の検出ロジックと回帰テストも同時に更新する必要があります。

また、スレッド実行中の `_reset()` は危険です。実行中スレッドのスタック、TLS、同期プリミティブ、共有グローバルを破壊する可能性があります。ターゲット再初期化は、全スレッドが終了し、メモリアクセスが安全に停止している状態で行う必要があります。
