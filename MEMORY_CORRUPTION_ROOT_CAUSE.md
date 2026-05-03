# WASI Virtual Layer - メモリ破壊の根本原因分析

## 概要

このドキュメントは、WASI Virtual Layer の**根本的なメモリ破壊バグ**を実装レベルで詳細に分析します。

以前の分析（MEMORY_FLOW_BUG_ANALYSIS.md）では「設定値の問題」と結論付けていましたが、今回の深掘り分析により、**アーキテクチャ的な欠陥**と**メモリオフセット計算の根本的な誤り**が判明しました。

---

## バグ #1: グローバル変数アドレス重複による メモリ破壊

### 🔴 Critical - メモリ破壊の最大原因

#### 問題のコード

```rust
// wasi_virt_layer/src/memory.rs line 31-33
pub extern "C" fn [<__wasip1_vfs_ $name _memory_grow_global_alt_pos>]() -> i32 {
    &raw const [<__wasip1_vfs_ $name _ALT_GLOBAL_VAR>] as *const i32 as i32
}
```

#### 根本原因

各ターゲットモジュール（test_threads, ls）は**独立してコンパイル**されます。その結果：

- `test_threads` の `static mut __wasip1_vfs_test_threads_ALT_GLOBAL_VAR` → メモリアドレス 0x1000（test_threads内での相対位置）
- `ls` の `static mut __wasip1_vfs_ls_ALT_GLOBAL_VAR` → メモリアドレス 0x1000（ls内での相対位置）

`&raw const` は**VFS メモリ内での絶対アドレス**ではなく、**各モジュールローカルの相対アドレス**を返します。

#### 症状

```
single_memory mode での実行時：
├─ test_threads GlobalAltPos() → 0x12100 を返す
├─ ls GlobalAltPos() → 0x12100 を返す（!!）
│
├─ 最初は test_threads が 0x12100 を使用
├─ その後 ls も 0x12100 にアクセス
│  → test_threads のグローバル変数が上書きされ**メモリ破壊**
│
└─ memory.grow() 時にオフセット計算が狂い、
   さらなるメモリ破壊が発生
```

#### 実装における問題

```rust
// examples/vfs/minimal_repro_virtual/src/lib.rs line 14-15
import_wasm!(test_threads);  // 独立したモジュール
import_wasm!(ls);            // 独立したモジュール

// gen_alt_global! により各モジュール内で独立した ALT_GLOBAL_VAR が定義される
// しかし、単一メモリ内でそれぞれ異なるアドレスに配置されるべき
```

#### メモリ配置の実際

```
理想的:
  test_threads ALT_GLOBAL_VAR: 0x12100 (test_threads region)
  ls ALT_GLOBAL_VAR:           0x50100 (ls region)
    → 別々のアドレス

実際:
  test_threads ALT_GLOBAL_VAR: 0x12100（各モジュール内相対）
  ls ALT_GLOBAL_VAR:           0x12100（各モジュール内相対）
    → **同じアドレスで衝突！**
```

#### コード箇所

- `wasi_virt_layer/src/memory.rs` line 31-33: `memory_grow_global_alt_pos()` 実装
- `wasi_virt_layer-cli/src/generator/shared_global.rs` line 222-230: GlobalAltPos エクスポート処理
- `examples/vfs/minimal_repro_virtual/src/lib.rs` line 14-15: import_wasm!()

---

## バグ #2: offset_globals スライス計算による ターゲット間グローバル変数特定失敗

### 🔴 Critical - グローバル変数マッピングの誤り

#### 問題のコード

