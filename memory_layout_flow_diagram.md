# VFS + WasmA + WasmB メモリ配置フロー図（スレッド機能付き）

このドキュメントは、WASI Virtual Layer (WVL) プロジェクトにおいて、VFS（Virtual File System）とターゲットWasmモジュール（WasmA、WasmB）を`single_memory`モードで統合し、複数スレッド実行時のメモリレイアウトと制御フロー図を示しています。

---

## 1. メモリ領域割当図（Memory Allocation Diagram）

このセクションは、`single_memory`モード下での共有メモリの物理的レイアウトを示します。複数のモジュールが1つのメモリインスタンスを共有し、各モジュールのデータはアドレス空間内の異なる領域に配置されます。

```mermaid
graph TB
    subgraph "Shared Linear Memory (single_memory)"
        direction TB
        
        subgraph "VFS Region"
            VFS_HEAP["VFS Heap<br/>(VFS Data Structures)"]
            VFS_STACK["VFS Stack"]
            VFS_GLOBALS["VFS Globals<br/>(FileSystem, Env)"]
        end
        
        subgraph "Shared Metadata Region"
            METADATA["TargetMemoryMetadata[]<br/>(WasmA, WasmB Metadata)<br/>base_ptr, limit_ptr,<br/>current_pages, max_pages"]
            ATOMIC_GLOBALS["Atomic Global Variables<br/>(Thread-safe Shared State)"]
            LOCK_STORAGE["Lock Storage<br/>(Parking_lot RwLock)"]
        end
        
        subgraph "WasmA Region"
            WASMA_HEAP["WasmA Heap<br/>(Module A Data)"]
            WASMA_STACK["WasmA Stack"]
            WASMA_GLOBALS["WasmA Globals"]
        end
        
        subgraph "WasmB Region"
            WASMB_HEAP["WasmB Heap<br/>(Module B Data)"]
            WASMB_STACK["WasmB Stack"]
            WASMB_GLOBALS["WasmB Globals"]
        end
    end
    
    VFS_HEAP --> METADATA
    METADATA --> WASMA_HEAP
    WASMA_GLOBALS --> ATOMIC_GLOBALS
    ATOMIC_GLOBALS --> LOCK_STORAGE
    LOCK_STORAGE --> WASMB_HEAP
    WASMB_GLOBALS --> VFS_GLOBALS
    
    style VFS_HEAP fill:#e1f5ff
    style WASMA_HEAP fill:#f3e5f5
    style WASMB_HEAP fill:#e8f5e9
    style METADATA fill:#fff3e0
    style ATOMIC_GLOBALS fill:#ffe0b2
    style LOCK_STORAGE fill:#ffccbc
```

### メモリ領域の説明

| 領域 | 用途 | アクセス権 | スレッド安全性 |
|------|------|--------|------------|
| **VFS Region** | VFSのヒープ、スタック、グローバル変数 | VFS主導 | 内部ロック管理 |
| **Shared Metadata** | 各モジュールのメモリ管理情報 | 全モジュール | ターゲット登録時のみ |
| **WasmA/B Region** | 各ターゲットモジュールのデータ | 各モジュール主導 | per-module操作 |
| **Atomic Globals** | スレッド間共有グローバル変数 | 全スレッド | Atomic操作 + Locker |
| **Lock Storage** | スレッド同期用ロック | 全スレッド | RwLock (parking_lot) |

---

## 2. スレッド実行時系列図（Thread Execution Timeline）

このセクションは、Root SpawnとChild Threadの実行順序とメモリアクセスパターンを示します。

```mermaid
sequenceDiagram
    participant Host as Host/Runtime
    participant VFS as VFS Module
    participant RootSpawn as Root Spawn<br/>Thread
    participant ChildT1 as Child Thread 1
    participant ChildT2 as Child Thread 2
    participant SharedMem as Shared Memory

    Host->>VFS: 1. Initialize VFS Module
    VFS->>SharedMem: Allocate Metadata Region + Lock Storage
    
    Host->>RootSpawn: 2. Call _start()
    RootSpawn->>VFS: 3. Initialize WasmA, WasmB Metadata
    VFS->>SharedMem: Register WasmA base_ptr, limit_ptr
    VFS->>SharedMem: Register WasmB base_ptr, limit_ptr
    
    RootSpawn->>RootSpawn: 4. Execute Application Logic (Single Thread)
    
    RootSpawn->>RootSpawn: 5. Decision: isRootSpawn() → true
    RootSpawn->>Host: 6. Call __wasip1_vfs_real_thread_spawn_fn(arg)
    
    par Thread Spawning
        Host->>ChildT1: 7a. Allocate Thread 1 Stack
        ChildT1->>VFS: Access __self_wasi_thread_start (VFS routing)
        
        Host->>ChildT2: 7b. Allocate Thread 2 Stack
        ChildT2->>VFS: Access __self_wasi_thread_start (VFS routing)
    end
    
    par Parallel Execution
        ChildT1->>SharedMem: 8a. Lock (Atomic::compare_exchange)
        ChildT1->>ChildT1: Execute Application Code
        ChildT1->>SharedMem: 9a. Access Shared Global (GlobalAltGet)
        ChildT1->>SharedMem: 10a. Unlock
        
        ChildT2->>SharedMem: 8b. Lock (Atomic::compare_exchange)
        ChildT2->>ChildT2: Execute Application Code
        ChildT2->>SharedMem: 9b. Access Shared Global (GlobalAltGet)
        ChildT2->>SharedMem: 10b. Unlock
    end
    
    ChildT1->>RootSpawn: 11a. Thread Join
    ChildT2->>RootSpawn: 11b. Thread Join
    
    RootSpawn->>Host: 12. Return from Application
```

