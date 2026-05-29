# 共有メモリABI仕様ドラフト v1

## 1. 概要と目的

### 目的
`pseudo_import_wasm`におけるメモリ共有を実現し、VFSとターゲットWasmモジュール間のメモリコピーを完全に排除する。

### 前提
- ターゲットWasmモジュールが使用するメモリは、**全て** 共有メモリ内に存在する
- メモリコピーは一切発生しない（ゼロコピー）
- **ターゲット間でメモリを共有** - wasm-optのメモリ単層化と同様のアプローチ
- メモリ成長は **ターゲット側が要求** し、VFS側が自動的に確保
- **threadsフィーチャ有効時のみ提供**

---

## 2. アーキテクチャ概要

### 2.1 VFS側（Rust）の構成

```
Export関数（export_fs!, export_env!など）
  ↓
機能検出 → 共有メモリ機能を使用しているか判定
  ↓ （使用している場合）
Static確保 → VFS内の特定メモリ領域をstaticで確保
  ├─ parking_lot::RwLock<SharedMemoryManager>
  └─ 共有メモリ管理状態
  ↓
ABI関数の生成 → export_*マクロによるABIエクスポート
```

### 2.2 ターゲット側（ユーザーWASIP1）の処理フロー

```
元のWASIP1バイナリ
  ↓
新サブコマンド（wasi_virt_layer prepare-target）
  ├─ 全メモリアクセス命令を検出
  │  （load, store, load_align, store_align等）
  │
  ├─ wasm-optのメモリ単層化と同様のアプローチで
  │  メモリアクセスにフック処理を注入
  │
  ├─ memory.grow → ABI関数呼び出しに置換
  │
  ├─ メタデータ領域を線形メモリに確保
  │  (base_ptr, limit_ptr)
  │
  └─ 必要なABIインポートを追加
        ↓
変更済みWASIP1バイナリ
```

### 2.3 実行時のメモリ構成

```
VFS側（Rust）
├─ static SHARED_MEMORY: parking_lot::RwLock<SharedMemoryManager>
│  ├─ memory: 線形メモリ（全ターゲット共有）
│  └─ targets: Vec<TargetMetadata>
│     ├─ [0] TargetMetadata @ ptr_A
│     ├─ [1] TargetMetadata @ ptr_B
│     └─ [2] TargetMetadata @ ptr_C
│
└─ 公開ABI: wasip1_vfs_shared_memory_grow(metadata_ptr: u32, required_pages: u32) -> i32

共有線形メモリ（Shared Linear Memory）
├─ VFS内部用領域

├─ Target A メモリ領域
│  ├─ Code/Data/Heap/Stack
│  └─ 動的に成長可能
│
├─ Target B メモリ領域
│  ├─ Code/Data/Heap/Stack
│  └─ 動的に成長可能
│
└─ ... 他のターゲット

※各ターゲットは自分の TargetMetadata へのポインタのみをグローバル変数で保持
※Vecはメタデータ要素の配列
```

### 2.4 メモリアクセスの特殊処理

```
ターゲットWasmのメモリアクセス
  ↓
注入されたフック処理
  ├─ メタデータ読み込み（base_ptr, limit_ptr をメモリから取得）
  ├─ bounds check（オプション）
  ├─ shared memory同期（RwLock）
  └─ 実メモリアクセス
  ↓
共有メモリ内の実位置
```

---

## 3. ABI関数仕様（3つに削減）

### 3.1 `wasip1_vfs_register_shared_memory_target`

**呼び出し**: ターゲットが初期化時に呼び出す（1回のみ）

```
入力:
  - base_ptr: u32         // このターゲットのメモリ領域開始アドレス
  - current_pages: u32    // 初期ページ数
  - max_pages: u32        // 最大ページ数（0 = 無制限）

出力:
  - metadata_ptr: u32     // VFS が割り当てたメタデータへのポインタ
                          // 0 = 失敗

機能:
  - VFS側の targets Vec に新しい TargetMetadata を追加
  - そのメタデータへのポインタを返す
  - ターゲットはこのポインタをグローバル変数に保持
```

### 3.2 `wasip1_vfs_shared_memory_get_lock_ptr`

**呼び出し**: ターゲットが初期化時に呼び出す（1回のみ）

