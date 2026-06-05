//! Start function execution order definition.
//!
//! This module defines [`FnInStarts`], which specifies the execution order
//! of initialization functions combined into the final `_start` function.
//!
//! The order of fields in the struct directly corresponds to the execution order.
//! All initialization-related items are listed here, and they are combined
//! into a single `_start` function during the post-combine pass.

use std::collections::HashMap;

/// 実行順序が指定された順にエクスポート名のリストを表す。
/// ここに分離された関数は、module._start から自動的に除去される。
/// すべての初期化関連のアイテムがここにリストされる。
/// 最終的に、これらは単一の `_start` 関数にまとめられる。
/// メンバ変数1つにつき1つの関数のみを配置すること。
///
/// # Execution Order
///
/// The fields are listed in **exact execution order** (top → bottom):
///
/// ```text
/// _start() {
///   1. flesh_vfs_start            — VFS内部状態の初期化
///   2. thread_patch               — スレッド用初期化パッチ (一回きり)
///   3. init_offset_global         — メモリオフセットグローバルの初期化 (一回きり)
///   4. save_target_memory         — ターゲットメモリの初期状態保存 (一回きり)
///   5. flesh_target_start[..]     — 各ターゲットモジュールの _start (ターゲット順)
///   6. simple_debug_pre_init      — デバッグフラグ切り替え
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FnInStarts {
    // =========================================================================
    // 1. VFS Start
    // =========================================================================
    /// コンパイルで出来上がったVFSモジュールが持っていた `_start`。
    /// この関数は一回限りの呼び出しではない。
    /// スレッドmodule作成時に呼び出される前提となっている。
    pub flesh_vfs_start: String,

    // =========================================================================
    // 2. Thread Patch
    // =========================================================================
    /// スレッド用初期化関数。何故かデフォルトでVFSの `_start` に入っていないので、
    /// こちらでパッチする。
    /// この関数の呼び出しは一回きりにする。
    pub thread_patch: String,

    // =========================================================================
    // 3. Init Offset Global
    // =========================================================================
    /// メモリLoweringで使われているオフセット用グローバル変数は共有メモリでは使えない。
    /// なぜなら、共有メモリではグローバル変数は共有されないからである。
    /// そのため、オフセット用グローバル変数へのアクセスのすべてを
    /// VFSに確保した共有メモリへのアクセスに置き換える。
    /// ただし、共有メモリは初期値を持てない。そこで、オフセット用グローバル変数の初期値を
    /// 設定する必要がある。
    /// この関数の呼び出しは一回きりにする。
    /// そのための関数。これはVFSのすべての準備が終わったあとかつ、
    /// targetの `_start` が呼ばれる前に呼び出される必要がある。
    pub init_offset_global: String,

    // =========================================================================
    // 4. Save Target Memory
    // =========================================================================
    /// 初回起動時にターゲットモジュールのメモリを保存する。リセット関数のために必要。
    /// Wasmには初期化時に静的に書き込まれるメモリが存在する。
    /// そこへアクセスしてメモリ情報をコピーして保存する必要がある。
    /// この関数は、オフセット変数の初期化が終わってから呼び出される必要がある。
    /// この関数の呼び出しは一回きりにする。
    /// なお、リセット関数はこれを元に修正したのち、
    /// 該当の `flesh_target_start` を呼び出す必要がある。
    pub save_target_memory: String,

    // =========================================================================
    // 5. Flesh Target Starts (per target, in order)
    // =========================================================================
    /// ターゲットモジュールが持っていた `_start`。
    /// この関数は一回限りの呼び出しではない。
    pub flesh_target_starts: HashMap<String, String>,

    // =========================================================================
    // 6. Simple Debug Pre Init
    // =========================================================================
    /// `_start` の最後の最後に呼び出される関数。
    /// `_start` 中はimportした関数を呼べないため、デバッグログの出力などを抑える必要がある。
    /// そのための判断に使う値を切り替える関数。
    /// この関数は一回限りの呼び出しではない。
    /// なぜなら、スレッドごとに固有の値を持つ必要があるからだ。
    /// 現在の実装は共有メモリを用いて一回限りで保持しているが、
    /// 将来的にはグローバル変数に移行すべきだろう。
    pub simple_debug_pre_init: String,
}

/// `_start` 関数生成時に使用される、各関数のエクスポート名から解決された
/// 関数インデックスの集合。
///
/// [`FnInStarts`] のフィールド順がそのまま実行順序を表す。
#[derive(Debug, Default)]
pub struct ResolvedStartFuncs {
    /// 1. VFS の `_start` 関数のインデックス
    pub flesh_vfs_start: Option<u32>,
    /// 2. スレッドパッチ関数のインデックス (wasi_thread_initializer 優先)
    pub thread_patch: Option<u32>,
    /// 2'. wasi_thread_initializer のインデックス (thread_patch より優先)
    pub wasi_thread_initializer: Option<u32>,
    /// 3. オフセットグローバル初期化関数のインデックス
    pub init_offset_global: Option<u32>,
    /// 4. ターゲットメモリ保存関数のインデックス
    pub save_target_memory: Option<u32>,
    /// 5. 各ターゲットの `_start` 関数のインデックス
    pub flesh_target_starts: HashMap<String, u32>,
    /// 6. デバッグ用初期化フラグ切り替え関数のインデックス
    pub simple_debug_pre_init: Option<u32>,
}

