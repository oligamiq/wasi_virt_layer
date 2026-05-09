use std::collections::HashMap;

use eyre::Context as _;
use walrus::FunctionId;

use crate::{
    generator::Generator,
    util::{ResultUtil, WalrusFID, WalrusUtilExport, WalrusUtilModule as _, WasmName},
};

#[derive(Debug)]
/// A list of export names is displayed in the order specified. Order, etc.
/// When isolated here, it is automatically removed from module._start
/// All initialization-related items are listed here.
/// Last, we will combine them to single _start function.
/// Only put one item per member variable.
pub struct FnInStarts {
    /// コンパイルで出来上がったVFSモジュールが持っていた_start
    /// この関数は一回限りの呼び出しではない
    /// スレッドmodule作成時に呼び出される前提となっている
    pub flesh_vfs_start: String,

    /// スレッド用初期化関数。何故かデフォルトでVFSの_startに入っていないので、こちらでパッチする
    /// この関数の呼び出しは一回きりにする
    pub thread_patch: String,

    /// メモリLoweringで使われているオフセット用グローバル変数は共有メモリでは使えない。
    /// なぜなら、共有メモリではグローバル変数は共有されないからである。
    /// そのため、オフセット用グローバル変数へのアクセスのすべてをVFSに確保した共有メモリへのアクセスに置き換える。
    /// ただし、共有メモリは初期値を持てない。そこで、オフセット用グローバル変数の初期値を設定する必要がある。
    /// この関数の呼び出しは一回きりにする
    /// そのための関数。これは VFSのすべての準備が終わったあとかつ、targetの_startが呼ばれる前に呼び出される必要がある。
    pub init_offset_global: String,

    /// 初回起動時にターゲットモジュールのメモリを保存する。リセット関数のために必要
    /// Wasmには初期化時に静的に書き込まれるメモリが存在する。そこへアクセスしてメモリ情報をコピーして保存する必要がある。
    /// この関数は、オフセット変数の初期化が終わってから呼び出される必要がある。
    /// この関数の呼び出しは一回きりにする
    /// なお、リセット関数はこれを元に修正したのち、該当のflesh_target_startを呼び出す必要がある。
    pub save_target_memory: String,

    /// ターゲットモジュールが持っていた_start
    /// この関数は一回限りの呼び出しではない
    pub flesh_target_start: HashMap<WasmName, String>,

    /// _startの最後の最後に呼び出される関数
    /// _start中はimportした関数を呼べないため、デバッグログの出力などを抑える必要がある。
    /// そのための判断に使う値を切り替える関数。
    /// この関数は一回限りの呼び出しではない
    /// なぜなら、スレッドごとに固有の値を持つ必要があるからだ。
    /// 現在の実装は共有メモリを用いて一回限りで保持しているが、将来的にはグローバル変数に移行すべきだろう
    /// todo!();
    pub simple_debug_wasip1_vfs_pre_init: String,
}

impl FnInStarts {
    pub fn new(wasms: &[WasmName]) -> Self {
        let flesh_vfs_start = "__flesh_vfs_start".to_string();
        let thread_patch = "__thread_patch".to_string();
        let init_offset_global = "__init_offset_global".to_string();
        let save_target_memory = "__save_target_memory".to_string();
        let simple_debug_wasip1_vfs_pre_init = "__simple_debug_wasip1_vfs_pre_init".to_string();

        let flesh_target_start = wasms
            .iter()
            .map(|name| (name.clone(), format!("__flesh_{name}_start")))
            .collect();

        Self {
            flesh_vfs_start,
            thread_patch,
            init_offset_global,
            save_target_memory,
            flesh_target_start,
            simple_debug_wasip1_vfs_pre_init,
        }
    }

    fn init(&self, module: &mut walrus::Module, _: &super::GeneratorCtx) -> eyre::Result<()> {
        // add empty functions for all the start functions to be used later
        let mut empty_func = |name: &str| -> eyre::Result<()> {
            let fid = Self::create_dummy_start(module, name)?;
            module.exports.add(name, fid);
            Ok(())
        };

        empty_func(&self.thread_patch)?;
        empty_func(&self.init_offset_global)?;
        empty_func(&self.save_target_memory)?;
        empty_func(&self.simple_debug_wasip1_vfs_pre_init)?;

        Ok(())
    }

