# minimal_repro_virtual 実行時メモリフロー詳細解析

## 概要

このドキュメントは、`examples/vfs/minimal_repro_virtual`プロジェクトの実行時メモリレイアウトと処理フローを詳細に解析したものです。

**プロジェクト構成：**
- **VFS Module** (minimal_repro_virtual): メインモジュール
- **Target 1** (test_threads): スレッド対応ターゲット（wasip1-threads）
- **Target 2** (ls): ファイルシステムターゲット（wasip1）
- **Mode**: single_memory、threads 有効

---

## 1. 初期メモリレイアウト（起動時）

```mermaid
graph TB
    subgraph "Shared Linear Memory (0x00000000 - MAX)"
        direction TB
        
        subgraph "VFS Region (Address Range: 0x0000 - 0x10000)"
            VFS_META["VFS Metadata<br/>(Hello world)"]
            VFS_POOL["THREAD_POOL<br/>(VirtualThreadPool)"]
            VFS_FS["VIRTUAL_FILE_SYSTEM<br/>(StandardMultipleFileSystem)"]
            VFS_ENV["VIRTUAL_ENV<br/>(LazyLock Mutex)"]
            VFS_HEAP["VFS Heap<br/>(Vec, LazyLock storage)"]
        end
        
        subgraph "Shared Metadata Region (0x10000 - 0x12000)"
            METADATA["TargetMemoryMetadata[]<br/>- test_threads metadata<br/>- ls metadata"]
            ATOMIC_LOCKS["Atomic Lock Variables<br/>(for memory.grow)"]
        end
        
        subgraph "test_threads Region (0x12000 - 0x50000)"
            TT_HEAP["test_threads Heap<br/>(Thread local data)"]
            TT_STACK["test_threads Stack<br/>(Main + Child)"]
            TT_GLOBALS["test_threads Globals"]
        end
        
        subgraph "ls Region (0x50000 - 0x60000)"
            LS_HEAP["ls Heap<br/>(Directory entries)"]
            LS_STACK["ls Stack"]
            LS_GLOBALS["ls Globals"]
        end
    end
    
    style VFS_META fill:#b3e5fc
    style VFS_POOL fill:#81d4fa
    style VFS_FS fill:#4fc3f7
    style VFS_ENV fill:#29b6f6
    style VFS_HEAP fill:#03a9f4
    style METADATA fill:#fff9c4
    style ATOMIC_LOCKS fill:#ffeb3b
    style TT_HEAP fill:#f3e5f5
    style TT_STACK fill:#e1bee7
    style TT_GLOBALS fill:#ce93d8
    style LS_HEAP fill:#c8e6c9
    style LS_STACK fill:#a5d6a7
    style LS_GLOBALS fill:#81c784
```

---

## 2. 実行フェーズ1: VFS 初期化

```mermaid
sequenceDiagram
    participant Host as Host/Deno Runtime
    participant VFS as VFS Module<br/>(minimal_repro_virtual)
    participant POOL as THREAD_POOL<br/>(VirtualThreadPool)
    participant FS as VIRTUAL_FILE_SYSTEM<br/>(StandardMultipleFileSystem)
    participant SharedMem as Shared Memory

    Host->>VFS: 1. Instantiate Component
    VFS->>SharedMem: 2. Allocate VFS Region (0x0000-0x10000)
    
    Host->>VFS: 3. Call main()
    Note over VFS: unsafe { THREAD_POOL.init() }
    VFS->>POOL: 4. Initialize VirtualThreadPool
    POOL->>SharedMem: Create thread pool metadata (capacity=1)
    
    VFS->>POOL: 5. THREAD_POOL.set_capacity(1)
    POOL->>SharedMem: 6. Allocate 1 thread entry
    
    VFS->>POOL: 7. THREAD_POOL.flush_capacity().wait()
    POOL->>SharedMem: 8. Setup thread pool ready state
    
    VFS->>FS: 9. Initialize VIRTUAL_FILE_SYSTEM (LazyLock)
    FS->>SharedMem: 10. Allocate FS Metadata Region (0x10000-0x12000)
    
    FS->>FS: 11. add_wasm::<test_threads>()
    FS->>SharedMem: Register test_threads (base=0x12000, limit=0x50000)
    
    FS->>FS: 12. add_wasm::<ls>()
    FS->>SharedMem: Register ls (base=0x50000, limit=0x60000)
    
    FS->>FS: 13. add_lfs(StandardDynamicLFS)
    FS->>SharedMem: Add host filesystem access layer
```

