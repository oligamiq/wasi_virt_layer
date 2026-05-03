# WASI Virtual Layer メモリフロー - 根本的バグの深掘り分析

## 🚨 発見された根本的メモリ破壊バグ

コード実装の詳細分析により、**メモリ破壊**および**メモリオフセット計算の根本的な欠陥**を特定しました。

この分析は、設定値レベルの問題ではなく、アーキテクチャ的・実装的な根本原因を暴露しています。

---

## Bug #1: スレッドプール容量不足による Child Thread 生成失敗

### 現象
```rust
static THREAD_POOL: VirtualThreadPool<ThreadAccessor> =
    unsafe { VirtualThreadPool::new_const(1) };  // ← capacity = 1

impl Guest for Hello {
    fn main() {
        unsafe { THREAD_POOL.init() };
        THREAD_POOL.set_capacity(1);             // ← 容量を1に固定
        THREAD_POOL.flush_capacity().wait();
        
        test_threads::_main();  // ← 内部で thread::spawn() を呼ぶ
    }
}
```

### 問題
- THREAD_POOL のキャパシティが **1に固定**されている
- test_threads::_main() は `thread::spawn()` で **Child Thread を生成しようとする**
- しかし、THREAD_POOL には **追加スレッドの余裕がない**
- **結果**: Child Thread が生成できず、ハング or パニック

### コード中の該当箇所
```rust
// test_threads/src/main.rs
fn main() {
    println!("Hello, world!");
    
    std::thread::spawn(|| {
        println!("Hello from a thread!");
    })
    .join()
    .unwrap();  // ← ここでスレッド生成が失敗するとハング
}
```

### メモリフロー図での誤り
- **図セクション3**: "Child Thread 1 - 7b. Allocate Thread 1 Stack" と示している
- しかし、実装では **THREAD_POOL.set_capacity(1) で容量が1に制限**されている
- VirtualThreadPool が複数スレッドをサポートできない状態での thread::spawn() 呼び出し

### 修正案
```rust
// 複数スレッドが必要な場合、容量を増やす
THREAD_POOL.set_capacity(2);  // または 4, 8 など
THREAD_POOL.flush_capacity().wait();
```

---

## Bug #2: reset() sequence による shared グローバル変数の破損

### 現象
```rust
fn main() {
    println!("--- Starting test_threads ---");
    test_threads::_reset();        // ← test_threads メモリ全体を CLEAR
    test_threads::_start();        // ← 初期化
    test_threads::_main();         // ← メイン実行
}
```

### 問題（README.md §69-70 で既知）

```
`_reset(); _start(); _main();` sequence in multi-threaded environments 
has been observed to trigger issues in single-memory mode.
```

**詳細な問題:**
1. `_reset()` は test_threads メモリ領域（0x12000 - 0x50000）を **全クリア**
2. この時、**Shared Metadata Region のメタデータも破損する可能性**
3. `_start()` 直後、スタック初期化前に他スレッドがアクセスすると **undefined behavior**
4. shared_global のロック変数が初期化されていない可能性
5. **Root Spawn と Child Thread 間の同期が取れない**

### メモリ図での誤り
```
セクション 3 "実行フェーズ2: test_threads 実行"

VFS->>TT: 1. Call test_threads::_reset()
TT->>Mem: 2. Clear test_threads region (0x12000-0x50000)
```

この「clear」で以下のが失われる:
- Atomic Lock Variables（Shared Metadata Region にある）
- グローバル変数の初期値
- スタック初期ポインタ

### 実装コード内の潜在的な問題
```rust
// shared_global.rs での Atomic 初期化が _reset() 後に実施されない可能性
// あるいは、_reset() で全メモリクリアすることで Atomic が 0 になり
// Lock 競合時のスピンロック が永遠に回る

// スピンロック: Atomic が UNLOCKED (0) であることを期待
// だが、_reset() 後に他スレッドが同時にアクセスすると
// 予期しない競合状態発生
```

