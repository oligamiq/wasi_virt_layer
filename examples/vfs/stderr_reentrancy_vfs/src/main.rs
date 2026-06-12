use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_self_vfs__start_anchor() {}

#[cfg(target_arch = "wasm32")]
#[unsafe(export_name = "main")]
pub extern "C" fn wasm_main() {
    real_main();
}

fn main() {
    real_main();
}

fn real_main() {
    // Exercise stderr on several threads with the re-entrancy detector enabled.
    // A host route that synchronously enters virtual fd_write again traps instead
    // of hanging in an unbounded call cycle.
    eprintln!("--- Stderr Re-entrancy Test Starting ---");

    let mut threads = vec![];
    for i in 0..4 {
        threads.push(std::thread::spawn(move || {
            eprintln!("Hello from thread {}", i);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    eprintln!("--- Stderr Re-entrancy Test Finished ---");
}

// --- Virtualization Layer Definition ---

// We must use 'self' because the bug occurs when the target is also the VFS,
// which causes `ConnectWasip1ABIPreVfsStreamPass` to export `__wasip1_vfs_self_fd_write`
const FILE_COUNT: usize = 2;
#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([(".", [("reentrancy-check", WasiEmbeddedFile::new(""))])]);

type LFS = StandardEmbeddedNormalLFS<
    EmbeddedFilesTy,
    WasiEmbeddedFile<&'static str>,
    FILE_COUNT,
    DefaultStdIO,
>;

static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
    StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

plug_fs!(&VIRTUAL_FILE_SYSTEM, self);
plug_process!(StandardProcess, self);
plug_poll!(self);