```
入力:
  - metadata_ptr: u32     // wasip1_vfs_register_shared_memory_target から返されたポインタ

出力:
  - lock_mgr_ptr: u32     // VFS側のロックマネージャーへのポインタ
                          // 0 = 失敗

機能:
  - ロック操作用のポインタを返す
  - ターゲットはこのポインタをグローバル変数に保持
  - ターゲット側で直接メモリアクセス（ABI呼び出し不要）
```

### 3.3 `wasip1_vfs_shared_memory_grow`

**呼び出し**: ターゲットが範囲外アクセス時に呼び出す

```
入力:
  - metadata_ptr: u32     // このターゲットの TargetMetadata へのポインタ
  - required_pages: u32   // 必要なページ数

出力:
  - success: i32          // 0=成功, -1=失敗

機能:
  - writeロックで保護（readロック保持下では呼び出されない前提）
  - 共有線形メモリを拡張
  - metadata_ptr が指す TargetMetadata 要素を更新
  - メモリ内容の移動・ポインタ値の更新
```

---

## 9. メモリアクセスと成長の仕組み

### 9.1 VFS側のメタデータ管理（一元化）

```rust
#[cfg(feature = "threads")]
pub struct SharedMemoryManager {
    /// 全ターゲットのメタデータ（配列）
    pub targets: Vec<TargetMetadata>,
    
    /// 実メモリ領域
    pub memory: &'static mut [u8],
}

pub struct TargetMetadata {
    /// メモリ領域開始アドレス
    pub base_ptr: u32,
    
    /// メモリ領域上限（成長に応じて更新）
    pub limit_ptr: u32,
    
    /// 現在のページ数
    pub current_pages: u32,
    
    /// ターゲット識別子（オプション）
    pub target_id: u32,
}
```

### 9.2 ターゲット側の初期化フロー

各ターゲットの初期化時に以下の処理を実行：

```wasm
; グローバル変数（初期値 0）
(global $metadata_ptr (mut i32) (i32.const 0))
(global $lock_mgr_ptr (mut i32) (i32.const 0))

; 初期化関数
(func $__init_shared_memory
  (local $result i32)
  
  ; VFS ABI 呼び出し1：ターゲット登録
  (local.set $result 
    (call $wasip1_vfs_register_shared_memory_target
      (i32.const <base_ptr>)      ; このターゲットのメモリ開始
      (i32.const <initial_pages>) ; 初期ページ数
      (i32.const 0)               ; max_pages（無制限）
    )
  )
  
  ; メタデータポインタをグローバル変数に保存
  (global.set $metadata_ptr (local.get $result))
  
  ; VFS ABI 呼び出し2：ロックマネージャーポインタ取得
  (local.set $result 
    (call $wasip1_vfs_shared_memory_get_lock_ptr
      (global.get $metadata_ptr)  ; 先ほど返されたメタデータポインタ
    )
  )
  
  ; ロックマネージャーポインタをグローバル変数に保存
  (global.set $lock_mgr_ptr (local.get $result))
)
```

### 9.3 初期化後のグローバル変数

```wasm
(global $metadata_ptr)  ← VFSから返されたメタデータへのポインタ
(global $lock_mgr_ptr)  ← VFSから返されたロックマネージャーへのポインタ
```

### 9.2 ターゲット側のグローバル変数（最小構成）

各ターゲットに注入されるグローバル変数（**ポインタのみ**）：

```wasm
; 自分の TargetMetadata へのポインタ
(global $metadata_ptr (mut i32) (i32.const 0))

; VFS側のロックマネージャーへのポインタ
(global $lock_mgr_ptr (mut i32) (i32.const 0))

; 初期化時に設定
(func $__init_shared_memory_globals
  (local $result i32)
  
  ; 登録
  (local.set $result 
    (call $wasip1_vfs_register_shared_memory_target (i32.const <base_ptr>) ...)
  )
  (global.set $metadata_ptr (local.get $result))
  
  ; ロック取得
  (local.set $result 
    (call $wasip1_vfs_shared_memory_get_lock_ptr (global.get $metadata_ptr))
  )
  (global.set $lock_mgr_ptr (local.get $result))
)
```

