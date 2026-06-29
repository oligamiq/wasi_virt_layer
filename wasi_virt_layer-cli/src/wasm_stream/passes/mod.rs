#![allow(unused_variables, unused_assignments, unreachable_patterns, dead_code)]
pub mod abi_connect;
pub mod anonymous;
pub mod check;
pub mod check_unused_threads;
pub mod dummy_injector;
pub mod multi_memory_lowering;
pub mod patch_component;
pub mod post_combine;
pub mod pre_vfs_memory_refuge;
pub mod producer;
pub mod shared_global;
pub mod special_func;
pub mod starts_pre;
pub mod threads_spawn;
pub mod wrap_unreachable;

pub mod atomic_patch;
pub mod component_ctx;
pub mod deadlock_thread_id;
pub mod export_stack;
pub mod export_stack_arena;
pub mod export_stack_multi_memory_target;
pub mod extract_mem_sizes;
pub mod fn_in_starts;
pub mod memory_post_components;

#[cfg(test)]
mod deadlock_thread_id_tests {
    use super::deadlock_thread_id::DeadlockThreadIdPreTargetStreamPass;
    use crate::wasm_stream::pipeline::StreamPass;

    fn has_custom_section(bytes: &[u8], name: &str) -> bool {
        wasmparser::Parser::new(0).parse_all(bytes).any(|payload| {
            matches!(payload, Ok(wasmparser::Payload::CustomSection(section)) if section.name() == name)
        })
    }

    fn has_mutable_i32_global(bytes: &[u8]) -> bool {
        wasmparser::Parser::new(0).parse_all(bytes).any(|payload| {
            if let Ok(wasmparser::Payload::GlobalSection(section)) = payload {
                section.into_iter().any(|global| {
                    global.is_ok_and(|global| {
                        global.ty.mutable && global.ty.content_type == wasmparser::ValType::I32
                    })
                })
            } else {
                false
            }
        })
    }

    fn export_index(bytes: &[u8], name: &str) -> Option<u32> {
        for payload in wasmparser::Parser::new(0).parse_all(bytes) {
            if let Ok(wasmparser::Payload::ExportSection(section)) = payload {
                for export in section.into_iter().flatten() {
                    if export.name == name && export.kind == wasmparser::ExternalKind::Func {
                        return Some(export.index);
                    }
                }
            }
        }
        None
    }

    #[test]
    fn deadlock_thread_id_pass_injects_global_and_rebinds_thread_start_export() {
        let input = wat::parse_str(
            r#"
            (module
              (func $wasi_thread_start (param i32 i32))
              (func $_start)
              (func $__main_void (result i32) (i32.const 0))
              (export "wasi_thread_start" (func $wasi_thread_start))
              (export "_start" (func $_start))
              (export "__main_void" (func $__main_void)))
            "#,
        )
        .unwrap();

        let original_thread_start = export_index(&input, "wasi_thread_start").unwrap();
        let output = DeadlockThreadIdPreTargetStreamPass::new(true)
            .run(&input)
            .unwrap();

        assert!(has_custom_section(&output, "wvl.deadlock_thread_id.v1"));
        assert!(has_mutable_i32_global(&output));
        assert_ne!(
            export_index(&output, "wasi_thread_start"),
            Some(original_thread_start)
        );
        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&output)
            .unwrap();
    }
}