---

## 3. 実行フェーズ2: test_threads 実行（スレッド機能）

```mermaid
sequenceDiagram
    participant VFS as VFS (main)
    participant TP as THREAD_POOL
    participant TT as test_threads<br/>Module
    participant RT as test_threads<br/>Root Thread
    participant CT as test_threads<br/>Child Thread
    participant Mem as Shared Memory

    VFS->>TT: 1. Call test_threads::_reset()
    TT->>Mem: Clear test_threads region (0x12000-0x50000)
    
    VFS->>TT: 2. Call test_threads::_start()
    TT->>RT: 3. Initialize Root Thread (_start function)
    RT->>Mem: 4. Setup main stack (0x12000)
    
    VFS->>TT: 5. Call test_threads::_main()
    RT->>RT: 6. Execute: println!("Hello, world!")
    RT->>Mem: 7. Log to stdout via VFS I/O redirection
    
    RT->>RT: 8. Execute: std::thread::spawn(closure)
    RT->>TP: 9. Request thread spawn (isRootSpawn=true)
    
    Note over TP: Root Spawn Detection
    TP->>TP: 10. Check isRootSpawn() → true
    TP->>Host: 11. Call host thread spawn function
    Host->>CT: 12. Allocate new thread stack
    
    CT->>TT: 13. Enter thread start routine
    CT->>Mem: 14. Setup child thread local stack
    
    par Parallel Execution
        RT->>RT: 15a. Root: println!("Hello from a thread!")
        RT->>Mem: 16a. Root: Write to stdout
        
        CT->>CT: 15b. Child: Execute closure
        CT->>Mem: 16b. Child: Write to stdout
    end
    
    CT->>RT: 17. Signal thread completion
    RT->>RT: 18. join().unwrap() returns
    RT->>VFS: 19. Return from test_threads::_main()
```

---

## 4. 実行フェーズ3: ls 実行

```mermaid
sequenceDiagram
    participant VFS as VFS
    participant FS as VIRTUAL_FILE_SYSTEM
    participant LS as ls Module
    participant LSR as ls Root Thread
    participant Mem as Shared Memory
    participant HostFS as Host FileSystem

    VFS->>LS: 1. Call ls::_reset()
    LS->>Mem: 2. Clear ls region (0x50000-0x60000)
    
    VFS->>LS: 3. Call ls::_start()
    LSR->>Mem: 4. Initialize stack and globals
    
    VFS->>LS: 5. Call ls::_main()
    LSR->>LSR: 6. Execute: fs::read_dir(".")
    
    LSR->>FS: 7. VFS intercept: fs::read_dir(".")
    FS->>FS: 8. Check filesystem mounting
    Note over FS: - test_threads (VFS mounted)
    Note over FS: - ls (VFS mounted)
    Note over FS: - Host LFS (Standard Dynamic)
    
    FS->>Mem: 9. Prepare directory listing in LS heap
    
    par Parallel Iteration
        LS->>FS: 10a. Read test_threads VFS entry
        FS->>Mem: 10a. Copy data from test_threads region
        LS->>LS: 10a. println!("test_threads/")
        
        LS->>FS: 10b. Read ls VFS entry
        FS->>Mem: 10b. Copy data from ls region
        LS->>LS: 10b. println!("ls/")
        
        LS->>HostFS: 10c. List host directory
        HostFS-->>FS: Return host entries
        LS->>LS: 10c. println!("./host_file")
    end
    
    LSR->>VFS: 11. Return from ls::_main()
```

---

## 5. メモリアクセスの詳細：MemoryCopyTo/From

