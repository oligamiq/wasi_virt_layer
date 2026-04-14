use const_struct::const_struct;
use wasi_virt_layer::{
    file::{VFSConstNormalFiles, WasiConstFile},
    poll::PollOneoff,
    prelude::*,
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
        test_pool_thread::_main();
    }
}

export!(Starter);

import_wasm!(test_pool_thread);

const FILE_COUNT: usize = 10;

type F = WasiConstFile<&'static str>;
type NormalFILES = VFSConstNormalFiles<F, { FILE_COUNT }>;

#[const_struct]
const FILES: NormalFILES = ConstFiles!([
    ("/root", [("root.txt", WasiConstFile::new("This is root"))]),
    (
        ".",
        [
            ("hey", WasiConstFile::new("Hey!")),
            (
                "hello",
                [
                    ("world", WasiConstFile::new("Hello, world!")),
                    ("everyone", WasiConstFile::new("Hello, everyone!")),
                ]
            )
        ]
    ),
    (
        "~",
        [
            ("home", WasiConstFile::new("This is home")),
            ("user", WasiConstFile::new("This is user")),
        ]
    )
]);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new() },
    self,
    test_pool_thread
);
plug_process!(
    wasi_virt_layer::process::DefaultProcess,
    test_pool_thread,
    self
);
#[const_struct]
const ENV: VirtualEnvConstState = VirtualEnvConstState {
    environ: &["HOME=~/"],
};
plug_env!(@const, EnvTy, test_pool_thread, self);

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
    use wasi_virt_layer::file::{DefaultStdIO, VFSConstNormalLFS, Wasip1ConstVFS};

    use super::*;

    type LFS = VFSConstNormalLFS<FilesTy, F, FILE_COUNT, DefaultStdIO>;

    static VIRTUAL_FILE_SYSTEM: Wasip1ConstVFS<LFS, FILE_COUNT> =
        Wasip1ConstVFS::new(VFSConstNormalLFS::new());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, test_pool_thread, self);
}
