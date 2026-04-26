use const_struct::const_struct;
use wasi_virt_layer::{file::*, plug_process, prelude::*, process::StandardProcess};

#[unsafe(no_mangle)]
pub extern "C" fn __wasip1_vfs_self_vfs__start_anchor() {}

#[cfg(target_arch = "wasm32")]
#[unsafe(export_name = "main")]
pub extern "C" fn wasm_main() {
    // Calling the Rust main logic
    real_main();
}

fn main() {
    real_main();
}

fn real_main() {
    println!("--- Self-Virtualized Component Starting ---");

    // 1. Check environment variables (virtualized)
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    println!("Environment: USER={}", user);

    // 2. Read from the VFS (virtualized)
    println!("Reading /hello.txt...");
    match std::fs::read_to_string("/hello.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error: {}", e),
    }

    // 3. Listing directory (virtualized)
    println!("Listing /:");
    if let Ok(entries) = std::fs::read_dir("/") {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("  {}", entry.path().display());
            }
        }
    }

    println!("--- Self-Virtualized Component Finished ---");
}

// --- Virtualization Layer Definition ---

// Virtualize environment for 'self'
#[const_struct]
const HOST_ENV: VirtualEnvEmbeddedState = VirtualEnvEmbeddedState {
    environ: &["USER=self-virtualized-user"],
};
plug_env!(@embedded, HostEnvTy, self);

// Virtualize file system for 'self'
const FILE_COUNT: usize = 2;
#[const_struct]
const EMBEDDED_FILES: StandardEmbeddedFiles<WasiEmbeddedFile<&'static str>, { FILE_COUNT }> =
    EmbeddedFiles!([(
        "/",
        [(
            "hello.txt",
            WasiEmbeddedFile::new("Hello from my own virtual filesystem!")
        )]
    )]);

type LFS = StandardEmbeddedNormalLFS<
    EmbeddedFilesTy,
    WasiEmbeddedFile<&'static str>,
    FILE_COUNT,
    DefaultStdIO,
>;

static VIRTUAL_FILE_SYSTEM: StandardEmbeddedFileSystem<LFS, FILE_COUNT> =
    StandardEmbeddedFileSystem::new_const(StandardEmbeddedNormalLFS::new_const());

plug_fs!(&VIRTUAL_FILE_SYSTEM, self);

// Virtualize process and poll for 'self'
plug_process!(StandardProcess, self);
plug_poll!(self);
