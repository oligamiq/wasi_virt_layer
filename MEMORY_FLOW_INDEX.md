# WASI Virtual Layer - メモリフロー図完全ガイド

このディレクトリには、WASI Virtual Layer（WVL）のスレッド機能付きメモリ配置フロー図に関する2つの詳細なドキュメントが含まれています。

## 📚 ドキュメント一覧

### 1. **memory_layout_flow_diagram.md**
   - **対象**: 汎用的なスレッド機能付きVFS+WasmA+WasmB メモリレイアウト
   - **内容**:
     - メモリ領域割当図
     - スレッド実行時系列図
     - メモリアクセスパス図
     - ロック機構図
     - Root vs Child Thread分岐図
     - メモリアクセスシーケンス例
   - **用途**: WASI Virtual Layer の全般的な理解、single_memoryモードの仕組み学習
   - **対象者**: アーキテクチャ設計者、貢献者、学習者

### 2. **minimal_repro_virtual_memory_flow.md**
   - **対象**: `examples/vfs/minimal_repro_virtual`プロジェクトの実行時メモリフロー
   - **内容**:
     - 初期メモリレイアウト（具体的なアドレス範囲）
     - 実行フェーズ1~3の詳細フロー
     - test_threads と ls の相互作用
     - StandardMultipleFileSystem ルーティング
     - スレッドスポーン時のメモリスナップショット
     - メモリ使用量の推定
     - トラブルシューティング
   - **用途**: 実装例の詳細理解、実行フロー追跡、デバッグ参考
   - **対象者**: 実装者、テスト担当者、デバッグ者

---

## 🎯 クイックスタート

### シナリオA: 「WASI Virtual Layer の仕組みを理解したい」
→ **`memory_layout_flow_diagram.md`** から開始

1. セクション1: メモリ領域割当図
2. セクション4: ロック機構図
3. セクション5: multi_memory との比較

### シナリオB: 「minimal_repro_virtual を実行して何が起きているか知りたい」
→ **`minimal_repro_virtual_memory_flow.md`** から開始

1. セクション1: 初期メモリレイアウト
2. セクション9: 完全な実行シーケンス
3. セクション12: 実行結果の予想出力

### シナリオC: 「スレッド機能の詳細を学びたい」
→ 両方のドキュメントを組み合わせて読む

1. `memory_layout_flow_diagram.md` セクション2 (スレッド実行時系列)
2. `memory_layout_flow_diagram.md` セクション4 (ロック機構)
3. `minimal_repro_virtual_memory_flow.md` セクション3 (test_threads 実行)
4. `minimal_repro_virtual_memory_flow.md` セクション11 (スレッド同期詳細)

---

## 📊 図の対応関係

| 図 | 所在 | 説明 |
|----|------|------|
| メモリ領域割当図 | `memory_layout_flow_diagram.md` §1 | 共有メモリの物理レイアウト |
| スレッド実行時系列図 | `memory_layout_flow_diagram.md` §2 | Root Spawn から Child の動作順序 |
| メモリアクセスパス図 | `memory_layout_flow_diagram.md` §3 | スレッドからメモリへのアクセス経路 |
| ロック機構図 | `memory_layout_flow_diagram.md` §4 | Atomic Compare-Exchange の詳細 |
| Root vs Child 分岐図 | `memory_layout_flow_diagram.md` §6 | isRootSpawn チェックとルーティング |
| メモリアクセスシーケンス例 | `memory_layout_flow_diagram.md` §7 | 複数スレッド同時アクセス |
| minimal_repro_virtual 初期レイアウト | `minimal_repro_virtual_memory_flow.md` §1 | VFS + test_threads + ls の配置 |
| VFS 初期化シーケンス | `minimal_repro_virtual_memory_flow.md` §2 | THREAD_POOL, VIRTUAL_FILE_SYSTEM 初期化 |
| test_threads 実行フロー | `minimal_repro_virtual_memory_flow.md` §3 | スレッドスポーンと同期の実例 |
| ls 実行フロー | `minimal_repro_virtual_memory_flow.md` §4 | ファイルシステム仮想化の実例 |
| メモリスナップショット | `minimal_repro_virtual_memory_flow.md` §6 | 実行中の各段階でのメモリ状態 |
| StandardMultipleFileSystem ルーティング | `minimal_repro_virtual_memory_flow.md` §8 | 3つのファイルシステムマウント |
| 完全な実行シーケンス | `minimal_repro_virtual_memory_flow.md` §9 | end-to-end の処理フロー |

---

## 🔑 主要コンセプト

### single_memory モード
- 全モジュール（VFS、WasmA、WasmB）が1つのメモリインスタンスを共有
- メモリアドレス空間が統一されている
- **詳細**: `memory_layout_flow_diagram.md` §5

### TargetMemoryMetadata
- 各ターゲットモジュールのメモリ領域情報
- base_ptr, limit_ptr, current_pages, max_pages を管理
- Shared Metadata Region に配置
- **詳細**: `minimal_repro_virtual_memory_flow.md` §1

