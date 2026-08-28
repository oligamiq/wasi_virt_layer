use const_struct::const_struct;
use wasi_virt_layer::{
    file::*, plug_thread, poll::*, prelude::*, process::*, thread::VirtualThreadPool,
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(pool_own_mem_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(5) };

impl Guest for ComponentABI {
    fn main() {
        pool_own_mem_target::_reset();

        let self_reserve = crate::memory_reserve::<__self>(1024);
        println!("self reserve = {self_reserve}");

        let target_reserve = crate::memory_reserve::<pool_own_mem_target>(1024);
        println!("target reserve = {target_reserve}");

        assert_ne!(self_reserve, -1);
        assert_ne!(target_reserve, -1);

        unsafe { THREAD_POOL.init_with_capacity_and_wait(5) };
        println!("Pool threads initialized.");
        pool_own_mem_target::_start();
        pool_own_mem_target::_main();
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ &THREAD_POOL }, pool_own_mem_target, self);
plug_poll!(DefaultWaitPoll, pool_own_mem_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    pool_own_mem_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        pool_own_mem_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    plug_env!(@embedded, HostEnvTy, pool_own_mem_target, self);
}

mod fs {
    use super::*;
    const FILE_COUNT: usize = 2;
    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
        EmbeddedFiles!([("/", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, pool_own_mem_target, self);
}

wasi_virt_layer::own_memory!(pool_own_mem_target);