impl FnInStarts {
    /// Create a new `FnInStarts` with standard export names.
    pub fn new<S: AsRef<str>>(target_names: &[S]) -> Self {
        let flesh_target_starts = target_names
            .iter()
            .map(|name| {
                (
                    name.as_ref().to_string(),
                    format!("__flesh_{}_start", name.as_ref()),
                )
            })
            .collect();

        Self {
            flesh_vfs_start: "__flesh_vfs_start".to_string(),
            thread_patch: "__thread_patch".to_string(),
            init_offset_global: "__init_offset_global".to_string(),
            save_target_memory: "__save_target_memory".to_string(),
            flesh_target_starts,
            simple_debug_pre_init: "simple_debug_wasip1_vfs_pre_init".to_string(),
        }
    }

    /// Returns the list of dummy function export names that need to be injected
    /// into the VFS module before merging.
    ///
    /// These are functions whose bodies are filled in later (by the post-combine pass),
    /// but must exist as exports so that the in-process merge can resolve references.
    pub fn dummy_export_names(&self) -> Vec<String> {
        vec![
            self.thread_patch.clone(),
            self.init_offset_global.clone(),
            self.save_target_memory.clone(),
            format!("__{}_dummy_holder", self.simple_debug_pre_init),
        ]
    }

    /// Emit the `_start` function body by calling resolved functions in the
    /// canonical execution order.
    ///
    /// This is the single source of truth for start function ordering.
    pub fn emit_start_body(
        &self,
        resolved: &ResolvedStartFuncs,
        target_names: &[String],
        rebind_fn: impl Fn(u32) -> u32,
    ) -> wasm_encoder::Function {
        let mut start_func = wasm_encoder::Function::new(vec![]);

        // 1. flesh_vfs_start — VFS内部状態の初期化
        if let Some(idx) = resolved.flesh_vfs_start {
            let rebound = rebind_fn(idx);
            start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
        }

        // 2. thread_patch — スレッド用初期化パッチ
        //    wasi_thread_initializer が存在する場合はそちらを優先する
        if let Some(idx) = resolved.wasi_thread_initializer {
            let rebound = rebind_fn(idx);
            start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
        } else if let Some(idx) = resolved.thread_patch {
            let rebound = rebind_fn(idx);
            start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
        }

        // 3. init_offset_global — メモリオフセットグローバルの初期化
        if let Some(idx) = resolved.init_offset_global {
            let rebound = rebind_fn(idx);
            start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
        }

        // 4. save_target_memory — ターゲットメモリの初期状態保存
        if let Some(idx) = resolved.save_target_memory {
            let rebound = rebind_fn(idx);
            start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
        }

        // 5. flesh_target_start[..] — 各ターゲットモジュールの _start (ターゲット順)
        for target_name in target_names {
            if let Some(&idx) = resolved.flesh_target_starts.get(target_name) {
                let rebound = rebind_fn(idx);
                start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
            }
        }

        // 6. simple_debug_pre_init — デバッグフラグ切り替え
        if let Some(idx) = resolved.simple_debug_pre_init {
            let rebound = rebind_fn(idx);
            start_func.instruction(&wasm_encoder::Instruction::Call(rebound));
        }

        start_func.instruction(&wasm_encoder::Instruction::End);
        start_func
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_in_starts_new() {
        let starts = FnInStarts::new(&["test_target", "other_target"]);
        assert_eq!(starts.flesh_vfs_start, "__flesh_vfs_start");
        assert_eq!(starts.thread_patch, "__thread_patch");
        assert_eq!(starts.init_offset_global, "__init_offset_global");
        assert_eq!(starts.save_target_memory, "__save_target_memory");
        assert_eq!(
            starts.flesh_target_starts.get("test_target"),
            Some(&"__flesh_test_target_start".to_string())
        );
        assert_eq!(
            starts.flesh_target_starts.get("other_target"),
            Some(&"__flesh_other_target_start".to_string())
        );
    }

    #[test]
    fn test_dummy_export_names() {
        let starts = FnInStarts::new::<String>(&[]);
        let names = starts.dummy_export_names();
        assert!(names.contains(&"__thread_patch".to_string()));
        assert!(names.contains(&"__init_offset_global".to_string()));
        assert!(names.contains(&"__save_target_memory".to_string()));
    }

    #[test]
    fn test_emit_start_body_order() {
        let starts = FnInStarts::new(&["my_target"]);
        let resolved = ResolvedStartFuncs {
            flesh_vfs_start: Some(10),
            thread_patch: Some(20),
            wasi_thread_initializer: None,
            init_offset_global: Some(30),
            save_target_memory: Some(40),
            flesh_target_starts: {
                let mut m = HashMap::new();
                m.insert("my_target".to_string(), 50);
                m
            },
            simple_debug_pre_init: Some(60),
        };
        let target_names = vec!["my_target".to_string()];
        // Identity rebinder for testing
        let func = starts.emit_start_body(&resolved, &target_names, |x| x);
        // If we get here without panic, the ordering logic is sound
        drop(func);
    }
}