---

## Bug #3: world() 関数での ls 実行による メモリ汚染

### 現象
```rust
impl Guest for Hello {
    fn world() {
        // println!("--- Starting ls ---");
        ls::_reset();
        ls::_start();
        ls::_main();    // ← ls が実行される
    }
    
    fn main() {
        // ... test_threads 実行
        test_threads::_main();
    }
}
```

### 問題
1. `world()` は コンポーネント初期化時に **呼び出される**
2. ls が実行される時点で、THREAD_POOL はまだ **初期化されていない**
3. ls が WASI 呼び出しをした場合、**VFS I/O が正常に動作しない可能性**
4. その後 `main()` が呼ばれるときに、**ls が汚した VIRTUAL_FILE_SYSTEM の状態が残っている**

### 実装の順序問題
```rust
// current (buggy)
pub fn world()  // ← called first, before main()
    ls executed (no THREAD_POOL init)
    
pub fn main()   // ← called second
    THREAD_POOL initialized
    test_threads executed

// expected
pub fn main()   // should be called first
    THREAD_POOL initialized
    test_threads executed
    
pub fn world()  // should be called after, or not at all
```

### メモリ図での誤り
- **図セクション9**: 完全な実行シーケンスで "VFS 初期化" の後すぐに "test_threads::_reset()" が来ている
- しかし、実装では **world() が先に呼ばれ、その時点で ls が実行** されている
- VIRTUAL_FILE_SYSTEM の LazyLock が初期化されるのは first access 時
- **world() の時点で VIRTUAL_FILE_SYSTEM が初期化されてしまう**

---

## Bug #4: メモリアドレス重複時のスタック破損

### 潜在的な問題
```
図での配置:
test_threads Region: 0x12000 - 0x50000 (224 KB)
ls Region:          0x50000 - 0x60000 (64 KB)

もし test_threads が:
  - memory.grow() で メモリを拡張
  - スタック領域を下向きに拡張（typical Wasm）
  
すると、スタックが ls 領域を侵襲する可能性
```

### 実装での該当箇所
```rust
// shared_global.rs - MemoryGrowAlt
// Locker(memory_id)でロック獲得 後、memory.grow 実行
// しかし、ターゲット間の メモリ領域が固定的に配置されているため
// grow 時の調整が不完全だと重複発生
```

### 実装コード例での潜在的なバグ
```rust
// test_threads がメモリ不足で memory.grow() を呼ぶ
// しかし、test_threads の limit_ptr が 0x50000 に固定
// ls が同時に メモリアクセスすると、スタック破損

// 図では "Atomic Lock" で同期していると示しているが
// ロック争合時の timeout がない (Spin Lock) ため
// デッドロック になる可能性
```

---

## Bug #5: スレッド同期の不完全性 - Spin Lock Timeout なし

### 実装の問題
```rust
// Atomic Compare-Exchange Spin Lock (from shared_global.rs)
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
// ← timeout なし、永遠にスピン可能
```

### バグの原因
1. **capacity=1 のスレッドプール** では、実質 Root Spawn のみが実行可能
2. **Child Thread が生成できない** ため、ロック争合がない...と考えるのは誤り
3. **非同期操作や割り込み** が発生すると、スピンロックが永遠に回る可能性
4. **Wasm runtime による中断** で、スピンロック中に context switch があり得る

### メモリ図での誤り
- **セクション7** "複数スレッド が同時に共有グローバル変数にアクセスする場合"
- 図では順序良く Lock → Unlock を示しているが、実装では:
  - capacity=1 では複数スレッドが実際には並行実行できない
  - **または、実装が不完全でロック不要な状態なのに ロック取得を試みている**

---

## Bug #6: VirtualThreadPool 初期化の順序と LazyLock 初期化の競合