### スレッド分岐（Root vs Child）
- **Root Spawn**: 初期スレッド、ホストレベルのスレッド生成を要求
- **Child Thread**: VFS ルーティングを経由してターゲットモジュールで実行
- **詳細**: `memory_layout_flow_diagram.md` §6、`minimal_repro_virtual_memory_flow.md` §11

### Atomic ロック
- Atomic Compare-Exchange Spin Lock で複数スレッド間のメモリアクセスを同期
- Locker(i) により per-target ロック
- **詳細**: `memory_layout_flow_diagram.md` §4

### MemoryCopyTo/From
- VFS ↔ Target モジュール間のメモリコピー
- MemoryDirector によるポインタルーティング
- **詳細**: `memory_layout_flow_diagram.md` §3、`minimal_repro_virtual_memory_flow.md` §5

### StandardMultipleFileSystem
- 複数のファイルシステムソースをマウント
- test_threads, ls (VFS mounted) + Host LFS
- **詳細**: `minimal_repro_virtual_memory_flow.md` §8

---

## 🛠️ 実装ファイルとの対応

```
wasi_virt_layer-cli/
  ├── src/generator/
  │   ├── threads.rs               ← Root vs Child 分岐実装
  │   ├── shared_global.rs         ← ロック機構実装
  │   └── memory.rs                ← メモリレイアウト管理
  └── src/abi.rs                   ← WASI ABI フック

wasi_virt_layer/
  ├── src/
  │   ├── shared_memory.rs         ← TargetMemoryMetadata
  │   ├── shared_global.rs         ← GlobalAltGet/Set 実装
  │   ├── thread.rs                ← VirtualThreadPool
  │   └── file/multiple.rs         ← StandardMultipleFileSystem

examples/
  └── vfs/
      └── minimal_repro_virtual/
          ├── src/lib.rs           ← VFS モジュール（この例）
          └── Cargo.toml
```

---

## 📖 読み方のコツ

### Mermaid 図の見方

1. **色分け**
   - 青系（#b3e5fc など）: VFS 関連
   - 紫系（#f3e5f5 など）: test_threads（WasmA）
   - 緑系（#c8e6c9 など）: ls（WasmB）
   - 黄系（#fff9c4 など）: 制御・メタデータ

2. **フロー図**
   - 矢印（→）: データ/制御の流れ
   - `|ラベル|`: 条件分岐
   - `par ... end`: 並列実行セクション

3. **シーケンス図**
   - 左から右: 参加者（モジュール）
   - 上から下: 時間の経過
   - `→`: 通常の呼び出し
   - `-->`: 応答/戻り値
   - `Note over`: コメント

---

## ⚠️ 既知の問題と対策

| 問題 | ドキュメント箇所 | 対策 |
|------|-----------------|------|
| single_memory での reset sequence 失敗 | `memory_layout_flow_diagram.md` §9 | multi_memory の使用推奨 |
| Component 変換時の threads feature 未サポート | `memory_layout_flow_diagram.md` §9 | TemporaryRefugeMemory で対応 |
| メモリ領域の重複 | `minimal_repro_virtual_memory_flow.md` §10 | アドレス計算の確認 |
| ロック競合でのデッドロック | `memory_layout_flow_diagram.md` §4 | Spin lock + timeout なし（注意） |

---

## 🎓 学習パス

### レベル1: 基礎理解（初心者向け）
1. `memory_layout_flow_diagram.md` §1 (メモリ領域割当)
2. `memory_layout_flow_diagram.md` §5 (multi_memory との比較)
3. `minimal_repro_virtual_memory_flow.md` §12 (実行結果)

### レベル2: 実装理解（開発者向け）
1. `minimal_repro_virtual_memory_flow.md` §1~4 (実行フェーズ)
2. `memory_layout_flow_diagram.md` §2 (時系列)
3. `memory_layout_flow_diagram.md` §4 (ロック機構)
4. `memory_layout_flow_diagram.md` §3 (メモリアクセス)

### レベル3: 深掘り理解（貢献者向け）
1. すべてのセクションを読破
2. 実装コード（threads.rs, shared_global.rs, memory.rs）と対応付け
3. トラブルシューティング（`minimal_repro_virtual_memory_flow.md` §14）で実装を追跡

---

## 🤝 貢献とフィードバック

これらのドキュメントは、WASI Virtual Layer の理解を深めるために作成されました。

**改善提案を歓迎します：**
- 図の明確さ向上
- 説明の簡潔化
- 新しいシナリオの追加
- バグ修正

---

## 📄 ドキュメント生成情報

| 属性 | 値 |
|------|------|
| 生成日時 | 2026-05-03 |
| memory_layout_flow_diagram.md | 425行 / 13.4 KB |
| minimal_repro_virtual_memory_flow.md | 510行 / 21.0 KB |
| 合計 Mermaid 図 | 14個 |
| 言語 | 日本語 |
| 形式 | Markdown + Mermaid |

---

## 📎 関連リソース

- **プロジェクト README**: `README.md`
- **IMPORTS/EXPORTS 追跡**: `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`
- **メモリレイアウト図**: `memory_layout_flow_diagram.pdf` (if available)
- **AGENTS 設定**: `AGENTS.md`

---

**最終更新**: 2026-05-03
**ドキュメント バージョン**: 1.0
