use crate::generator::{Generator, GeneratorCtx};
use crate::util::WalrusFID as _;

/// A generator for handling the atomic wait timeout in `WaitPoll`.
/// 
/// If `ctx.threads` is true, this generator replaces the `__wvl_poll_atomic_wait` import
/// with a function that executes the `memory.atomic.wait32` instruction.
/// If `ctx.threads` is false, it replaces the import with a dummy function that returns `2`
/// (which corresponds to timed out) immediately, allowing the fallback busy loop in `WaitPoll` to execute.
#[derive(Debug, Default)]
pub struct PollWait;

impl Generator for PollWait {
    fn post_combine(
        &mut self,
        module: &mut walrus::Module,
        ctx: &GeneratorCtx,
    ) -> eyre::Result<()> {
        if let Some(id) = ("wvl_poll", "__wvl_poll_atomic_wait").get_fid(&module.imports).ok() {
            if ctx.threads {
                let vfs_mem = ctx.vfs_used_memory_id.unwrap();
                module.replace_imported_func(id, |(builder, args)| {
                    builder.func_body()
                        .local_get(args[0])
                        .local_get(args[1])
                        .local_get(args[2])
                        .atomic_wait(vfs_mem, walrus::ir::MemArg { align: 4, offset: 0 }, false);
                }).map_err(|e| eyre::eyre!("Failed to replace __wvl_poll_atomic_wait with atomic.wait: {e}"))?;
            } else {
                module.replace_imported_func(id, |(builder, _args)| {
                    builder.func_body()
                        .i32_const(2); // 2 means timed out
                }).map_err(|e| eyre::eyre!("Failed to replace __wvl_poll_atomic_wait with dummy return: {e}"))?;
            }
        }
        Ok(())
    }
}