### 実行フロー解説

1. **初期化フェーズ (Steps 1-3)**
   - VFSモジュールが初期化され、共有メモリ領域とロック機構が確立される
   - WasmA、WasmB の メタデータが登録される

2. **Root Spawn フェーズ (Steps 4-6)**
   - Root Spawnスレッド（初期スレッド）が`_start()`から開始
   - `isRootSpawn()`チェックで真（true）を返す
   - ホストレベルのスレッド生成関数`__wasip1_vfs_real_thread_spawn_fn`を呼び出し

3. **子スレッド生成フェーズ (Step 7)**
   - ホストが新しいスレッドのスタック領域を割り当て
   - 各子スレッドは VFS の `__self_wasi_thread_start` ルーティング関数を経由

4. **並列実行フェーズ (Steps 8-10)**
   - 複数スレッドが共有メモリへ同時アクセス
   - **Lock**: Atomic操作でロック獲得
   - **共有グローバルアクセス**: `GlobalAltGet`で読み取り
   - **Unlock**: ロック解放

5. **同期フェーズ (Steps 11-12)**
   - 子スレッドの終了をRoot Spawnが待機
   - すべてのスレッドが終了後、ホストへ制御を返却

---

## 3. メモリアクセスパス図（Memory Access Path Diagram）

このセクションは、スレッドから共有メモリ内のデータにアクセスするまでの経路を示します。

```mermaid
graph LR
    subgraph "スレッド層"
        T1["Thread 1"]
        T2["Thread 2"]
        TN["Thread N"]
    end
    
    subgraph "メモリアクセス制御層"
        LOCK["Locker(i)<br/>Atomic Lock Manager"]
        ROUTING["Memory Router<br/>(MemoryDirector)"]
    end
    
    subgraph "メモリ操作層"
        COPY_TO["MemoryCopyTo<br/>(Target→VFS)"]
        COPY_FROM["MemoryCopyFrom<br/>(VFS→Target)"]
        GROW["memory.grow (Atomic)"]
    end
    
    subgraph "共有メモリ"
        VFS_DATA["VFS Data<br/>(Metadata, Globals)"]
        TARGET_DATA["Target Data<br/>(WasmA, WasmB)"]
    end
    
    T1 -->|Read/Write Request| LOCK
    T2 -->|Read/Write Request| LOCK
    TN -->|Read/Write Request| LOCK
    
    LOCK -->|Check isRootSpawn| ROUTING
    
    ROUTING -->|Target→VFS| COPY_FROM
    ROUTING -->|VFS→Target| COPY_TO
    ROUTING -->|Memory Expand| GROW
    
    COPY_TO --> VFS_DATA
    COPY_FROM --> TARGET_DATA
    GROW --> TARGET_DATA
    
    VFS_DATA -->|Metadata Lookup| ROUTING
    TARGET_DATA -->|Pointer Arithmetic| ROUTING
    
    style T1 fill:#f3e5f5
    style T2 fill:#e8f5e9
    style TN fill:#fff3e0
    style LOCK fill:#ffccbc
    style ROUTING fill:#ffe0b2
    style COPY_TO fill:#b3e5fc
    style COPY_FROM fill:#c8e6c9
    style GROW fill:#ffe082
    style VFS_DATA fill:#f0f4c3
    style TARGET_DATA fill:#d7ccc8
```

### アクセスパスの処理フロー

```
Thread Request
    ↓
Locker(i) - スレッド間のロック競合を解決
    ↓
isRootSpawn() チェック
    ├─ true → ホストレベルのスレッド操作
    └─ false → VFS経由のルーティング
    ↓
Memory Router (MemoryDirector)
    ├─ Read from Target → MemoryCopyFrom
    ├─ Write to VFS → MemoryCopyTo
    └─ Expand Memory → Atomic memory.grow
    ↓
Shared Memory Access
```

---

## 4. ロック機構図（Locking Mechanism Diagram）

