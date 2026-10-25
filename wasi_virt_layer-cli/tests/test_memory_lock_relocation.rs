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
        let mut args = vec!["--own-memory", "--validate", "--no-cache"];
        if candidate {
            args.extend(["--features", "waiter-aware-memory-lock"]);
        }
        let dir = run_wasi_virt_layer(
            Some("memory_lock_relocation_vfs"),
            Some(target.to_str().unwrap()),
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
    }
    Ok(())
}