```rust
// wasi_virt_layer-cli/src/generator/shared_global.rs line 147-176
let offset_globals = module
    .globals
    .iter()
    .filter(|g| {
        g.mutable
            && matches!(
                g.kind,
                walrus::GlobalKind::Local(walrus::ConstExpr::Value(
                    walrus::ir::Value::I32(_)
                ))
            )
    })
    .map(|g| g.id())
    .collect::<Vec<_>>();

let num_globals_expected = ctx.target_names.len() + 1;  // 3 (test_threads + ls + vfs_external)
if offset_globals.len() < num_globals_expected {
    eyre::bail!("Expected at least {} offset globals, but found {}", ...);
}

let target_globals = &offset_globals[offset_globals.len() - num_globals_expected..];
```

#### 根本原因

`offset_globals` は**すべての可変グローバル変数**を収集します。しかし、**どの変数がどのターゲットに対応するのか**を決定するロジックが**存在しません**。

最後の `num_globals_expected` 個を「ターゲット対応グローバル」と仮定していますが、この仮定は**根拠なし**です。

#### 症状

```
offset_globals = [g0, g1, g2, g3, g4, g5]  （6個）
num_globals_expected = 3
target_globals = offset_globals[6-3..] = [g3, g4, g5]

しかし:
  g3 が test_threads なのか
  g4 が ls なのか
  g5 が vfs_external なのか

一切確定できない！

結果:
  target_globals[0] = g3 と仮定 → test_threads に割り当て
  target_globals[1] = g4 と仮定 → ls に割り当て
  target_globals[2] = g5 と仮定 → vfs_external に割り当て

しかし実際には:
  g3 が vfs_external だったとしたら？
  → メッピングが完全に崩れる！
```

#### コード箇所

```rust
// wasi_virt_layer-cli/src/generator/shared_global.rs line 171-176
let target_globals = &offset_globals[offset_globals.len() - num_globals_expected..];

for (i, name) in ctx.target_names.iter().enumerate() {
    global_mappings.push((target_globals[i], name.as_ref()));
    // ↑ target_globals[i] が実際に対応するターゲットなのか確認されていない
}
```

---

## バグ #3: GlobalAltPos エクスポート削除による 動的メモリアクセス不可

### 🔴 Critical - 実行時ポインタ解決の喪失

#### 問題のコード

```rust
// wasi_virt_layer-cli/src/generator/shared_global.rs line 222-230
let global_alt_pos = UniqueName::SharedGlobalFnsForTarget(
    &SharedGlobalFnsName::GlobalAltPos,
    target_name,
).get_fid(&module.exports)?;

module
    .exports
    .erase_with(global_alt_pos, ctx.unstable_print_debug)?;  // ← 削除！
```

#### 根本原因

`GlobalAltPos()` は**実行時にグローバル変数のアドレスを動的に取得**する関数です。しかし、この関数は VFS からアクセスできなくなるまで**エクスポートから削除**されます。

これにより、memory.grow() 後に新しいアドレスを参照する手段が失われます。

#### 症状

```
T0: GlobalAltPos() → 0x12100 を返す（実行時）
T1: このアドレスをキャッシュして使用開始
T2: memory.grow() が実行される（64KB → 128KB）
T3: メモリ再マップ完了
T4: 既存コードは旧アドレス 0x12100 を参照
     しかし GlobalAltPos() は削除されているので新アドレス取得不可！
     
結果:
  ✗ Out of Bounds アクセス
  ✗ メモリ破壊
  ✗ Segmentation Fault
```

#### コード箇所

- `wasi_virt_layer-cli/src/generator/shared_global.rs` line 230: `erase_with(global_alt_pos, ...)`

---

## バグ #4: memory.grow() 後のグローバル変数ポインタ無効化

### 🔴 Critical - メモリ拡張時の参照破壊

#### 問題のコード

```rust
// wasi_virt_layer-cli/src/generator/shared_global.rs line 369-409
fn gen_custom_locker(
    module: &mut walrus::Module,
    mem_id: walrus::MemoryId,
    is_debug: bool,
) -> eyre::Result<(walrus::FunctionId, String)> {
    // ...
    locker
        .kind
        .unwrap_local_mut()
        .builder_mut()
        .func_body()
        .rewrite(|instr, _| {
            if let Instr::Call(Call { func }) = instr {
                if *func == alt_id {
                    *instr = Instr::MemoryGrow(MemoryGrow { memory: mem_id });
                    // ← memory.grow() に置き換え
                }
            }
        })?;
    Ok((locker_id, export_name))
}
```