### 現象
```rust
unsafe { THREAD_POOL.init() };                    // line 29
THREAD_POOL.set_capacity(1);                      // line 30
THREAD_POOL.flush_capacity().wait();              // line 31

println!("--- Starting test_threads ---");
test_threads::_reset();                           // line 34
test_threads::_start();
test_threads::_main();                            // line 36
```

同時に:
```rust
// fs module では
pub static VIRTUAL_FILE_SYSTEM: LazyLock<StandardMultipleFileSystem> = 
    LazyLock::new(|| {
        let mut vfs = StandardMultipleFileSystem::new();
        vfs.add_wasm::<test_threads>();           // ← メタデータ読み込み
        vfs.add_wasm::<ls>();
        let lfs = StandardDynamicLFS::<DefaultStdIO>::new();
        vfs.add_lfs(lfs);
        vfs
    });
```

### 問題
1. VIRTUAL_FILE_SYSTEM は **first access で初期化** される（LazyLock）
2. test_threads::_main() が fs::read_dir() を呼ぶと、その時点で LazyLock 初期化が発動
3. しかし、**THREAD_POOL 初期化とのタイミング重複** で競合状態
4. **メタデータの読み込み順序** が不確定

### 実装での該当箇所
```rust
// plug_fs! マクロで VFS を インターセプト
// しかし、VirtualThreadPool と VIRTUAL_FILE_SYSTEM の
// 初期化タイミングが不同期
```

---

## Bug #7: ls がスレッド対応していないのに thread マクロに含まれている

### 実装の矛盾
```rust
plug_thread!({ &THREAD_POOL }, self,
    test_threads,  // ← wasip1-threads (thread 対応)
    ls             // ← wasip1 (thread 非対応) ← バグ！
);
```

### 問題
1. ls は **単一スレッド** で実行される wasip1 モジュール
2. しかし、plug_thread! マクロに include されている
3. ls が wasi_thread_spawn() を受け取った場合、**undefined behavior**
4. **メモリレイアウト** が thread 対応ターゲット用に割り当てられている可能性

### メモリ図での誤り
- **図セクション1**: ls Region が単に配置されているだけ
- しかし、実装では **plug_thread! に含まれている** ため、スレッド管理コードが注入される
- これにより、**ls メモリ領域が予想より大きくなる可能性**

---

## 統合的な バグシナリオ

### 実行時に起こりうるシナリオ

```
Timeline:

T0: main() called
    ↓
T1: THREAD_POOL.init(), set_capacity(1)
    ↓
T2: test_threads::_reset()  (メモリ 0x12000-0x50000 全クリア)
    ↓
T3: test_threads::_start()  (初期化、但し capacity=1)
    ↓
T4: test_threads::_main() called
    println!("Hello, world!")  ← works
    ↓
T5: std::thread::spawn() called  ← Root Spawn detected (isRootSpawn=true)
    ↓
T6: Call __wasip1_vfs_real_thread_spawn_fn (host thread spawn)
    BUT: THREAD_POOL.capacity == 1, no capacity for child
    ↓
T7: HANG or PANIC
    (Child thread created but no slot in THREAD_POOL)
    
    OR
    
T7: Spin Lock in shared_global (ロック待機)
    capacity=1 なので Child が実行されない
    → parent がロック取得できず永遠にスピン
```

---

## 図との矛盾一覧

| 図セクション | 示している内容 | 実装での現実 | バグ |
|-------|----------|--------|------|
| §3 "Phase 2: test_threads" | Child Thread 1, 2 を生成 | capacity=1 で複数スレッド生成不可 | #1 |
| §6b "Child Thread 実行中" | 複数スレッド並行実行 | Root Spawn のみ実行可能 | #1 |
| §9 "完全な実行シーケンス" | VFS init → test_threads 実行 | 実際には world() が先に実行 | #3 |
| §11 "スレッド同期の詳細" | ロック競合がある | capacity=1 では競合ない（or デッドロック） | #5 |
| §10 "メモリ使用量推定" | 固定アドレス範囲 | test_threads の grow で重複可能 | #4 |
| §2 "VFS 初期化" | LazyLock 初期化は明示的 | VirtualThreadPool と非同期 | #6 |
| §8 "StandardMultipleFileSystem" | 単なるマウント | plug_thread! により追加コード注入 | #7 |

