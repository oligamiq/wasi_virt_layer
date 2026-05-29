# スレッド VFS の例

この例では、マルチスレッド WASI モジュールで WASI Virtual Layer を使用する方法を示します。

## 特徴
- VFS とターゲットモジュール間の共有メモリ。
- `wasip1-threads` ABI のサポート。
- スレッドセーフな仮想ファイルシステムアクセス。

## 例の実行
```bash
cargo run -r -- -p threads_vfs test_threads -t single --threads true
```
