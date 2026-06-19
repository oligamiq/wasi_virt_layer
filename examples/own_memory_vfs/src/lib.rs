use const_struct::const_struct;
use parking_lot::Mutex;
use std::sync::LazyLock;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

import_wasm!(big_alloc);

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");

        type F = WasiEmbeddedFile<&'static str>;
        const FILES2: StandardEmbeddedFiles<F, 5> = EmbeddedFiles!([(
            ".",
            [
                ("hey", F::new("Hey!")),
                (
                    "hello",
                    [
                        ("world", F::new("Hello, world!")),
                        ("everyone", F::new("Hello, everyone!")),
                    ],
                ),
            ],
        )]);

        println!("Files: {FILES2:?}");
    }
    fn add_env(env: String) {
        let mut state = VIRTUAL_ENV.lock();
        state.environ.push(env.clone());
        println!("Adding env: {}", env);
    }
    fn get_envs() -> Vec<String> {
        VIRTUAL_ENV.lock().get_environ().to_vec()
    }
    fn main() {
        // Expand memory for big_alloc to handle allocations
        #[cfg(feature = "manual_expand_memory")]
        {
            println!("Expanding memory by 8000 pages for big_alloc...");
            let res = crate::memory_grow::<big_alloc>(8000);
            println!("memory_grow result: {}", res as isize);
        }

        #[cfg(feature = "self_own_memory_api")]
        {
            let self_size = crate::memory_size_self();
            let self_grow = crate::memory_grow_self(0);
            let generic_self_grow = crate::memory_grow::<__self>(0);
            println!(
                "self own-memory API results: size={self_size}, grow={self_grow}, generic_grow={generic_self_grow}"
            );
        }

        big_alloc::_reset();
        big_alloc::_start();
        big_alloc::_main();
    }
}

#[cfg(not(test))]
export!(Hello);

plug_thread!(
    { wasi_virt_layer::thread::DirectThreadPool::<ThreadAccessor>::new_const() },
    self,
    big_alloc
);

plug_process!(StandardProcess, big_alloc, self);
plug_random!(StandardRandom, big_alloc, self);
wasi_virt_layer::plug_clock!(wasi_virt_layer::clock::StandardClock, big_alloc, self);

struct VirtualEnvState {
    environ: Vec<String>,
}

impl<'a> VirtualEnv<'a> for VirtualEnvState {
    type Str = String;

    fn get_environ(&mut self) -> &[Self::Str] {
        &self.environ
    }
}

static VIRTUAL_ENV: LazyLock<Mutex<VirtualEnvState>> = LazyLock::new(|| {
    let mut environ = Vec::<String>::new();
    // environ.push("RUST_MIN_STACK=16777216".into());
    environ.push("HOME=~/".into());
    environ.push("RUST_BACKTRACE=1".into());
    Mutex::new(VirtualEnvState { environ })
});

plug_env!(@dynamic, &mut VIRTUAL_ENV.lock(), big_alloc);

#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &[
        // "RUST_MIN_STACK=16777216",
        "HOME=~/",
        "RUST_BACKTRACE=1",
    ],
};

plug_env!(@embedded, HostEnvTy, self);

const FILE_COUNT: usize = 10;

#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([
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

mod fs {
    use super::*;

    type LFS = StandardEmbeddedNormalLFS<
        EmbeddedFilesTy,
        WasiEmbeddedFile<&'static str>,
        FILE_COUNT,
        DefaultStdIO,
    >;

    static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
        StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

    plug_fs!(&VIRTUAL_FILE_SYSTEM, big_alloc);
}

#[cfg(feature = "invalid_self_own_memory_arg")]
wasi_virt_layer::own_memory!(self, big_alloc);

#[cfg(not(feature = "invalid_self_own_memory_arg"))]
wasi_virt_layer::own_memory!(big_alloc);
