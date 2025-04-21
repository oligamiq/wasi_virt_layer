use std::fs;

use camino::Utf8PathBuf;
use walrus::{
    LocalId,
    ir::{MemArg, StoreKind, Value},
};

pub fn adjust_merged_wasm(path: &Utf8PathBuf) -> anyhow::Result<Utf8PathBuf> {
    let mut module = walrus::Module::from_file(path)?;

    // module
    //     .imports
    //     .remove("$root", "[static]wasip1.fd-write-import")
    //     .expect("fd_write_import not found");

    let memory_id = module
        .memories
        .iter()
        .next()
        .expect("Memory not found")
        .id();

    let import_id = module
        .imports
        .find("$root", "[static]wasip1.environ-sizes-get-import")
        .expect("environ_sizes_get_import not found");

    let fid = module
        .funcs
        .iter()
        .find(|f| {
            if let walrus::FunctionKind::Import(imported_function) = &f.kind {
                imported_function.import == import_id
            } else {
                false
            }
        })
        .expect("environ_sizes_get_import not found")
        .id();

    module
        .replace_imported_func(fid, |(body, arg_locals)| {
            // #[unsafe(no_mangle)]
            // pub unsafe extern "C" fn environ_sizes_get(
            //     environ_count: *mut wasip1::Size,
            //     environ_buf: *mut wasip1::Size,
            // ) -> wasip1::Errno {
            //     unsafe { *environ_count = 0 };
            //     unsafe { *environ_buf = 0 };
            //     ERRNO_SUCCESS
            // }

            // (func $environ_sizes_get (;0;) (type 0) (param i32 i32) (result i32)
            //     local.get 0
            //     i32.const 0
            //     i32.store
            //     local.get 1
            //     i32.const 0
            //     i32.store
            //     i32.const 0
            // )

            body.local_get(arg_locals[0])
                .const_(Value::I32(0))
                .store(
                    memory_id,
                    StoreKind::I32 { atomic: false },
                    MemArg {
                        align: 0,
                        offset: 0,
                    },
                )
                .local_get(arg_locals[1])
                .const_(Value::I32(0))
                .store(
                    memory_id,
                    StoreKind::I32 { atomic: false },
                    MemArg {
                        align: 0,
                        offset: 0,
                    },
                )
                .const_(Value::I32(0))
                .return_();
        })
        .expect("Failed to replace fd_write_import");

    let new_path = path.with_extension("adjusted.wasm");

    if fs::metadata(&new_path).is_ok() {
        fs::remove_file(&new_path).expect("Failed to remove existing file");
    }
    module.emit_wasm_file(new_path.clone())?;

    Ok(new_path)
}