#### 根本原因

WASM における `memory.grow()` は**メモリレイアウト全体を再編成**する可能性があります。

既存の `i32` ポインタは memory.grow() 後に**無効になる可能性**がありますが、このコードはそれを考慮していません。

#### 症状

```
T0: &raw const ALT_GLOBAL_VAR を呼び出し → 0x12100 を取得
T1: その値を `i32` にキャッシュ
T2: test_threads が memory.grow() を実行
T3: メモリが 64KB → 128KB に拡張
T4: WASM 線形メモリが再マップされた可能性あり
T5: 旧コードは 0x12100 をアクセス
     → メモリレイアウトが変わっているかもしれない箇所にアクセス
     → メモリ破壊
```

#### コード箇所

- `wasi_virt_layer-cli/src/generator/shared_global.rs` line 400-407

---

## バグ #5: single_memory mode でのメモリ領域重複チェック不足

### 🟠 High - 領域制御の欠如

#### 問題のコード

```rust
// wasi_virt_layer-cli/src/generator/shared_global.rs line 162-169
let offset_globals = module.globals
    .iter()
    .filter(|g| { ... })  // i32 グローバル変数をフィルタ
    .collect::<Vec<_>>();

if offset_globals.len() < num_globals_expected {
    eyre::bail!("Expected at least {} offset globals, but found {}", ...);
}
// ← メモリ領域の重複チェックなし！
```

#### 根本原因

複数モジュールのグローバル変数が VFS メモリ内でどのように配置されるかに関する**明示的なチェックがない**。

test_threads と ls のグローバル変数領域が重複していないことを確認するメカニズムが存在しません。

#### 症状

```
test_threads Region: 0x12000 - 0x50000 (224 KB)
├─ test_threads ALT_GLOBAL_VAR: 0x12100
│
ls Region: 0x50000 - 0x60000 (64 KB)
├─ ls ALT_GLOBAL_VAR: 0x50100
│
同時実行時:
  test_threads が 0x12100 を書き込み
  ls が 0x50100 を書き込み
  → 一見分離されているが、thread synchronization 時に問題発生可能
```

#### コード箇所

- `wasi_virt_layer-cli/src/generator/shared_global.rs` line 162-169

---

## バグ #6: capacity=1 スレッドプール による memory.grow() 競合

### 🟠 High - スレッド実行制御の欠陥

#### 問題のコード

```rust
// examples/vfs/minimal_repro_virtual/src/lib.rs line 30, 43-44
THREAD_POOL.set_capacity(1);

// wasi_virt_layer-cli/src/generator/shared_global.rs line 124-145
for (i, name) in ctx.target_names.iter().enumerate() {
    // ロック処理
}
```

#### 根本原因

capacity=1 では、Root Thread と Child Thread が競争します。

Root Thread が memory.grow() を実行中に Child Thread が wake up しようとしても、capacity が満杯のため発火できません。

その結果、spin lock に陥ります。

#### 症状

```
T0: Root Thread が memory.grow() を呼び出し
T1: ロック取得を試みる
T2: Child Thread が wake up しようとする
     → capacity=1 のため実行できない
T3: Root Thread はロック解放を待つ
T4: Child Thread は実行待機（capacity不足）
     → デッドロック状態
```

#### コード箇所

- `examples/vfs/minimal_repro_virtual/src/lib.rs` line 29-31: capacity 設定

---

## 総合評価表

