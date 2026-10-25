pub mod utils;
use std::time::Duration;
use utils::*;

// No guest waits or escaped pointers: every sentinel load is a lowered guest
// access protected by the relocation lock. VFS threads do the coordination.
const TARGET: &str = r#"
(module
  (memory (export "memory") 1 16)
  (data (i32.const 1024) "\78\56\34\12\87\a9\cb\ed")
  (func (export "_start"))
  ;; VFS pool routes include this ABI even though this guest spawns no threads.
  (func (export "wasi_thread_start") (param i32 i32) unreachable)
  (func (export "__main_void") (result i32) (local $i i32)
    (loop $check
      (if (i32.ne (i32.load (i32.const 1024)) (i32.const 0x12345678))
        (then unreachable))
      (if (i32.ne (i32.load (i32.const 1028)) (i32.const 0xedcba987))
        (then unreachable))
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br_if $check (i32.lt_u (local.get $i) (i32.const 8192))))
    (i32.const 0)))
"#;

#[test]
fn concurrent_guest_loads_survive_physical_relocation_in_both_modes() -> color_eyre::Result<()> {
    color_eyre::install().ok();
    assert!(
        has_required_wasi_targets(true),
        "relocation gate requires WASI targets"
    );
    let temporary = tempfile::tempdir()?;
    let target = temporary.path().join("memory_lock_relocation_target.wasm");
    std::fs::write(&target, wat::parse_str(TARGET)?)?;
    for candidate in [false, true] {
        let mut args = vec!["--own-memory", "--validate"];
        if candidate {
            args.extend(["--features", "waiter-aware-memory-lock"]);
        }
        // Features apply to the preceding target, so put the VFS feature before
        // the positional guest. The helper's `wasm` argument would put it first.
        args.push(target.to_str().unwrap());
        let dir = run_wasi_virt_layer(
            Some("memory_lock_relocation_vfs"),
            None,
            Some(true),
            true,
            OutDir::Random,
            true,
            &args,
            Some(Duration::from_secs(60)),
        )?;
        let output = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))?;
        assert!(
            output.contains(&format!("waiter-aware={candidate}")),
            "{output}"
        );
        assert!(output.contains("physical-relocations=32"), "{output}");
        assert!(
            output.contains("relocation sentinel checks passed"),
            "{output}"
        );
        assert_release_shape(
            &std::fs::read(dir.0.join("memory_lock_relocation_vfs.core.wasm"))?,
            candidate,
        )?;
    }
    Ok(())
}

fn assert_release_shape(wasm: &[u8], candidate: bool) -> color_eyre::Result<()> {
    use wasmparser::{Operator, Parser, Payload, TypeRef};
    let mut release = None;
    let mut index = 0;
    let mut checked = false;
    for payload in Parser::new(0).parse_all(wasm) {
        match payload? {
            Payload::ImportSection(section) => {
                for group in section {
                    for import in group? {
                        if matches!(import?.1.ty, TypeRef::Func(_)) {
                            index += 1;
                        }
                    }
                }
            }
            Payload::ExportSection(section) => {
                for export in section {
                    let export = export?;
                    if export.name == "__wasip1_vfs_memory_lock_read_release" {
                        release = Some(export.index);
                    }
                }
            }
            Payload::CodeSectionEntry(body) => {
                if release == Some(index) {
                    let mut calls = 0;
                    let mut expected_constant = false;
                    let expected = if candidate { (1 << 30) | 1 } else { 1 };
                    for op in body.get_operators_reader()? {
                        match op? {
                            Operator::Call { .. } => calls += 1,
                            Operator::I32Const { value } if value == expected => {
                                expected_constant = true
                            }
                            _ => {}
                        }
                    }
                    assert!(expected_constant, "wrong generated last-reader guard");
                    assert!(calls <= 1, "unexpected hot-path forwarding calls: {calls}");
                    checked = true;
                }
                index += 1;
            }
            _ => {}
        }
    }
    assert!(checked, "read-release export/body missing");
    Ok(())
}