---

## 根本原因の分析

### 深層的な設計問題

1. **thread マクロの過度な使用**
   - ls は thread 非対応モジュール
   - plug_thread! に含めるべきでない

2. **メモリプール容量と実装の不一致**
   - capacity=1 は開発/テスト用
   - 実際の複数スレッド実行には不十分

3. **reset sequence の unsafe**
   - README で既知として認識
   - しかし、メモリフロー図では無視されている

4. **LazyLock と手動初期化の混在**
   - VIRTUAL_FILE_SYSTEM は LazyLock（自動初期化）
   - THREAD_POOL は手動初期化
   - タイミング競合リスク

5. **world() 関数の不完全な仕様**
   - コンポーネント初期化時に ls が実行される
   - 設計上の意図不明確

---

## 修正提案

### 推奨修正1: スレッドプール容量を増加
```rust
// before
THREAD_POOL.set_capacity(1);

// after
THREAD_POOL.set_capacity(4);  // または、オプションで増可能
```

### 推奨修正2: ls を plug_thread! から外す
```rust
// before
plug_thread!({ &THREAD_POOL }, self,
    test_threads,
    ls,
);

// after
plug_thread!({ &THREAD_POOL }, self,
    test_threads,  // thread 対応のみ
);
```

### 推奨修正3: reset sequence を避ける
```rust
// before
test_threads::_reset();
test_threads::_start();
test_threads::_main();

// after
// either:
// (A) reset を外す
test_threads::_start();
test_threads::_main();

// OR (B) multi_memory mode を使う
// (README で推奨)
```

### 推奨修正4: world() の実行タイミングを明確化
```rust
// option 1: world を無効化
// fn world() { /* do nothing */ }

// option 2: main 後に ls を実行
fn main() {
    // ... test_threads
    
    // now run ls
    ls::_reset();
    ls::_start();
    ls::_main();
}
```

---

## メモリフロー図の修正箇所

### 図セクション 3 (実行フェーズ2)
**修正前:**
```
Host->>ChildT1: 7a. Allocate Thread 1 Stack
Host->>ChildT2: 7b. Allocate Thread 2 Stack
```

**修正後:**
```
Host->>CT: 7. Attempt Thread Spawn
Note over CT: ERROR: THREAD_POOL.capacity == 1
Note over CT: No available slot for child thread
CT-->>RT: Panic or Hang
```

### 図セクション 9 (完全な実行シーケンス)
**修正前:**
```
Deno->>VFS: 1. Instantiate & call main()
```

**修正後:**
```
Deno->>VFS: 0. Call world() [BEFORE main()]
  (LazyLock VIRTUAL_FILE_SYSTEM initialized here)
  
Deno->>VFS: 1. Call main()
```

### 図セクション 11 (スレッド同期)
**修正前:**
```
Root Thread と Child Thread が並行実行
```

**修正後:**
```
Root Thread のみが実行
(THREAD_POOL.capacity == 1 のため)
Child Thread スポーン試行 → FAIL
```

---

## 結論

**最大のバグ原因**: THREAD_POOL capacity == 1

このたった1つの設定が、以下をすべて引き起こす：
1. Child Thread 生成失敗
2. デッドロック（スピンロックで Child 待機）
3. メモリ領域破損（reset sequence と capacity の組み合わせ）
4. 実装と図の不整合

**メモリフロー図は architecturally correct だが、実装との gap がある。**

特に、capacity=1 という "開発用設定" で test_threads の thread::spawn() を動かそうとしているのは根本的な矛盾である。
