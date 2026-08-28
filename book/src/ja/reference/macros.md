# コアマクロ

WASI Virtual Layer は、仮想化された実装を WASI ABI に「プラグイン」するためのいくつかのマクロを提供します。

## `import_wasm!`

仮想化レイヤーで使用する WebAssembly モジュールをインポートします。

```rust
import_wasm!(module_name);
```

## `plug_fs!`

仮想ファイルシステムの実装を 1 つ以上の Wasm モジュールに接続します。

### 使用法
```rust
plug_fs!(vfs_implementation, target_wasm, self);
```

- `vfs_implementation`: `Wasip1FileSystem` トレイトを実装するオブジェクト。
- `target_wasm`: (`import_wasm!` でインポートされた) Wasm モジュールの識別子。
- `self`: オプション。含めると、VFS 自体の中でのファイル操作も仮想化されます。

## `plug_env!`

仮想環境変数を接続します。

### 使用法
```rust
plug_env!(env_implementation, target_wasm, self);
```

## `plug_args!`

仮想コマンドライン引数を接続します。

## `plug_process!`

仮想プロセス管理（例：終了ハンドラ）を接続します。

## `plug_thread!`

仮想スレッドサポートを接続します（`threads` フィーチャーが必要）。

## `configure_wasm_stack!`

CLIビルドツールがモジュールのスタックサイズ、スロット数、およびリリース許可を構成できるようにするための、既知のグローバル変数をエクスポートします。

### 使用法
```rust
configure_wasm_stack!(
    size: 1048576,    // スタックサイズ（バイト単位、例: 1MiB）
    slots: 10,        // マルチメモリ環境用のスタックスロット数
    allow_release: true // スタックを動的に解放できるかどうか
);
```

## `protect_wasm_exports!`

特定のエクスポートされた関数がスタック分離ラッパーを受け取る必要があることを `wasi_virt_layer-cli` に通知するための静的マーカーをエクスポートします。

### 使用法
```rust
protect_wasm_exports!(
    "my_exported_function",
    "another_function"
);
```