### 9.3 初期化フロー（全体図）

```
ターゲットWasm 起動
  ↓
_start() / _main() 呼び出し
  ↓
__init_shared_memory() 実行
  ├─ VFS ABI呼び出し1：wasip1_vfs_register_shared_memory_target(...)
  │  └─ VFS側：targets Vec に新規要素追加 → メタデータポインタ返却
  ├─ メタデータポインタをグローバル変数に保存
  │
  ├─ VFS ABI呼び出し2：wasip1_vfs_shared_memory_get_lock_ptr(...)
  │  └─ VFS側：ロックマネージャーへのポインタ返却
  ├─ ロックマネージャーポインタをグローバル変数に保存
  └─ 処理完了
  ↓
メインのターゲットコード実行
  ↓
メモリアクセス時：
  ├─ readロック取得（直接メモリ操作、ABI呼び出し不要）
  ├─ グローバル変数から metadata_ptr を取得
  ├─ メタデータ読み込み → base_ptr / limit_ptr 取得
  ├─ bounds check
  ├─ 範囲内の場合：メモリアクセス実行
  ├─ 範囲外の場合：
  │  ├─ readロック解放（直接メモリ操作）
  │  ├─ wasip1_vfs_shared_memory_grow() 呼び出し（ABI関数）
  │  ├─ readロック再度取得（直接メモリ操作）
  │  └─ メタデータ再度読み込み → アクセス実行
  └─ readロック解放（直接メモリ操作）
```

### 9.4 TargetMetadata のメモリレイアウト

```
各TargetMetadata要素（16 bytes）

Offset  Size  名前           型
------  ----  -----------   ----
0       4     base_ptr      i32
4       4     limit_ptr     i32
8       4     current_pages i32
12      4     target_id     i32
```

### 9.4 TargetMetadata のメモリレイアウト

```
各TargetMetadata要素（16 bytes）

Offset  Size  名前           型
------  ----  -----------   ----
0       4     base_ptr      i32
4       4     limit_ptr     i32
8       4     current_pages i32
12      4     max_pages     i32
```

### 9.5 メモリアクセスのフロー（直接ロック操作版）

```
ターゲットWasmのメモリアクセス（offset値）
  ↓
1. readロック取得（直接メモリ操作）
   lock_mgr_ptr = global.get $lock_mgr_ptr
   状態をロック領域のメモリに記録して readロック取得
   
2. グローバル変数から自分のメタデータへのポインタを取得
   metadata_ptr = global.get $metadata_ptr
  
3. メタデータから値を読み取り
   base_ptr = load_i32(metadata_ptr + 0)
   limit_ptr = load_i32(metadata_ptr + 4)
  
4. bounds check
   actual_addr = offset + base_ptr
   if (actual_addr < limit_ptr) {
     // メモリアクセス実行
     result = メモリ操作
   } else {
     // readロック一度解放（直接メモリ操作）
     
     // 成長処理が必要
     call wasip1_vfs_shared_memory_grow(metadata_ptr, required_pages)
     
     // readロック再度取得（直接メモリ操作）
     
     // メタデータ再度読み込み
     base_ptr = load_i32(metadata_ptr + 0)
     limit_ptr = load_i32(metadata_ptr + 4)
     
     // アクセス実行
     result = メモリ操作
   }
   
5. readロック解放（直接メモリ操作）
```

### 9.5 メモリ成長時のフロー

```
ターゲット A がメモリ範囲外アクセス
  ↓
1. ターゲット: ABI呼び出し
   wasip1_vfs_shared_memory_grow(metadata_ptr, required_pages)
  ↓
2. VFS側: writeロック取得
  ├─ メモリ拡張（線形メモリ増加）
  ├─ metadata_ptr が指す TargetMetadata を更新
  │  ├─ limit_ptr = 新しい上限値
  │  ├─ current_pages = 新しいページ数
  │  └─ base_ptr も必要に応じて更新
  ├─ メモリ内容移動（既存ターゲットのデータ）
  └─ writeロック解放
  ↓
3. ターゲット: ABI関数から戻る
  ↓
4. ターゲット: メタデータを再度読み込み
   limit_ptr = load_i32(metadata_ptr + 4)  // 新しい値
  ↓
5. アクセス実行
```