このセクションは、`single_memory`モード下でのスレッド同期とロック管理を示します。

```mermaid
graph TB
    subgraph "Thread Synchronization Model"
        direction TB
        
        subgraph "Atomic Global Variables"
            ATOMIC_VAR["Atomic<i32><br/>(Compare-Exchange Spin Lock)"]
        end
        
        subgraph "SharedGlobal Function Variants"
            GLOBAL_ALT_GET["GlobalAltGet<br/>(Thread-safe Read)"]
            GLOBAL_ALT_SET["GlobalAltSet<br/>(Thread-safe Write)"]
            GLOBAL_ALT_INIT["GlobalAltInitOnce<br/>(One-time Init)"]
            GLOBAL_ALT_GET_NOWAIT["GlobalAltGetNoWait<br/>(Fast Read, No Lock)"]
        end
        
        subgraph "Lock Variants"
            LOCKER_BASE["LockerBase<br/>(Primary Lock)"]
            LOCKER_I["Locker(i)<br/>(Per-Target Lock)"]
        end
        
        subgraph "Memory Grow Operation"
            MEMORY_GROW_ALT["MemoryGrowAlt<br/>(Atomic Expand)"]
        end
    end
    
    subgraph "Lock Acquisition Pattern"
        direction LR
        SPIN["1. Spin Loop:<br/>Atomic::compare_exchange"]
        ENTER_CS["2. Enter Critical Section"]
        UPDATE["3. Update Shared State"]
        EXIT["4. Release Lock"]
    end
    
    ATOMIC_VAR --> SPIN
    SPIN --> ENTER_CS
    ENTER_CS --> UPDATE
    UPDATE --> EXIT
    
    GLOBAL_ALT_GET --> ENTER_CS
    GLOBAL_ALT_SET --> ENTER_CS
    GLOBAL_ALT_INIT --> ENTER_CS
    
    LOCKER_BASE --> SPIN
    LOCKER_I --> SPIN
    
    MEMORY_GROW_ALT --> SPIN
    
    style ATOMIC_VAR fill:#ffcdd2
    style GLOBAL_ALT_GET fill:#c8e6c9
    style GLOBAL_ALT_SET fill:#bbdefb
    style GLOBAL_ALT_INIT fill:#ffe0b2
    style GLOBAL_ALT_GET_NOWAIT fill:#f8bbd0
    style LOCKER_BASE fill:#d1c4e9
    style LOCKER_I fill:#c0ca33
    style MEMORY_GROW_ALT fill:#ffb74d
    style SPIN fill:#ffccbc
    style ENTER_CS fill:#a5d6a7
    style UPDATE fill:#64b5f6
    style EXIT fill:#ffb74d
```

### スレッド間ロック機構の詳細

#### Atomic Compare-Exchange Spin Lock
```
loop {
    old_value = atomic_var.load(Ordering::Acquire)
    if old_value == UNLOCKED {
        if atomic_var.compare_exchange(
            UNLOCKED, LOCKED,
            Ordering::Release,
            Ordering::Relaxed
        ).is_ok() {
            break;  // Lock acquired
        }
    }
}
```

#### GlobalAltGet（スレッドセーフな読み取り）
```
1. Lock獲得（Atomic compare-exchange）
2. グローバル変数の値をメモリから読み込み
3. Lock解放
4. 値を返却
```

#### GlobalAltSet（スレッドセーフな書き込み）
```
1. Lock獲得
2. グローバル変数にメモリへ値を書き込み
3. Lock解放
```

#### MemoryGrowAlt（Atomic メモリ拡張）
```
1. Locker(memory_id)でロック獲得
2. memory.grow 操作を実行
3. TargetMemoryMetadata.current_pages を更新
4. Lock解放
```

#### GlobalAltGetNoWait（ロック不要な高速読み取り）
```
- グローバル変数が読み取り専用の場合の最適化
- Lock なしで直接メモリ から値を読み込み
- Cache-coherency に依存
```

---

## 5. multi_memory との比較

`single_memory`と`multi_memory`の主要な違い：

### single_memory モード
- ✅ **利点**
  - メモリ操作が単純（1つのメモリインスタンス）
  - MemoryDirector ルーティングが効率的
  
- ❌ **課題**
  - スレッド同期が複雑（全モジュール が1つのメモリ を共有）
  - メモリ領域の分離管理が必要
  - reset sequence でのスタック破損リスク（既知の問題）

### multi_memory モード
- ✅ **利点**
  - 各モジュールが独立したメモリ を持つ
  - スレッド安全性が容易
  - reset sequence が安定
  
- ❌ **課題**
  - Memory 間のデータ コピーが頻繁
  - wasm-opt によるメモリ拡張が複雑
  - component 変換時に threads feature が未サポート

---

## 6. スレッド実行の詳細フロー