```mermaid
graph TB
    subgraph "データ流通パス"
        direction TB
        
        subgraph "VFS 側操作"
            VFS_OP["VFS Operation<br/>(fs::read_dir, etc.)"]
            VFS_BUFFER["VFS Buffer<br/>(Shared Metadata Region)"]
        end
        
        subgraph "メモリコピー層"
            COPY_FROM["MemoryCopyFrom<br/>(Target→VFS)<br/>Atomic Read"]
            COPY_TO["MemoryCopyTo<br/>(VFS→Target)<br/>Atomic Write"]
        end
        
        subgraph "ターゲット側メモリ"
            TT_DATA["test_threads Data<br/>(0x12000-0x50000)"]
            LS_DATA["ls Data<br/>(0x50000-0x60000)"]
        end
        
        subgraph "制御フロー"
            DIRECTOR["MemoryDirector<br/>(Pointer Routing)"]
            LOCKER["Locker(i)<br/>(per-Target Atomic Lock)"]
        end
    end
    
    VFS_OP -->|Request Data| VFS_BUFFER
    VFS_BUFFER -->|Acquire Lock| LOCKER
    LOCKER -->|Access test_threads| COPY_FROM
    COPY_FROM -->|Read from| TT_DATA
    TT_DATA -->|Data Flow| VFS_BUFFER
    
    VFS_BUFFER -->|Write Data| COPY_TO
    COPY_TO -->|Atomic Write| TT_DATA
    
    DIRECTOR -->|Route Pointer| LOCKER
    LOCKER -->|Release| VFS_BUFFER
    
    style VFS_OP fill:#b3e5fc
    style VFS_BUFFER fill:#81d4fa
    style COPY_FROM fill:#c8e6c9
    style COPY_TO fill:#a5d6a7
    style TT_DATA fill:#f3e5f5
    style LS_DATA fill:#ce93d8
    style DIRECTOR fill:#fff9c4
    style LOCKER fill:#ffeb3b
```

---

## 6. スレッド実行時のメモリスナップショット

### 6a. test_threads 実行中（Child Thread 生成前）

```
Memory State: After test_threads::_main() called

Shared Linear Memory:
┌────────────────────────────────────────────┐
│ 0x0000: VFS Region                         │
│   - THREAD_POOL metadata                   │
│   - VIRTUAL_FILE_SYSTEM mounted modules    │
│   - VIRTUAL_ENV (Mutex<VirtualEnvState>)   │
│   - VFS Heap (Vec, etc.)                   │
├────────────────────────────────────────────┤
│ 0x10000: Shared Metadata                   │
│   - TargetMemoryMetadata[test_threads]     │
│     {base_ptr: 0x12000, limit_ptr: 0x50000}
│   - TargetMemoryMetadata[ls]               │
│     {base_ptr: 0x50000, limit_ptr: 0x60000}
│   - Atomic Lock Variables                  │
├────────────────────────────────────────────┤
│ 0x12000: test_threads Region (ACTIVE)      │
│   - Heap: dir listing, stdout buffer       │
│   - Main Stack: [Frame: _main, etc.]       │
│     Stack Pointer (SP): 0x12100            │
│   - Globals: module-level static vars      │
├────────────────────────────────────────────┤
│ 0x50000: ls Region (INACTIVE)              │
│   - Heap: (unused)                         │
│   - Stack: (unused)                        │
│   - Globals: (uninitialized)               │
└────────────────────────────────────────────┘
```

### 6b. Child Thread スポーン後（並列実行中）

```
Memory State: Child Thread active in test_threads

Root Thread (Main):
┌─────────────────────────────────────┐
│ Stack Frame: _main → join()         │
│ Stack Pointer: 0x12100              │
│ Waiting for child completion        │
└─────────────────────────────────────┘
         ↓ (Blocked on join)

Child Thread:
┌─────────────────────────────────────┐
│ Stack Frame: closure (spawned fn)   │
│ Stack Pointer: 0x12080 (new entry)  │
│ Executing: println!(...)            │
│ Lock Count: 1 (holds Locker)        │
└─────────────────────────────────────┘

Shared Resources:
┌─────────────────────────────────────┐
│ Locker(test_threads):               │
│   Status: LOCKED (owned by Child)   │
│   Lock Count: Atomic<i32> = 1       │
│                                      │
│ stdout buffer:                      │
│   "Hello, world!"                   │
│   "Hello from a thread!"            │
└─────────────────────────────────────┘
```

### 6c. Child Thread 終了後

```
Memory State: After Child Thread join()

Shared Memory:
┌────────────────────────────────────────────┐
│ 0x12000: test_threads Region               │
│   - Heap: cleaned up (Locker released)     │
│   - Main Stack: back at join() completion  │
│   - Child Stack Frame: deallocated         │
├────────────────────────────────────────────┤
│ Locker(test_threads):                      │
│   Status: UNLOCKED (Atomic<i32> = 0)       │
└────────────────────────────────────────────┘

Control Flow:
test_threads::_main() completes → VFS main() continues
```

---

## 7. VFS i/o インターセプション

