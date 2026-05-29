# wasi-virt-layer
WASI Virtual Layer は、WebAssembly System Interface (WASI) モジュールのための仮想ファイルシステムレイヤーです。カスタマイズや拡張が可能な仮想ファイルシステムを使用して、WASI モジュールを実行できます。

## 使用例
0. CLI ツールをインストールします：
```bash
cargo binstall wasi_virt_layer-cli
```
1. wasip1 向けにビルドされた WebAssembly モジュール（例：wasm32-wasip1 または wasm32-wasip1-threads）を用意します。
2. 新しい仮想ファイルシステム (VFS) プロジェクトを作成します：
```bash
wasi_virt_layer new my_vfs_project
```
3. VFS の実装を編集します。`import_wasm!` マクロを使用して、ターゲットとなる wasm モジュールの使用準備をします。
4. `plug!` マクロシリーズを使用して、wasip1 ABI に接続します。
5. ビルドコマンドを実行します：
```bash
wasi_virt_layer build <wasm_path>
```
6. ビルドされたファイルは `dist` ディレクトリに生成されます。
7. Deno または Node.js で実行します。