### 9.6 VFS側の実装

```rust
#[export_name = "wasip1_vfs_register_shared_memory_target"]
extern "C" fn register_shared_memory_target(
    base_ptr: u32,
    current_pages: u32,
    max_pages: u32,
) -> u32 {
    #[cfg(not(feature = "threads"))]
    return 0;  // 失敗
    
    #[cfg(feature = "threads")]
    {
        let mut mgr = SHARED_MEMORY.write();
        
        // TargetMetadata を Vec に追加
        let metadata = TargetMetadata {
            base_ptr,
            limit_ptr: base_ptr + (current_pages * 65536),
            current_pages,
            max_pages,  // 0 = 無制限
        };
        
        mgr.targets.push(metadata);
        
        // 最後に追加した要素へのポインタを返す
        let ptr = &mgr.targets[mgr.targets.len() - 1] as *const TargetMetadata;
        ptr as u32
    }
}

#[export_name = "wasip1_vfs_shared_memory_get_lock_ptr"]
extern "C" fn get_lock_ptr(metadata_ptr: u32) -> u32 {
    #[cfg(not(feature = "threads"))]
    return 0;  // 失敗
    
    #[cfg(feature = "threads")]
    {
        // ロックマネージャーへのポインタを返す
        // 実装例：SHARED_MEMORY オブジェクトへのポインタ
        let mgr_ptr = &SHARED_MEMORY as *const parking_lot::RwLock<SharedMemoryManager>;
        mgr_ptr as u32
    }
}

#[export_name = "wasip1_vfs_shared_memory_grow"]
extern "C" fn grow_shared_memory(
    metadata_ptr: u32,
    required_pages: u32,
) -> i32 {
    #[cfg(not(feature = "threads"))]
    return -1;
    
    #[cfg(feature = "threads")]
    {
        let mut mgr = SHARED_MEMORY.write();
        
        // ポインタから TargetMetadata を特定
        let target_ref = unsafe {
            &mut *(metadata_ptr as *mut TargetMetadata)
        };
        
        // メモリ拡張
        let current_pages = mgr.memory.len() / 65536;
        let new_pages = current_pages + required_pages as usize;
        let new_size = new_pages * 65536;
        
        if mgr.memory.len() < new_size {
            let mut new_memory = vec![0u8; new_size];
            new_memory[..mgr.memory.len()].copy_from_slice(mgr.memory);
            mgr.memory = new_memory.into_boxed_slice();
        }
        
        // メタデータ更新
        target_ref.limit_ptr = (new_pages as u32) * 65536;
        target_ref.current_pages = new_pages as u32;
        
        0  // 成功
    }
}
```

---

## 10. 実装の流れ（計画）

### 10.1 Phase 1: 仕様確定
- [ ] このドラフトをユーザと共に精査
- [ ] メモリレイアウト戦略を明確化
- [ ] 仕様をロック

### 10.2 Phase 2: VFS側の実装
- [ ] export_*マクロに機能検出ロジック追加
- [ ] static確保とABI生成を自動化
- [ ] SharedMemoryManager実装

### 10.3 Phase 3: ターゲット側の実装
- [ ] `prepare-target`サブコマンド作成
- [ ] Wasmバイナリ変換ロジック
  - メモリアクセス命令のフック
  - memory.grow置換
  - インポート追加

### 10.4 Phase 4: 統合とテスト
- [ ] VFS側とターゲット側の連携テスト
- [ ] マルチターゲット構成でのテスト
- [ ] スレッドセーフティテスト
- [ ] パフォーマンステスト

---

## 11. 変更点まとめ（v0 → v1）

| 項目 | v0 | v1 |
|------|-----|-----|
| **ABI関数数** | 3個 | 1個（grow のみ） |
| **メモリレイアウト決定** | VFS生成時 | 未定（TBD） |
| **ターゲット間メモリ** | 分離（オプション） | **共有必須** |
| **メモリアクセス** | 直接 | wasm-optと同様に処理 |
| **メモリ成長権限** | VFS側 | **ターゲット側** |
| **Thread Safety** | 未定 | **parking_lot Mutex** |
| **サブコマンド** | なし | **prepare-target** 新規 |
| **Feature** | 未定 | **threads のみ** |
| **Static確保** | 未定 | export_*マクロ or ユーザ |