    fn build(&self, module: &mut walrus::Module, ctx: &super::GeneratorCtx) -> eyre::Result<()> {
        let empty_start = module.add_func(&[], &[], |_, _| Ok(()))?;
        // let inner_start = module.add_func(&[], &[], |_, _| Ok(()))?;

        // let f = module.funcs.get_mut(empty_start);
        // let builder = f.kind.unwrap_local_mut().builder_mut();
        // let mut body = builder.func_body();
        // let global_id = module.globals.add_local(walrus::ValType::I32, true, false, walrus::ConstExpr::Value(walrus::ir::Value::I32(0)));
        // body
        // .global_get(global_id)
        // .unop(walrus::ir::UnaryOp::I32Eqz)
        // .if_else(
        //     None,
        //     |then| {
        //         then
        //         .i32_const(1)
        //         .global_set(global_id)
        //         .call(inner_start);
        //     },
        //      |_| {},
        // );

        let mut adder = |name: &str| -> eyre::Result<()> {
            let dummy_name = format!("__{name}_dummy_holder");
            if let Some(dummy_fid) = ("__dummy", &dummy_name).get_fid(&module.imports).ok() {
                module
                    .replace_imported_func(dummy_fid, |(_, _)| return)
                    .to_eyre()
                    .wrap_err_with(|| format!("Failed to replace dummy import for {name}"))?;
            }

            let fid = name.get_fid(&module.exports)?;
            // let f = module.funcs.get_mut(inner_start);
            let f = module.funcs.get_mut(empty_start);
            let builder = f.kind.unwrap_local_mut().builder_mut();
            let mut body = builder.func_body();

            body.call(fid);

            module.exports.erase_with(name, ctx.unstable_print_debug)?;

            Ok(())
        };
        adder(&self.flesh_vfs_start)?;
        adder(&self.thread_patch)?;
        adder(&self.init_offset_global)?;
        adder(&self.save_target_memory)?;
        for name in self.flesh_target_start.values() {
            adder(name)?;
        }
        adder(&self.simple_debug_wasip1_vfs_pre_init)?;

        module.start = Some(empty_start);

        Ok(())
    }

    fn create_dummy_start(
        module: &mut walrus::Module,
        name: impl AsRef<str>,
    ) -> eyre::Result<FunctionId> {
        // 中身が空だと統合される可能性がある
        // よって関数をimportする
        let name = name.as_ref();
        let import_name = format!("__{name}_dummy_holder");
        let type_id = module.types.add(&[], &[]);
        let (fid, _) = module.add_import_func("__dummy", &import_name, type_id);
        let empty_start = module.add_func(&[], &[], |builder, _| {
            builder.func_body().call(fid);
            Ok(())
        })?;
        Ok(empty_start)
    }
}

#[derive(Debug, Default)]
pub struct FnInStartsGeneratorFirst;

#[derive(Debug, Default)]
pub struct FnInStartsGeneratorLast;

impl Generator for FnInStartsGeneratorFirst {
    fn pre_vfs(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
    ) -> eyre::Result<()> {
        // remove _start from target module and save its name
        let start_fid = if let Some(fid) = module.start.take() {
            fid
        } else {
            FnInStarts::create_dummy_start(module, &ctx.starts.flesh_vfs_start)?
        };

        module.exports.add(&ctx.starts.flesh_vfs_start, start_fid);

        ctx.starts.init(module, ctx)?;

        Ok(())
    }

    fn pre_target(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
        external: &super::ModuleExternal,
    ) -> eyre::Result<()> {
        let start_fid = if let Some(fid) = module.start.take() {
            fid
        } else {
            FnInStarts::create_dummy_start(module, &ctx.starts.flesh_target_start[&external.name])?
        };

        module
            .exports
            .add(&ctx.starts.flesh_target_start[&external.name], start_fid);
        Ok(())
    }
}

impl Generator for FnInStartsGeneratorLast {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
    ) -> eyre::Result<()> {
        if ctx.target_memory_type.is_single() {
            return Ok(());
        }

        ctx.starts.build(module, ctx)
    }

    fn post_lower_memory(
        &mut self,
        module: &mut walrus::Module,
        ctx: &super::GeneratorCtx,
    ) -> eyre::Result<()> {
        ctx.starts.build(module, ctx)
    }
}
