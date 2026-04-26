use const_struct::const_struct;
use wasi_virt_layer::{
    file::{StandardEmbeddedFiles, WasiEmbeddedFile},
    poll::PollOneoff,
    prelude::*,
    thread::VirtualThreadPool,
};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "init",
});

struct Starter;

impl Guest for Starter {
    fn main() {
        test_pool_thread::_reset();
        test_pool_thread::_start();

        Self::init(4);

        test_pool_thread::_main();
    }

    fn init(pool_size: u32) {
        println!("%%% Initializing thread pool with size {}", pool_size);

        let pool = &raw mut THREAD_POOL;
        let pool = unsafe { &mut *pool };
        pool.init();

        println!("%%% Setting thread pool capacity to {}", pool_size);

        pool.set_capacity(pool_size as usize);

        println!("%%% Flushing thread pool capacity...");
        let waiter = pool.flush_capacity();

        println!("%%% Waiting for thread pool to initialize...");

        waiter.wait();

        println!("%%% Thread pool initialized.");
    }
}

#[cfg(not(test))]
export!(Starter);

import_wasm!(test_pool_thread);

const FILE_COUNT: usize = 10;

type F = WasiEmbeddedFile<&'static str>;
type NormalFILES = StandardEmbeddedFiles<F, { FILE_COUNT }>;

#[const_struct]
const EMBEDDED_FILES: NormalFILES = EmbeddedFiles!([
    (
        "/root",
        [("root.txt", WasiEmbeddedFile::new("This is root"))]
    ),
    (
        ".",
        [
            ("hey", WasiEmbeddedFile::new("Hey!")),
            (
                "hello",
                [
                    ("world", WasiEmbeddedFile::new("Hello, world!")),
                    ("everyone", WasiEmbeddedFile::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", WasiEmbeddedFile::new("This is home")),
            ("user", WasiEmbeddedFile::new("This is user")),
        ]
    )
]);

static mut THREAD_POOL: VirtualThreadPool<ThreadAccessor> =
    unsafe { VirtualThreadPool::new_const(4) };

plug_thread!(
    { unsafe { &mut *(&raw mut THREAD_POOL) } },
    // { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    test_pool_thread
);
plug_process!(
    wasi_virt_layer::process::StandardProcess,
    test_pool_thread,
    self
);
#[const_struct]
const ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["HOME=~/"],
};
plug_env!(@embedded, EnvTy, test_pool_thread, self);

struct WaitPoll;

impl PollOneoff for WaitPoll {
    #[inline(never)]
    fn poll_oneoff<Wasm: WasmAccess>(
        subscriptions_ptr: *const wasi_virt_layer::__private::wasip1::Subscription,
        ret_event_ptr: *mut wasi_virt_layer::__private::wasip1::Event,
        nsubscriptions: wasi_virt_layer::__private::wasip1::Size,
        ret_stored_events_ptr: *mut wasi_virt_layer::__private::wasip1::Size,
    ) -> wasi_virt_layer::__private::wasip1::Errno {
        use wasi_virt_layer::__private::wasip1::*;

        if nsubscriptions == 0 {
            return ERRNO_INVAL;
        }

        // TODO: For now, we only support a single subscription just to be enough for wasi-libc's
        // clock_nanosleep.
        if nsubscriptions > 1 {
            return ERRNO_NOTSUP;
        }

        let (userdata, event_type, timeout, precision, flags) = unsafe {
            let base_ptr = subscriptions_ptr as *const u8;

            let userdata = Wasm::load_le::<u64>(base_ptr as *const u64);
            let event_type = Wasm::load_le::<u8>(base_ptr.add(8) as *const u8);
            // let clock_id = Wasm::load_le::<u32>(base_ptr.add(16) as *const u32);
            let timeout = Wasm::load_le::<Timestamp>(base_ptr.add(24) as *const Timestamp);
            let precision = Wasm::load_le::<Timestamp>(base_ptr.add(32) as *const Timestamp);
            let flags = Wasm::load_le::<Subclockflags>(base_ptr.add(40) as *const Subclockflags);

            (userdata, event_type, timeout, precision, flags)
        };

        // TODO: For now, we only support clock subscriptions.
        if event_type != EVENTTYPE_CLOCK.raw() {
            return ERRNO_NOTSUP;
        }

        fn get_now() -> Timestamp {
            let sys_time = std::time::SystemTime::now();

            let nano = sys_time
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as Timestamp;

            nano
        }

        // Perform the wait
        let end_time = if (flags & SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME) != 0 {
            timeout
        } else {
            get_now().saturating_add(timeout)
        }
        .saturating_sub(precision);

        loop {
            std::thread::yield_now();

            let now = get_now();

            // println!("### Current time: {}", now);

            if now >= end_time {
                break;
            }
        }

        // Write an event to the out buffer
        let event = Event {
            userdata: userdata,
            error: ERRNO_SUCCESS,
            type_: EVENTTYPE_CLOCK,
            fd_readwrite: EventFdReadwrite {
                nbytes: 0,
                flags: 0,
            },
        };

        Wasm::store_le(ret_event_ptr, event);
        Wasm::store_le(ret_stored_events_ptr, 1);

        ERRNO_SUCCESS
    }
}

plug_poll!(WaitPoll, test_pool_thread);

mod fs {
    use wasi_virt_layer::file::{
        DefaultStdIO, StandardEmbeddedFileSystem, StandardEmbeddedNormalLFS,
    };

    use super::*;

    type LFS = StandardEmbeddedNormalLFS<EmbeddedFilesTy, F, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_pool_thread, self);
}