---

## 12. 実装の流れ（計画）

### 12.1 Phase 1: 仕様確定
- [ ] このドラフトをユーザと共に精査
- [ ] parking_lot RwLockの詳細設計
- [ ] メモリ移動・ポインタ更新処理の仕様
- [ ] 仕様をロック

### 12.2 Phase 2: VFS側の実装
- [ ] export_*マクロに機能検出ロジック追加
- [ ] static確保（parking_lot RwLock）
- [ ] SharedMemoryManager実装
  - readロック取得
  - writeロックによる成長処理
  - 全ターゲットのメタデータ更新

### 12.3 Phase 3: ターゲット側の実装
- [ ] `prepare-target`サブコマンド作成
- [ ] Wasmバイナリ変換ロジック
  - メモリアクセス命令のフック生成
  - 各アクセスにbounds check と ロック取得を注入
  - 範囲外アクセス時の成長処理呼び出し
  - グローバル変数（base_ptr, limit_ptr）の注入

### 12.4 Phase 4: 統合とテスト
- [ ] VFS側とターゲット側の連携テスト
- [ ] マルチターゲット構成でのテスト
- [ ] スレッドセーフティテスト
- [ ] パフォーマンステスト
- [ ] メモリ移動シナリオのテスト

---

## 13. 変更点まとめ（v1 → v2）

| 項目 | v1 | v2 |
|------|-----|-----|
| **呼び出し方式** | 直接呼び出し | **VFS側フック内から自動** |
| **ターゲット識別** | Target ID使用 | **ポインタレンジで判定** |
| **ロック機構** | Mutex | **parking_lot RwLock** |
| **メモリアクセス** | 直接 | **readロック + bounds check** |
| **メモリ成長時** | 単純拡張 | **全ターゲット更新 + 移動** |
| **グローバル変数** | 未定 | **base_ptr, limit_ptr** |
| **Thread Safety** | Mutex | **RwLock（read優先）** |

---

## 14. 重要な実装ポイント

### 14.1 メタデータ一元管理の原則

- **VFS側が全ターゲットのメタデータを Vec で管理**
- **ターゲット側は自分の TargetMetadata へのポインタのみをグローバル変数で保持**
- **成長時は ABI経由で VFS に依頼 → VFSがメモリ + メタデータ更新**

### 14.2 ABI関数の署名

```wasm
(import "env" "wasip1_vfs_shared_memory_grow"
  (func $grow_memory (param i32 i32) (result i32))
  ; param 0: metadata_ptr (このターゲットの TargetMetadata へのポインタ)
  ; param 1: required_pages (必要なページ数)
  ; return: 0=成功, -1=失敗
)
```

### 14.3 ターゲット側のメモリアクセス実装

```wasm
; メタデータ読み込み例
(local $metadata_ptr i32)
(local $base_ptr i32)
(local $limit_ptr i32)

; グローバル変数から自分のメタデータへのポインタを取得
(local.set $metadata_ptr (global.get $metadata_ptr))

; base_ptr = *(metadata_ptr + 0)
(local.set $base_ptr
  (i32.load (local.get $metadata_ptr))
)

; limit_ptr = *(metadata_ptr + 4)
(local.set $limit_ptr
  (i32.load (i32.add (local.get $metadata_ptr) (i32.const 4)))
)

; 以降 base_ptr / limit_ptr を使用
```

### 14.4 メモリ拡張時の呼び出し

```wasm
; 範囲外アクセス時
(if (i32.ge_u (i32.add (local.get $offset) (local.get $base_ptr))
               (local.get $limit_ptr))
  (then
    ; VFS ABI呼び出し
    (call $grow_memory
      (global.get $metadata_ptr)
      (i32.const <required_pages>)
    )
    
    ; 戻ってきたら limit_ptr を再度読み込み
    ; （メタデータは VFS側で更新済み）
    (local.set $limit_ptr
      (i32.load (i32.add (global.get $metadata_ptr) (i32.const 4)))
    )
    
    ; アクセス実行
  )
)
```

### 14.5 VFS側の実装詳細

