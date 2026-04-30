use const_struct::const_struct;
use std::thread;
use wasi_virt_layer::{file::*, plug_thread, prelude::*, process::*};

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
    use wasi_virt_layer::poll::DefaultWaitPoll;

    use super::*;
    plug_poll!(DefaultWaitPoll, anonymous, self);
}
