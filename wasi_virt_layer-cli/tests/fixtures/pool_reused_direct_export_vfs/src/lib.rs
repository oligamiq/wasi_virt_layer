use const_struct::const_struct;
use std::sync::atomic::{AtomicU32, Ordering};
use wasi_virt_layer::{
    file::*,
    plug_thread,
    poll::*,
    prelude::*,
    thread::{ThreadRunner, VirtualThread, VirtualThreadPool},
};

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(pool_reused_direct_export_target);

static THREAD_POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(1) };
static DIRECT_EXPORT_CALLS: AtomicU32 = AtomicU32::new(0);
static FIRST_WORKER_ID: AtomicU32 = AtomicU32::new(0);
static SECOND_WORKER_ID: AtomicU32 = AtomicU32::new(0);

fn current_thread_debug_id() -> u32 {
    format!("{:?}", std::thread::current().id())
        .trim_start_matches("ThreadId(")
        .trim_end_matches(')')
        .parse()
        .unwrap_or(0)
}

#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn reused_worker_call_direct_export() {
    pool_reused_direct_export_target::_start();
    pool_reused_direct_export_target::_main();
    let worker_id = current_thread_debug_id();
    match DIRECT_EXPORT_CALLS.fetch_add(1, Ordering::SeqCst) + 1 {
        1 => FIRST_WORKER_ID.store(worker_id, Ordering::SeqCst),
        2 => SECOND_WORKER_ID.store(worker_id, Ordering::SeqCst),
        _ => {}
    }
}

fn run_direct_export_on_pool() {
    let boxed: Box<dyn FnOnce()> = Box::new(|| {});
    let runner = ThreadRunner::__new(Box::into_raw(Box::new(boxed)));
    THREAD_POOL
        .new_thread(ThreadAccessor::pool_reused_direct_export_target, runner)
        .expect("VTP should allocate a thread id for direct-export test");
}

fn wait_for_call_count(expected_count: u32) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while DIRECT_EXPORT_CALLS.load(Ordering::SeqCst) < expected_count {
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for reused VTP direct-export call {expected_count}"
        );
        std::thread::yield_now();
    }
}

impl Guest for ComponentABI {
    fn main() {
        unsafe { THREAD_POOL.init_with_capacity_and_wait(1) };
        println!("Starting reused VTP direct-export test");

        DIRECT_EXPORT_CALLS.store(0, Ordering::SeqCst);
        FIRST_WORKER_ID.store(0, Ordering::SeqCst);
        SECOND_WORKER_ID.store(0, Ordering::SeqCst);
        run_direct_export_on_pool();
        wait_for_call_count(1);
        run_direct_export_on_pool();
        wait_for_call_count(2);

        let first_worker = FIRST_WORKER_ID.load(Ordering::SeqCst);
        let second_worker = SECOND_WORKER_ID.load(Ordering::SeqCst);
        assert_ne!(
            first_worker, 0,
            "first direct export did not record a worker id"
        );
        assert_eq!(
            first_worker, second_worker,
            "direct exports should run on the same reused VTP worker"
        );

        println!("Reused VTP direct-export test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!({ &THREAD_POOL }, pool_reused_direct_export_target, self);
plug_poll!(DefaultWaitPoll, pool_reused_direct_export_target);

wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    pool_reused_direct_export_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        pool_reused_direct_export_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };
    plug_env!(@embedded, HostEnvTy, pool_reused_direct_export_target, self);
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

    plug_fs!(&VIRTUAL_FILE_SYSTEM, pool_reused_direct_export_target, self);
}
