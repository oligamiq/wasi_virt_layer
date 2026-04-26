use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_thread, prelude::*, process::*};
use std::thread;

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(<anonymous>);

impl Guest for ComponentABI {
    fn main() {
        anonymous::_reset();
        anonymous::_start();

        // Spawn multiple threads to perform concurrent file access
        let mut handles = vec![];

        for i in 0..3 {
            let handle = thread::spawn(move || {
                // Simulate some work and concurrent access
                // In a real WASI app, this would use standard file system APIs
                // which are intercepted by our VFS layer.
                println!("Thread {} accessing VFS...", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("All threads finished.");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    anonymous,
    self
);

mod process {
    use super::*;
    plug_process!(StandardProcess, anonymous, self);
}

mod env {
    use super::*;

    #[const_struct]
    const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
        environ: &["HOME=~/"],
    };

    plug_env!(@embedded, HostEnvTy, anonymous, self);
}

mod fs {
    use super::*;

    const FILE_COUNT: usize = 7;

    #[const_struct]
    const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
        EmbeddedFiles!([
            (
                "/",
                [
                    ("data1.txt", WasiEmbeddedFile::new("Data 1")),
                    ("data2.txt", WasiEmbeddedFile::new("Data 2")),
                    ("data3.txt", WasiEmbeddedFile::new("Data 3")),
                ]
            ),
            (
                "/logs",
                [
                    ("thread1.log", WasiEmbeddedFile::new("Log 1")),
                    ("thread2.log", WasiEmbeddedFile::new("Log 2")),
                ]
            )
        ]);

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, anonymous, self);
}

mod poll {

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

    use wasi_virt_layer::poll::PollOneoff;

    use super::*;
    plug_poll!(WaitPoll, anonymous, self);
}
