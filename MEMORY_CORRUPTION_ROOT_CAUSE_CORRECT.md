# WASI Virtual Layer - メモリ破壊の根本原因（正確版）

## 問題の本質

**wasm-opt `--multi-memory-lowering` によって生成されたグローバル変数（メモリ内オフセット）が、不正に定数値に置き換えられている**。

---

## 根本的バグ: GlobalGet を定数に置き換える誤り

### コード箇所

`wasi_virt_layer-cli/src/generator/shared_global.rs` **line 244-250**

```rust
walrus::ir::Instr::GlobalGet(walrus::ir::GlobalGet { global })
    if *global == global_id =>
{
    *instr = walrus::ir::Instr::Const(walrus::ir::Const {
        value: walrus::ir::Value::I32(init),  // ← ここが問題！
    });
    0usize
}
```

### 背景: wasm-opt lowering の仕組み

`wasm-opt --multi-memory-lowering` により：

1. 複数メモリを単一メモリに統合
2. グローバル変数 → **メモリ内オフセット** に置き換え
3. グローバル変数の参照（`global.get`）→ **メモリロード命令** に置き換え

例：
```
元:
  (global $offset (mut i32) (i32.const 0x1000))
  (global.get $offset)
  
lowering後:
  (i32.const 0x1000)  ← グローバル変数の初期値
  (i32.load)          ← メモリから読み込み
```

### 現在の実装の誤り

```rust
// line 195-198: グローバル変数の"初期値"を取得
let init = match global.kind {
    walrus::GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I32(value))) => value,
    _ => unreachable!(),
};

// line 244-250: ALL GlobalGet を init に置き換え
*instr = walrus::ir::Instr::Const(walrus::ir::Const {
    value: walrus::ir::Value::I32(init),
});
```

**問題**: 

- `init` = グローバル変数の**初期値**（例: `0x1000`）
- しかし `global.get` の参照先は **メモリ上の現在のオフセット値**
- memory.grow() 後、オフセット値は変わる可能性がある
- ⚠️ **定数に置き換えたため、その後の更新ができない**

### メモリ破壊シナリオ

```
Time    Event
-----   -----
T0:     wasm-opt lowering
        global $offset = メモリ内オフセット（初期値: 0x1000）
        
T1:     shared_global.rs 実行
        global.get $offset → Const(0x1000) に置き換え
        
T2:     スレッド実行開始
        コード: val = global.get $offset
        実行: val = 0x1000 （定数）
        
T3:     メモリが拡張される (memory.grow)
        メモリ内オフセット値が更新される
        例: 0x1000 → 0x2000 (新しい領域)
        
T4:     コード実行継続
        コード: base_ptr = global.get $offset + delta
        実行: base_ptr = 0x1000 + delta
        期待: base_ptr = 0x2000 + delta
        
T5:     結果
        ✗ base_ptr が古いオフセットを参照
        ✗ メモリ範囲外アクセス → メモリ破壊
```

---

## なぜこの置き換えが行われているのか

### 設計意図（コメント 26-34行）

```
When a newly created thread is executed,
it will use the always-executable VFS code and memory,
which are based on an address that never changes,
and perform operations on them atomically.
Operations on Global variables are replaced,
and before memory unification,
memory.grow is modified to be an atomic operation.
```

**意図**:
- グローバル変数への参照を「atomic な操作」で置き換えたい
- しかし実装では、単に「定数に置き換え」されている

### 問題の本質

このコードは、グローバル変数を「不変な初期値」として扱っていますが、**wasm-opt lowering によるグローバル変数は mutable であり、memory.grow() で更新される可能性がある** ということを見落としています。

---

## 正確な根本原因まとめ

| 項目 | 詳細 |
|------|------|
| **バグ名** | GlobalGet を init 定数に置き換える誤り |
| **ファイル** | `wasi_virt_layer-cli/src/generator/shared_global.rs` |
| **行番号** | 244-250（GlobalGet 置き換え）+ 195-198（init 取得） |
| **原因** | wasm-opt lowering による mutable グローバル変数を、不変な定数として扱っている |
| **影響** | memory.grow() 後にオフセット値が更新されず、メモリ範囲外アクセスが発生 |
| **メモリ破壊の種類** | Out of Bounds アクセス、データ破壊、Segmentation Fault |

---

## 修正案

### 案1: GlobalGet を削除しない（推奨）

定数置き換えの代わりに、グローバル変数参照を保持し、各スレッド開始時に更新する：

```rust
// 現在（誤り）:
*instr = walrus::ir::Instr::Const(walrus::ir::Const {
    value: walrus::ir::Value::I32(init),
});

// 修正案:
// global.get を保持 or atomic load に置き換える
// グローバル変数の更新メカニズムを実装
```

### 案2: Atomic な メモリロード に置き換え

```rust
// 定数ではなく、メモリから毎回読み込み
*instr = walrus::ir::Instr::Load(walrus::ir::Load {
    kind: walrus::ir::LoadKind::I32_8U,  // or appropriate kind
    arg: MemArg { ... },
});
```

---

## 検証ポイント

ユーザーの指摘：
- ✅ "オフセットを用いるのはwasm-optのloweringです" → ここの話
- ✅ "そのグローバル変数を共有メモリに置き換えるコードがあり、そのあたりに詳細をメモしたはずです" → 正確にはこの置き換えが誤っている
- ✅ "memory.growではメモリアクセスに対するオフセットを更新します" → しかし定数置き換えのせいで更新が反映されない

---

## 結論

**根本的なメモリ破壊バグ**: 

wasm-opt lowering によって生成されたグローバル変数（メモリ内オフセット）が、`shared_global.rs` で不正に **不変な定数** に置き換えられている。その結果、memory.grow() 後にオフセット値が更新されてもコードから参照できず、古いアドレスでメモリアクセスが続き、メモリ破壊が発生する。

**修正難度**: 中程度（グローバル変数参照の保持メカニズムを実装する必要がある）