```mermaid
graph LR
    TT["test_threads<br/>Call: println!()"]
    INTERCEPT["VFS Intercept<br/>(plug_fs!)"]
    VFS_IO["VFS I/O Handler<br/>(fd_write)"]
    STDOUT["stdout buffer"]
    
    TT -->|System Call| INTERCEPT
    INTERCEPT -->|Route through| VFS_IO
    VFS_IO -->|Write| STDOUT
    
    STDOUT -->|Via Deno| HOST["Host Console"]
    
    style TT fill:#f3e5f5
    style INTERCEPT fill:#fff9c4
    style VFS_IO fill:#c8e6c9
    style STDOUT fill:#81d4fa
    style HOST fill:#b3e5fc
```

---

## 8. StandardMultipleFileSystem ル―ティング

minimal_repro_virtual では 3 つのファイルシステムをマウント：

```mermaid
graph TB
    subgraph "StandardMultipleFileSystem"
        direction TB
        
        VFS["VFS Router<br/>(ls::read_dir)"]
        
        VFS -->|Path: /test_threads/| TT_FS["test_threads VFS<br/>(add_wasm::<test_threads>)"]
        VFS -->|Path: /ls/| LS_FS["ls VFS<br/>(add_wasm::<ls>)"]
        VFS -->|Path: /| HOST_FS["Host LFS<br/>(StandardDynamicLFS)"]
    end
    
    TT_FS -->|Read from| TT_MEM["test_threads Memory<br/>(0x12000-0x50000)"]
    LS_FS -->|Read from| LS_MEM["ls Memory<br/>(0x50000-0x60000)"]
    HOST_FS -->|Read from| HOST_DIR["Host Directory"]
    
    style VFS fill:#fff9c4
    style TT_FS fill:#f3e5f5
    style LS_FS fill:#c8e6c9
    style HOST_FS fill:#e8f5e9
    style TT_MEM fill:#ce93d8
    style LS_MEM fill:#a5d6a7
    style HOST_DIR fill:#c5e1a5
```

---

## 9. 完全な実行シーケンス

```mermaid
sequenceDiagram
    participant Deno as Deno Runtime
    participant VFS as VFS<br/>(minimal_repro_virtual)
    participant TT as test_threads
    participant LS as ls
    participant Mem as Shared Memory

    Deno->>VFS: 1. Instantiate & call main()
    VFS->>Mem: 2. Initialize VFS Region
    VFS->>VFS: 3. THREAD_POOL.init()
    VFS->>VFS: 4. VIRTUAL_FILE_SYSTEM.init()
    Mem->>Mem: 5. Register TargetMemoryMetadata[TT, LS]
    
    VFS->>TT: 6. _reset()
    Mem->>Mem: 7. Clear test_threads region
    
    VFS->>TT: 8. _start()
    TT->>Mem: 9. Setup main thread
    
    VFS->>TT: 10. _main()
    TT->>TT: 11a. println!("Hello, world!")
    Mem->>Deno: 11b. stdout output
    
    TT->>VFS: 12. std::thread::spawn()
    VFS->>VFS: 13. isRootSpawn() → true
    VFS->>Deno: 14. Request host thread
    Deno->>Deno: 15. Create new thread
    
    par Execution
        TT->>Deno: 16a. println!("Hello from thread!")
        TT->>TT: 16b. Child executes
    end
    
    TT->>TT: 17. join() completes
    VFS->>TT: 18. test_threads done
    
    VFS->>LS: 19. _reset()
    Mem->>Mem: 20. Clear ls region
    
    VFS->>LS: 21. _start()
    LS->>Mem: 22. Setup stack
    
    VFS->>LS: 23. _main()
    LS->>LS: 24. fs::read_dir(".")
    
    VFS->>VFS: 25. Intercept fs operation
    VFS->>Mem: 26. Route through StandardMultipleFileSystem
    
    Mem->>Deno: 27a. test_threads VFS entry
    Deno->>LS: 27b. println!("test_threads/")
    
    Mem->>Deno: 28a. ls VFS entry
    Deno->>LS: 28b. println!("ls/")
    
    Mem->>Deno: 29. Host filesystem entries
    LS->>Deno: 30. println!("./host_file")
    
    VFS->>Deno: 31. Return completed
```

---

## 10. メモリ使用量の推定

