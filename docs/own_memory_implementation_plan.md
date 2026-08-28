# Own-Memory Mode (Final Design Plan)

おっしゃる通りです。完全に理解しました。「既存の `memory.grow`（ターゲット内部の自動拡張）」と、「インポートされる `memory_grow`（VFSが明示的に呼ぶもの）」の役割を明確に分ける必要がありました。

`--own-memory` の真の目的は、**「ターゲット自身による予期せぬタイミングでのメモリ拡張（自動拡張）を禁止し、VFS側（ユーザ）が任意のタイミングで安全にメモリを拡張できるようにすること」**ですね。

したがって、以下の挙動が正解となります：
1. **ターゲットの内部 `memory.grow`**: 常に `-1` を返し、失敗しなければならない（自動拡張の禁止）。
2. **エクスポートされる `memory_grow`**: ロックを持たず、物理 `memory.grow` とメモリのシフト・グローバル変数の更新を正しく実行し、成功しなければならない（VFSによる手動拡張）。
3. **エクスポートされる `memory_size`**: 正確な論理サイズを計算して返さなければならない。ターゲット内部の `memory.size` もこれを呼び出して成功する。

## Proposed Changes

## 修正方針

### 1. `wasi_virt_layer/src/memory.rs` (新規または既存の修正)
`own_memory!` マクロを新しく定義します。
このマクロは、対象の全ターゲット（Wasm）を受け取り、Rust内でユーザが呼び出せる公開関数 `pub fn memory_grow` と `pub fn memory_size` を生成します。

- **`memory_grow(wasm: WasmAccessName, pages: i32) -> i32`**:
  - 対象の `wasm` に応じて内部の `match` で分岐。
  - `core::arch::wasm32::memory_grow` で物理メモリを拡張。
  - 以降のターゲットのオフセットグローバルを `__wasip1_vfs_{name}_memory_grow_global_alt_get` / `_set` を用いて取得・更新し、`memory.copy` (または `ptr::copy`) でデータをシフトします。
- **`memory_size(wasm: WasmAccessName) -> i32`**:
  - `memory_grow` 同様、内部で分岐し、グローバル変数を用いて対象Wasmの論理メモリサイズを計算して返します。

これらは `extern "C"` ではなく**通常のRust関数**として公開され、VFSの利用者が自身でメモリ管理を行う際に呼び出します。

### 2. `wasi_virt_layer-cli/src/wasm_stream/passes/multi_memory_lowering.rs`
`--own-memory` が指定された場合のターゲット内部のメモリ命令の挙動を変更します。

- **`memory.grow` の置換**:
  - ターゲットモジュール内部の `memory.grow` 命令は、`Drop` して常に `-1` を返すダミー (`I32Const(-1)`) に置き換えます。
  - これにより、ユーザーの意図通り「既存の `memory.grow` は必ず失敗」します。
- **`memory.size` のインライン化**:
  - ターゲットモジュール内部の `memory.size` 命令は、VFSの関数を呼び出すのではなく、内部のグローバル変数 (`this_gid`, `next_gid`) を用いたサイズ計算処理をインライン展開するか、または `!own_memory` 時に生成されるラッパーと同じロジックを直接埋め込みます。

これにより、VFSとターゲットのWasmレベルでのエクスポート/インポートの依存（`own_size_fns_array` 等の解決）を完全に不要にし、CLI側のエクスポート追加も不要になります。

## 確認事項
1. **マクロの仕様**: ユーザ指定により、`own_memory!` には全てのWasm名を渡す必要があります。CLI側で全Wasmが指定されているかのバリデーションは、現段階のスコープ（`multi_memory_lowering.rs`）とは独立して後続で実装可能です。まずはメモリ拡張とダミー化に焦点を当てます。

### 3. VFS サンプルの修正 (`examples/own_memory_vfs/src/lib.rs`)
- マクロに「拡張する際は別のスレッドがメモリを書き換えていないことを保証する必要がある」旨のDocコメントを付与します。
- サンプルにおいて、ターゲット内の `dlmalloc` が失敗しないよう、スレッド生成前（安全なタイミング）に、マクロで生成した `memory_grow` を用いて十分なメモリを事前確保（Pre-allocate）する処理を実装します。

---

> [!IMPORTANT]
> ## User Review Required
> ターゲット内部の `memory.grow` を完全に無効化（常に `-1` を返す）し、代わりにロックを持たないエクスポート版の `memory_grow` をVFSから呼び出させる設計に変更しました。
> この「内部は失敗・インポート（VFSから見た呼び出し）は成功」という設計で、ご指摘の要件を完全に満たしていると考えますが、いかがでしょうか？