### Root Spawn vs Child Thread の分岐

```mermaid
graph TD
    START["wasi_thread_spawn 呼び出し"]
    CHECK{"isRootSpawn()?"}
    
    START --> CHECK
    
    CHECK -->|true| ROOT["Root Spawn<br/>(初期スレッド)"]
    CHECK -->|false| CHILD["Child Thread<br/>(子スレッド)"]
    
    ROOT --> ROOT1["1. Host-level spawn function を呼び出し<br/>(__wasip1_vfs_real_thread_spawn_fn)"]
    ROOT --> ROOT2["2. ホスト OS が新スレッドを生成"]
    ROOT --> ROOT3["3. Child Thread として継続"]
    
    CHILD --> CHILD1["1. __self_wasi_thread_start (VFS routing)"]
    CHILD --> CHILD2["2. VFS 経由でターゲットモジュールルーティング"]
    CHILD --> CHILD3["3. ターゲットモジュールの thread-start routine を実行"]
    CHILD --> CHILD4["4. スレッドローカルスタック で独立実行"]
    
    style ROOT fill:#b3e5fc
    style CHILD fill:#c8e6c9
```

---

## 7. メモリアクセスシーケンス例

### 複数スレッド が同時に共有グローバル変数にアクセスする場合

```mermaid
sequenceDiagram
    participant Thread1
    participant Thread2
    participant Locker
    participant SharedGlobal as Shared Global<br/>in Memory

    Thread1->>Locker: 1a. Lock Request
    Thread2->>Locker: 1b. Lock Request (Block)
    
    Locker->>Locker: 2a. compare_exchange<br/>(UNLOCKED → LOCKED)
    Note over Locker: Thread1 Lock Acquired
    
    Thread1->>SharedGlobal: 3a. GlobalAltGet (Read)
    SharedGlobal-->>Thread1: value = 42
    
    Thread1->>SharedGlobal: 4a. Compute new_value = 43
    Thread1->>SharedGlobal: 5a. GlobalAltSet (Write)
    SharedGlobal->>SharedGlobal: Update: 43
    
    Thread1->>Locker: 6a. Release Lock
    Note over Locker: Thread2 Lock Acquired
    
    Locker->>Locker: 2b. compare_exchange<br/>(UNLOCKED → LOCKED)
    
    Thread2->>SharedGlobal: 3b. GlobalAltGet (Read)
    SharedGlobal-->>Thread2: value = 43
    
    Thread2->>SharedGlobal: 4b. Compute new_value = 44
    Thread2->>SharedGlobal: 5b. GlobalAltSet (Write)
    SharedGlobal->>SharedGlobal: Update: 44
    
    Thread2->>Locker: 6b. Release Lock
```

---

## 8. 実装コード構造

プロジェクト内の関連ファイル：

| ファイル | 役割 |
|---------|------|
| `wasi_virt_layer-cli/src/generator/threads.rs` | ThreadsSpawn ジェネレータ（Root vs Child判定） |
| `wasi_virt_layer-cli/src/generator/shared_global.rs` | SharedGlobal ジェネレータ（ロック管理） |
| `wasi_virt_layer-cli/src/generator/memory.rs` | メモリレイアウト管理、TemporaryRefugeMemory |
| `wasi_virt_layer/src/shared_memory.rs` | TargetMemoryMetadata、SharedMemoryManager |
| `wasi_virt_layer/src/wasi/shared_global.rs` | グローバル変数アクセス実装 |

---

## 9. 已知の制限事項と対策

### 既知の問題：single_memory での reset sequence の失敗

**問題シナリオ：**
```
_reset() → _start() → _main() 
```
multi_memory では成功するが、single_memory では失敗することがある

**原因：**
- 共有メモリのスタック領域が上書きされる
- スレッドローカルスタックの初期化タイミングの問題

**対策：**
- multi_memory モード の使用推奨
- 或いは、スタック領域の明示的な初期化

### Component 変換での制限

**制限：**
- Wasip1-threads feature が component 変換時にサポートされていない

**対策：**
- TemporaryRefugeMemory により、shared フラグを一時的に外す
- component 変換後に再度リンク

---

## まとめ

WASI Virtual Layer（WVL）の `single_memory`モード下での複数スレッド実行は、以下の機構により実現されています：

1. **統一されたメモリ空間**: VFS、WasmA、WasmB が1つのメモリを共有
2. **メタデータ管理**: TargetMemoryMetadata で各モジュールのメモリ領域を追跡
3. **スレッド分岐**: Root Spawn と Child Thread で異なるルーティング
4. **Atomic ロック**: 複数スレッド間のメモリアクセスを同期
5. **SharedGlobal 関数**: グローバル変数への安全なアクセス

この設計により、複数のWasm モジュールが協調して動作し、VFS を通じたファイルシステムアクセスやスレッド間通信が実現されます。