| コンポーネント | 予想アドレス範囲 | 使用量 | 目的 |
|--------|------------|------|------|
| VFS Metadata | 0x0000 - 0x1000 | ~4 KB | ComponentABI, Hello impl |
| THREAD_POOL | 0x1000 - 0x2000 | ~4 KB | VirtualThreadPool (capacity=1) |
| VIRTUAL_FILE_SYSTEM | 0x2000 - 0x4000 | ~8 KB | StandardMultipleFileSystem + mounts |
| VIRTUAL_ENV | 0x4000 - 0x8000 | ~16 KB | LazyLock<Mutex<>> + environ Vec |
| VFS Heap/Stack | 0x8000 - 0x10000 | ~32 KB | Runtime allocation |
| TargetMemadata | 0x10000 - 0x10200 | ~512 B | [test_threads, ls] metadata |
| Atomic Locks | 0x10200 - 0x10800 | ~1.5 KB | Locker storage |
| test_threads | 0x12000 - 0x50000 | ~224 KB | Full heap + stack + globals |
| ls | 0x50000 - 0x60000 | ~64 KB | Full heap + stack + globals |
| **合計** | 0x00000 - 0x60000 | ~384 KB | Total per execution |

---

## 11. スレッド同期の詳細

### Root Spawn 判定フロー

```
std::thread::spawn() call in test_threads
    ↓
VFS intercepts: wasi_thread_spawn()
    ↓
Check: isRootSpawn()?
    ├─ YES (Root Thread) → 
    │   Call __wasip1_vfs_real_thread_spawn_fn (Host-level spawn)
    │   ↓
    │   Host OS creates new thread
    │   ↓
    │   Child Thread enters thread start routine
    │
    └─ NO (Child Thread) →
        __self_wasi_thread_start (VFS routing)
        ↓
        Child thread initialization
```

### ロック競合シナリオ（多くの場合発生しない）

minimal_repro_virtual では:
- test_threads: Root + 1 Child のみ
- ls: Single-threaded

**最大同時ロック争合度**: 1 (test_threads.Child waiting for Root unlock)

```
Timeline:
┌─────────────────────────────────────┐
│ Root Thread                         │
│ [Lock acquired for memory op]       │
│ ... reading from target memory ...  │
│ [Lock held]                         │
└─────────────────────────────────────┘
         ↑ Atomic Lock State = 1
┌─────────────────────────────────────┐
│ Child Thread                        │
│ [Spinning on lock acquire]          │
│ ... waiting for memory access ...   │
│ [Spin loop: compare_exchange loop]  │
└─────────────────────────────────────┘
         ↑ Trying to acquire = blocked
```

---

## 12. 実行結果の予想出力

```
--- Starting test_threads ---
Hello, world!
Hello from a thread!
--- Starting ls ---
test_threads/
ls/
<host filesystem entries>
```

---

## 13. 重要なポイント

### メモリ配置の最適化
1. **VFS Region** が小さく（64 KB）、ほとんどが LazyLock initialization
2. **test_threads Region** が大きい（224 KB）、Rust std thread support のため
3. **ls Region** が中程度（64 KB）、directory listing data 格納

### ス レッド安全性
- test_threads 内の thread::spawn は **isRootSpawn() = true** を検出
- Root Spawn は ホストレベルでの OS スレッド生成を要求
- Child Thread は VFS ルーティングを通じてタ ーゲットモジュールで実行

### ファイルシステム仮想化
- **StandardMultipleFileSystem** が 3 つのソースをマウント
- VFS の read_dir は各ソースを列挙
- HostLFS により host ファイルシステムへのアクセス も実現

---

## 14. トラブルシューティングと予想される問題

| 問題 | 原因 | 対策 |
|------|------|------|
| スレッド作成失敗 | isRootSpawn 判定エラー | threads.rs のロジック確認 |
| メモリアクセス違反 | MemoryCopyFrom/To の不整合 | memory.rs のアドレス計算確認 |
| ファイル読み取り失敗 | StandardMultipleFileSystem マウント漏れ | fs マウント順序確認 |
| stdout 出力なし | VFS I/O インターセプション失敗 | plug_fs! マクロ確認 |

---

## まとめ

`minimal_repro_virtual` は WASI Virtual Layer の完全なエコシステムを示す例です：

1. **複数モジュール統合**: VFS + test_threads + ls を single_memory で統合
2. **スレッド処理**: test_threads のスレッド生成と同期を実装
3. **ファイルシステム仮想化**: 複数ソースのマウントと統合
4. **メモリ管理**: TargetMemoryMetadata によるメモリ領域追跡
5. **I/O インターセプション**: WASI 呼び出しの VFS への再ルーティング

このプロジェクトを理解することで、WASI Virtual Layer の核となるメモリレイアウト、スレッド管理、ファイルシステム仮想化の全体像が明確になります。
