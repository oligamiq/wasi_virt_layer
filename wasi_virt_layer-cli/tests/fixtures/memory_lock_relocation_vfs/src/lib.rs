use const_struct::const_struct;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use wasi_virt_layer::{file::*, poll::*, prelude::*, thread::VirtualThreadPool};

struct ComponentABI;
wit_bindgen::generate!({ world: "component-abi" });
import_wasm!(memory_lock_relocation_target);

static POOL: VirtualThreadPool<ThreadAccessor> = unsafe { VirtualThreadPool::new_const(2) };
static STOP: AtomicBool = AtomicBool::new(false);
static IN_GUEST: AtomicBool = AtomicBool::new(false);
static CHECKS: AtomicUsize = AtomicUsize::new(0);

impl Guest for ComponentABI {
    fn main() {
        println!(
            "waiter-aware={}",
            cfg!(feature = "waiter-aware-memory-lock")
        );
        assert_ne!(memory_reserve::<__self>(1024), -1);
        assert_ne!(memory_reserve::<memory_lock_relocation_target>(16), -1);
        memory_lock_relocation_target::_reset();
        memory_lock_relocation_target::_start();
        unsafe { POOL.init_with_capacity_and_wait(2) };
        let reader = std::thread::spawn(|| {
            while !STOP.load(Ordering::Acquire) {
                IN_GUEST.store(true, Ordering::Release);
                let status = memory_lock_relocation_target::_main();
                IN_GUEST.store(false, Ordering::Release);
                assert_eq!(status.raw(), 0, "guest sentinel trap");
                CHECKS.fetch_add(1, Ordering::Release);
            }
        });
        for _ in 0..32 {
            let checks = CHECKS.load(Ordering::Acquire);
            while CHECKS.load(Ordering::Acquire) == checks || !IN_GUEST.load(Ordering::Acquire) {
                std::thread::yield_now();
            }
            let before = __wasip1_vfs_memory_lock_relocation_target_memory_grow_global_alt_get();
            assert_ne!(memory_reserve::<__self>(1), -1);
            let after = __wasip1_vfs_memory_lock_relocation_target_memory_grow_global_alt_get();
            assert_eq!(after - before, 65536, "guest base must physically move");
        }
        STOP.store(true, Ordering::Release);
        reader.join().unwrap();
        assert!(CHECKS.load(Ordering::Acquire) >= 32);
        println!("physical-relocations=32");
        println!("relocation sentinel checks passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);
wasi_virt_layer::plug_thread!({ &POOL }, memory_lock_relocation_target, self);
wasi_virt_layer::plug_poll!(DefaultWaitPoll, memory_lock_relocation_target);
wasi_virt_layer::plug_clock!(
    wasi_virt_layer::clock::StandardClock,
    memory_lock_relocation_target,
    self
);

mod process {
    use super::*;
    plug_process!(
        wasi_virt_layer::process::StandardProcess,
        memory_lock_relocation_target,
        self
    );
}

mod env {
    use super::*;
    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=/"],
    };
    plug_env!(@embedded, HostEnvTy, memory_lock_relocation_target, self);
}

mod fs {
    use super::*;
    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, 2> =
        EmbeddedFiles!([("/", [("dummy.txt", WasiEmbeddedFile::new("dummy"))])]);
    type Lfs =
        StandardEmbeddedNormalLFS<EmbeddedFilesTy, WasiEmbeddedFile<&'static str>, 2, DefaultStdIO>;
    static FILE_SYSTEM: StandardEmbeddedFileSystem<Lfs, 2> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());
    plug_fs!(&FILE_SYSTEM, memory_lock_relocation_target, self);
}

wasi_virt_layer::own_memory!(memory_lock_relocation_target);
