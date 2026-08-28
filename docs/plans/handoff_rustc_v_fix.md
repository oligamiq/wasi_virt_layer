# 引継ぎ資料: `rustc -v` が WebShell 上で動作しない問題

## 1. 問題の概要

WebShell 上で `rustc -v` を実行すると、バージョン情報が表示されず panic で終了する。

```
/ $ rustc -v
Executing command: ["rustc", "-v"]
VFS: Survived rustc_opt::_start!
rustc_opt: before panic
```

## 2. 根本原因（確定済み）

**`bun run vfs:build` の実行時に、本物の rustc コンパイラ WASM バイナリ（95MB）がモッククレートのビルド出力（64KB）で上書きされている。**

### 詳細

- `crates/vfs/rustc_opt.wasm` — 元々は 95MB の本物の rustc コンパイラ（Git コミット `c2bb521` で追加）
- `crates/rustc_opt/src/main.rs` — テスト用モッククレート。中身は `panic!("This is a test panic in rustc_opt")` のみ
- ビルドコマンド `wasi_virt_layer build -p vfs crates/vfs/rustc_opt.wasm ... vfs-shell --dev` が実行されると、`wasi_virt_layer` CLI ツールがワークスペースメンバー `crates/rustc_opt` を発見し、それをビルドして `crates/vfs/rustc_opt.wasm` を上書きしてしまう
- 結果、合成 WASM バイナリ (`vfs.core.wasm`) にモッククレートが埋め込まれ、`rustc -v` が panic する

### 証拠

```bash
# Git上の本来のファイル: 95MB
git show c2bb521 --stat
# crates/vfs/rustc_opt.wasm | Bin 0 -> 95427808 bytes

# ビルド後のファイル: 64KB（モッククレートで上書き済み）
ls -la crates/vfs/rustc_opt.wasm
# 64716 bytes

# 差分
git diff --stat HEAD -- crates/vfs/rustc_opt.wasm
# crates/vfs/rustc_opt.wasm | Bin 95427808 -> 64716 bytes
```

### 一時的な修正（実施済み）

```bash
git checkout HEAD -- crates/vfs/rustc_opt.wasm
```

これにより 95MB の本物の rustc バイナリが復元されたが、次回 `bun run vfs:build` 実行時に再び上書きされる。

## 3. 修正が必要な箇所

### 必須: ビルド時の上書き防止

`package.json` 内のビルドコマンド:

```json
"vfs:build": "wasi_virt_layer build -p vfs crates/vfs/llvm_opt.wasm crates/vfs/rustc_opt.wasm crates/vfs/lsp_opt.wasm vfs-shell --dev && ..."
```

**問題**: `crates/vfs/rustc_opt.wasm` がファイルパスとして渡されているが、`wasi_virt_layer` CLI がワークスペース内の同名クレート `crates/rustc_opt` を優先してビルド→上書きしている可能性がある。

**修正案**（いずれか）:

| 案 | 内容 | 影響範囲 |
|---|---|---|
| A | `crates/rustc_opt` クレートのパッケージ名を `rustc_opt_mock` 等に変更 | `Cargo.toml` のみ |
| B | `crates/vfs/rustc_opt.wasm` を別ディレクトリ（例: `assets/rustc_opt.wasm`）に移動し、ビルドコマンドのパスを更新 | `package.json`, ディレクトリ構造 |
| C | `wasi_virt_layer build` の引数指定を修正（`crates/vfs/rustc_opt.wasm` がクレート名ではなくファイルパスとして確実に解釈されるようにする） | `package.json` |
| D | ビルドスクリプトで、ビルド前に `.wasm` ファイルをバックアップし、ビルド後に復元する | `package.json` |

### 任意: デバッグ用 println の削除

`crates/vfs/src/command.rs` の L134, L136 にデバッグ用出力が残っている:

```rust
println!("VFS: Survived rustc_opt::_start!");
// ...
println!("VFS: Survived rustc_opt::_main!");
```

## 4. 関連ファイル一覧

| ファイル | 役割 |
|---|---|
| `package.json` | ビルドコマンド `vfs:build` の定義 |
| `crates/vfs/rustc_opt.wasm` | 本物の rustc コンパイラ WASM バイナリ（95MB） |
| `crates/rustc_opt/src/main.rs` | モッククレート（panic するだけ） |
| `crates/rustc_opt/Cargo.toml` | モッククレートの設定（パッケージ名 `rustc_opt`） |
| `crates/vfs/src/command.rs` | `rustc` コマンドのディスパッチ（L130-L137） |
| `crates/vfs/src/lib.rs` | VFS コンポーネント本体、`import_wasm!(rustc_opt)` (L330) |
| `crates/vfs/Cargo.toml` | VFS クレートの依存関係 |
| `Cargo.toml` | ワークスペース定義（`members = ["crates/*"]`） |

## 5. 同様の問題が起きうる他のファイル

同じパターンで、以下のファイルも上書きされるリスクがある:

| WASM バイナリ | 対応するモッククレート | 現在のサイズ |
|---|---|---|
| `crates/vfs/llvm_opt.wasm` | `crates/llvm_opt/` (存在すれば) | 81MB |
| `crates/vfs/lsp_opt.wasm` | `crates/lsp_opt/` (存在すれば) | 33MB |
| `crates/vfs/rustc_mock.wasm` | `crates/rustc_mock/` | 104KB |
| `crates/vfs/llvm_mock.wasm` | `crates/llvm_mock/` | 2.9MB |

## 6. プロジェクトのルール（GEMINI.md より）

修正時に注意すべき制約:

- **パッケージマネージャ**: `bun`（`cargo` は Rust 用）
- **`inst.ts` の変更**: `page/src/worker_process/vfs_bindings/inst.ts` を変更する場合、コミットメッセージは `wip` にする
- `bun run vfs:truebuild` は実行禁止
- Atomics / SharedArrayBuffer は使用禁止
- WIT での `list` 型は使用禁止

## 7. アーキテクチャ概要

```
ユーザー入力 → xterm.tsx → SharedObject → util_cmd.ts (Worker)
  → VFS Component (WASM) → vfs-shell (WASM)
  → command.rs::handle_command() → rustc_opt::_reset() / _start() / _main()
  → 本物の rustc WASM が実行される（はず）
```

`wasi_virt_layer build` は以下のモジュールを1つの `vfs.core.wasm` に合成する:
- `vfs` クレート（プライマリ）
- `vfs-shell` クレート
- `rustc_opt.wasm`（外部バイナリ）
- `llvm_opt.wasm`（外部バイナリ）
- `lsp_opt.wasm`（外部バイナリ）

## 8. 再現手順

```bash
# 1. 本物のバイナリを復元
git checkout HEAD -- crates/vfs/rustc_opt.wasm
ls -lh crates/vfs/rustc_opt.wasm  # → 92MB

# 2. ビルド実行（上書きが発生する）
bun run vfs:build
ls -lh crates/vfs/rustc_opt.wasm  # → 64KB に縮小 = モックで上書き

# 3. WebShell で確認
bun run --cwd page dev
# ブラウザで rustc -v → "rustc_opt: before panic" が表示される
```