```rust
#[export_name = "wasip1_vfs_shared_memory_grow"]
extern "C" fn grow_shared_memory(
    metadata_ptr: u32,
    required_pages: u32,
) -> i32 {
    #[cfg(not(feature = "threads"))]
    return -1;
    
    #[cfg(feature = "threads")]
    {
        // RwLock writeロック
        let mut mgr = SHARED_MEMORY.write();
        
        // ポインタから TargetMetadata を特定
        let target_ref = unsafe {
            &mut *(metadata_ptr as *mut TargetMetadata)
        };
        
        // メモリ拡張
        let current_pages = mgr.memory.len() / 65536;
        let new_pages = current_pages + required_pages as usize;
        let new_size = new_pages * 65536;
        
        if mgr.memory.len() < new_size {
            let mut new_memory = vec![0u8; new_size];
            new_memory[..mgr.memory.len()].copy_from_slice(mgr.memory);
            mgr.memory = new_memory.into_boxed_slice();
        }
        
        // メタデータ更新
        target_ref.limit_ptr = (new_pages as u32) * 65536;
        target_ref.current_pages = new_pages as u32;
        
        0  // 成功
    }
}
```

### 14.6 複数VFSインスタンスの独立性

```
VFS A: SHARED_MEMORY @ Rust側
       ├─ memory: 1GB
       ├─ targets: Vec<TargetMetadata>
       │  ├─ &targets[0] → metadata_ptr_A → ターゲット A1 が保持
       │  └─ &targets[1] → metadata_ptr_A → ターゲット A2 が保持
       
VFS B: SHARED_MEMORY @ Rust側
       ├─ memory: 2GB
       ├─ targets: Vec<TargetMetadata>
       │  ├─ &targets[0] → metadata_ptr_B → ターゲット B1 が保持
       │  └─ &targets[1] → metadata_ptr_B → ターゲット B2 が保持
```

各ターゲットは自分のメタデータ要素へのポインタを保持。

---

---

## 15. ロック機構の詳細設計

### 15.1 ロック操作の手法（直接メモリアクセス）

ABI呼び出しのオーバーヘッドを削減するため、ロック取得・解放は **直接メモリ操作** で行う：

```
VFS側が公開：
  - lock_mgr_ptr（parking_lot::RwLock<SharedMemoryManager>へのポインタ）
  - メモリ上のロック状態領域

ターゲット側が実行：
  - lock_mgr_ptr をグローバル変数で保持
  - メモリアクセス前：lock_mgr_ptr の指す領域でreadロック操作
  - メモリアクセス後：lock_mgr_ptr の指す領域でreadロック解放
```

### 15.2 VFS側のロックマネージャー構造

```rust
#[cfg(feature = "threads")]
static SHARED_MEMORY: parking_lot::RwLock<SharedMemoryManager> = 
    parking_lot::const_rwlock(SharedMemoryManager::new());

// ターゲット側に返すポインタ
let lock_mgr_ptr = &SHARED_MEMORY as *const parking_lot::RwLock<SharedMemoryManager>;
```

### 15.3 ターゲット側のロック操作（Wasm疑似コード）

```wasm
; グローバル変数
(global $lock_mgr_ptr (mut i32))

; readロック取得（メモリアクセス前）
(func $acquire_read_lock
  (local $lock_mgr_ptr i32)
  (local.set $lock_mgr_ptr (global.get $lock_mgr_ptr))
  
  ; parking_lot::RwLock のメモリ領域にアクセス
  ; ロック状態をメモリに記録して取得
  ; ※ 実装上の詳細は VFS側の RwLock 構造次第
)

; readロック解放（メモリアクセス後）
(func $release_read_lock
  (local $lock_mgr_ptr i32)
  (local.set $lock_mgr_ptr (global.get $lock_mgr_ptr))
  
  ; ロック状態をメモリから削除して解放
)
```

### 15.4 利点

- **ABI呼び出し削減**：毎メモリアクセスでのABI呼び出しが不要
- **性能向上**：ロック操作が直接メモリアクセスで完結
- **初期化2回のみ**：wasip1_vfs_register_shared_memory_target + wasip1_vfs_shared_memory_get_lock_ptr

---

**このドラフト v2.7（ABI 3関数に削減版）でよろしいでしょうか？**