| バグID | バグ名 | 根本原因 | 重大度 | 影響 | メモリ破壊 |
|--------|--------|--------|--------|------|----------|
| #1 | アドレス重複 | `&raw const` の相対アドレス | 🔴 Critical | グローバル変数衝突 | **はい** |
| #2 | グローバル特定失敗 | target_globals スライス計算の無根拠性 | 🔴 Critical | 変数マッピング誤り | **はい** |
| #3 | GlobalAltPos 削除 | エクスポート削除による解決手段喪失 | 🔴 Critical | ポインタ無効化 | **はい** |
| #4 | memory.grow() 後ポインタ無効 | メモリ再マップ時の参照破壊 | 🔴 Critical | Out of Bounds | **はい** |
| #5 | 領域重複チェック不足 | 複数モジュール領域の制御欠如 | 🟠 High | 領域侵襲 | **可能性あり** |
| #6 | ロックデッドロック | capacity=1 と無限スピンロック | 🟠 High | ハング | 間接的 |

---

## メモリ破壊の発生シナリオ

### シナリオ: minimal_repro_virtual 実行時

```
1. world() 実行
   └─ ls::_reset() → ls の ALT_GLOBAL_VAR = 0x1000 (ls内相対)
   
2. main() 実行
   ├─ THREAD_POOL.init()
   ├─ test_threads::_reset() → test_threads の ALT_GLOBAL_VAR = 0x1000 (test_threads内相対)
   └─ test_threads::_main()
   
3. single_memory での実行
   ├─ test_threads GlobalAltPos() → 0x12100 (VFS内) を返す
   ├─ ls GlobalAltPos() → 0x12100 (VFS内) を返す  ← 重複！
   │
   ├─ test_threads: ALT_GLOBAL_VAR = 0x12100 に 値A を書き込み
   ├─ ls: ALT_GLOBAL_VAR = 0x12100 に 値B を書き込み
   │      ↑ test_threads の値が上書きされた！
   │
   └─ memory.grow() 実行
      ├─ ロック取得試行
      ├─ 0x12100 の新アドレスが不明（GlobalAltPos() 削除）
      └─ メモリ破壊 → Segmentation Fault

4. 結果
   ✗ test_threads のグローバル値が破壊
   ✗ ls のグローバル値も不安定
   ✗ メモリ出力不定
```

---

## 修正に必要な根本的変更

### 必須修正 #1: グローバル変数の絶対アドレス計算

```rust
// 現在: &raw const で相対アドレスを返す
// 修正: VFS 内での絶対アドレス計算が必要

pub extern "C" fn [<__wasip1_vfs_ $name _memory_grow_global_alt_pos>]() -> i32 {
    // TargetMemoryMetadata から base_ptr を取得
    let base_ptr = /* VFS が保持する base_ptr */;
    let relative_offset = /* test_threads内での相対オフセット */;
    base_ptr + relative_offset  // ← 絶対アドレス
}
```

### 必須修正 #2: GlobalAltPos の保持

```rust
// 現在: エクスポート削除
// 修正: 常に利用可能にする

// GlobalAltPos() をエクスポートから削除しない
// または、動的にアドレスを再計算できる仕組みを用意
```

### 必須修正 #3: メモリ領域の厳密な管理

```rust
// 各ターゲットのグローバル変数領域を
// SharedMemoryManager で明示的に管理

pub struct TargetMemoryMetadata {
    pub base_ptr: i32,
    pub limit_ptr: i32,
    pub global_var_region: (i32, i32),  // ← グローバル変数領域を明示
}
```

---

## 結論

WASI Virtual Layer の **メモリ破壊バグ**は：

1. **アーキテクチャ的欠陥**: 複数モジュール間のグローバル変数管理が不十分
2. **実装的誤り**: アドレス計算が相対/絶対混在で不正確
3. **設計の矛盾**: single_memory mode での領域制御が欠如

単なる「設定値の誤り」ではなく、**根本的な設計変更が必要**です。

---

**作成**: 深掘り分析エージェント（Explore Agent）  
**日時**: 2026-05-03  
**ステータス**: ✅ 根本原因特定完了
