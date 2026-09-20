//! Regression coverage for folding the official reactor into WVL's core start.
pub mod utils;
use wasi_virt_layer_cli::wasm_stream::{
    passes::post_combine::PostCombineStreamPass, pipeline::StreamPass,
};

const REACTOR: &str = r#"(module
  ;; This import is rebound to a definition, shifting every defined index.
  (import "__wasip1_vfs-host" "hook" (func $hook))
  (memory (export "memory") 1 1 shared)
  (global (export "__wasip1_vfs_reactor_init_state") i32 (i32.const 16))
  (func (export "hook"))
  (func $memory_init (export "__flesh_vfs_start")
    (i32.store (i32.const 0) (i32.const 42)))
  (func $initialize (export "_initialize")
    (if (i32.atomic.rmw.cmpxchg (i32.const 4) (i32.const 0) (i32.const 1))
      (then unreachable))
    (call $ctors))
  (func $ctors
    (if (i32.ne (i32.load (i32.const 0)) (i32.const 42)) (then unreachable))
    (i32.store (i32.const 8) (i32.const 99)))
  (func (export "__init_offset_global")
    (if (i32.ne (i32.load (i32.const 8)) (i32.const 99)) (then unreachable)))
  (func (export "__save_target_memory"))
  (func (export "__flesh_target_start")
    (if (i32.ne (i32.load (i32.const 8)) (i32.const 99)) (then unreachable)))
  (func (export "simple_debug_wasip1_vfs_pre_init")))"#;

fn fold(wat: &str, threads: bool) -> eyre::Result<Vec<u8>> {
    let input = wasi_virt_layer_cli::wasm_stream::passes::starts_pre::StartsPreStreamPass::new(
        true,
        true,
        "__flesh_vfs_start".into(),
    )
    .with_reactor_folding(threads)
    .run(&wat::parse_str(wat)?)?;
    PostCombineStreamPass::new("vfs".into(), vec!["target".into()], vec![8], threads).run(&input)
}

fn exports(wasm: &[u8]) -> eyre::Result<Vec<String>> {
    let mut names = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        if let wasmparser::Payload::ExportSection(s) = payload? {
            for export in s {
                names.push(export?.name.to_owned());
            }
        }
    }
    Ok(names)
}

#[test]
fn reactor_is_called_in_the_v061_initializer_slot() -> eyre::Result<()> {
    let output = fold(REACTOR, true)?;
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(&output)?;
    assert!(!exports(&output)?.iter().any(|s| s == "_initialize"));
    let mut bodies = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(&output) {
        if let wasmparser::Payload::CodeSectionEntry(body) = payload? {
            bodies.push(body);
        }
    }
    let calls = bodies
        .last()
        .unwrap()
        .get_operators_reader()?
        .into_iter()
        .filter_map(|op| match op {
            Ok(wasmparser::Operator::Call { function_index }) => Some(function_index),
            _ => None,
        })
        .collect::<Vec<_>>();
    // Same six slots as v0.6.1; the second now calls official _initialize.
    assert_eq!(calls, [1, 2, 4, 5, 6, 7]);
    // Preserve the initializer's call to the nonempty constructor, not an inline copy.
    assert!(
        bodies[2]
            .get_operators_reader()?
            .into_iter()
            .any(|op| matches!(op, Ok(wasmparser::Operator::Call { function_index: 3 })))
    );
    Ok(())
}

#[test]
fn reactor_component_and_js_have_only_one_core_module() -> eyre::Result<()> {
    let output = fold(REACTOR, true)?;
    let component = wit_component::ComponentEncoder::default()
        .validate(true)
        .module(&output)
        .map_err(|e| eyre::eyre!(e))?
        .encode()
        .map_err(|e| eyre::eyre!(e))?;
    let modules = wasmparser::Parser::new(0)
        .parse_all(&component)
        .filter(|p| matches!(p, Ok(wasmparser::Payload::ModuleSection { .. })))
        .count();
    assert_eq!(modules, 1);
    let transpiled = js_component_bindgen::transpile(
        &component,
        js_component_bindgen::TranspileOpts {
            name: "vfs".into(),
            base64_cutoff: 0,
            no_typescript: true,
            instantiation_mode: Some(js_component_bindgen::InstantiationMode::Async),
            ..Default::default()
        },
    )
    .map_err(|e| eyre::eyre!(e))?;
    let names = transpiled
        .files
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names
            .iter()
            .filter(|n| n.ends_with(".wasm"))
            .copied()
            .collect::<Vec<_>>(),
        ["vfs.core.wasm"]
    );
    let js = String::from_utf8(
        transpiled
            .files
            .iter()
            .find(|(n, _)| n == "vfs.js")
            .unwrap()
            .1
            .clone(),
    )?;
    assert!(!js.contains("module1"));
    assert!(!js.contains("core2.wasm"));
    assert_eq!(js.matches("instantiateCore(").count(), 1);
    Ok(())
}

#[test]
fn non_threaded_reactor_keeps_initialize_export() -> eyre::Result<()> {
    assert!(
        exports(&fold(REACTOR, false)?)?
            .iter()
            .any(|s| s == "_initialize")
    );
    Ok(())
}

#[test]
fn reactor_rejects_invalid_initialize_exports() {
    for (from, to) in [
        (
            "(export \"_initialize\")",
            "(export \"_initialize\") (export \"_initialize\")",
        ),
        (
            "(func $initialize (export \"_initialize\")",
            "(func $initialize (export \"_initialize\") (param i32)",
        ),
        (
            "(func $initialize (export \"_initialize\")",
            "(global (export \"_initialize\") i32 (i32.const 0)) (func $initialize",
        ),
        (
            "(export \"__wasip1_vfs_reactor_init_state\")",
            "(export \"missing_state\")",
        ),
        ("(i32.const 16)", "(i32.const 17)"),
        (
            "(func (export \"hook\"))",
            "(func (export \"hook\") (call $initialize))",
        ),
    ] {
        let result = fold(&REACTOR.replace(from, to), true);
        assert!(result.is_err(), "accepted unexpected reactor: {to}");
    }
}

#[test]
fn old_toolchain_without_initialize_keeps_start_sequence() -> eyre::Result<()> {
    let output = fold(&REACTOR.replace("(export \"_initialize\")", ""), true)?;
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(&output)?;
    Ok(())
}

#[test]
fn reactor_runs_once_across_shared_worker_instances() -> eyre::Result<()> {
    use std::{
        process::{Command, Stdio},
        time::Duration,
    };
    use wait_timeout::ChildExt;

    let input = REACTOR.replace(
        "(memory (export \"memory\") 1 1 shared)",
        "(import \"env\" \"memory\" (memory 1 1 shared)) (export \"memory\" (memory 0))",
    );
    let output = fold(&input, true)?;
    let dir = tempfile::tempdir()?;
    std::fs::write(dir.path().join("vfs.core.wasm"), output)?;
    std::fs::write(
        dir.path().join("worker.mjs"),
        r#"
onmessage = ({ data: { module, memory } }) => {
  const instance = new WebAssembly.Instance(module, { env: { memory } });
  instance.exports._start(); // Repeated exported start must not re-run libc.
  postMessage(new Int32Array(memory.buffer)[2]);
  close();
};
"#,
    )?;
    std::fs::write(
        dir.path().join("test.mjs"),
        r#"
const module = await WebAssembly.compile(await Deno.readFile("vfs.core.wasm"));
const memory = new WebAssembly.Memory({ initial: 1, maximum: 1, shared: true });
const runWorker = () => new Promise((resolve, reject) => {
  const worker = new Worker(new URL("./worker.mjs", import.meta.url), { type: "module" });
  worker.onerror = reject;
  worker.onmessage = ({data}) => data === 99 ? resolve() : reject(new Error(`ctors not ready: ${data}`));
  worker.postMessage({ module, memory });
});
await Promise.all(Array.from({length: 8}, runWorker));
const instance = new WebAssembly.Instance(module, { env: { memory } });
instance.exports._start();
if (new Int32Array(memory.buffer)[1] !== 1) throw new Error("initializer did not run exactly once");
if (new Int32Array(memory.buffer)[4] !== 2) throw new Error("initialization not complete");
"#,
    )?;
    let log = std::fs::File::create(dir.path().join("deno.log"))?;
    let mut child = Command::new("deno")
        .args(["run", "--allow-read", "test.mjs"])
        .current_dir(dir.path())
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .spawn()?;
    let status = child.wait_timeout(Duration::from_secs(30))?;
    if status.is_none() {
        child.kill()?;
        child.wait()?;
    }
    assert!(
        status.is_some_and(|s| s.success()),
        "Deno failed/timed out: {}",
        std::fs::read_to_string(dir.path().join("deno.log"))?
    );
    Ok(())
}

#[test]
fn real_threaded_vfs_has_single_core_output() -> color_eyre::Result<()> {
    if !utils::has_required_wasi_targets(true) {
        return Ok(());
    }
    let dir = utils::run_wasi_virt_layer(
        Some("threads_vfs"),
        Some("test_threads"),
        Some(true),
        true,
        utils::OutDir::Random,
        true,
        &["--validate"],
        None,
    )?;
    let core = std::fs::read(dir.0.join("threads_vfs.core.wasm"))?;
    assert!(!exports(&core)?.iter().any(|s| s == "_initialize"));
    assert!(!dir.0.join("threads_vfs.core2.wasm").exists());
    let js = std::fs::read_to_string(dir.0.join("threads_vfs.js"))?;
    assert!(!js.contains("module1"));
    assert!(!js.contains("core2.wasm"));
    assert_eq!(js.matches("instantiateCore(").count(), 1);
    Ok(())
}

#[test]
fn optimized_threaded_vfs_has_single_core_output() -> color_eyre::Result<()> {
    if !utils::has_required_wasi_targets(true) {
        return Ok(());
    }
    let dir = utils::run_wasi_virt_layer(
        Some("threads_vfs"),
        Some("test_threads"),
        Some(true),
        true,
        utils::OutDir::Random,
        true,
        &["--validate", "--run-with-opt"],
        None,
    )?;
    let core = std::fs::read(dir.0.join("threads_vfs.core.wasm"))?;
    let names = exports(&core)?;
    assert!(
        !names
            .iter()
            .any(|s| s == "_initialize" || s.starts_with("__wvl_reactor_"))
    );
    assert!(!dir.0.join("threads_vfs.core2.wasm").exists());
    let js = std::fs::read_to_string(dir.0.join("threads_vfs.js"))?;
    assert!(!js.contains("module1"));
    assert_eq!(js.matches("instantiateCore(").count(), 1);
    Ok(())
}

#[test]
fn command_without_initialize_is_unchanged() -> eyre::Result<()> {
    let input = wat::parse_str(r#"(module (func (export "_start")))"#)?;
    let input = wasi_virt_layer_cli::wasm_stream::passes::starts_pre::StartsPreStreamPass::new(
        true,
        false,
        "__flesh_vfs_start".into(),
    )
    .run(&input)?;
    let output = PostCombineStreamPass::new("vfs".into(), vec![], vec![1], true).run(&input)?;
    wasmparser::Validator::new().validate_all(&output)?;
    assert!(!exports(&output)?.iter().any(|s| s == "_initialize"));
    Ok(())
}

#[test]
fn duplicate_reactor_export_is_rejected_before_merge_can_deduplicate() -> eyre::Result<()> {
    let input = wat::parse_str(&REACTOR.replace(
        "(export \"_initialize\")",
        "(export \"_initialize\") (export \"_initialize\")",
    ))?;
    let result = wasi_virt_layer_cli::wasm_stream::passes::starts_pre::StartsPreStreamPass::new(
        true,
        false,
        "__flesh_vfs_start".into(),
    )
    .run(&input);
    assert!(result.is_err());
    Ok(())
}

fn initializer_calls_in_start(wasm: &[u8]) -> eyre::Result<usize> {
    use wasi_virt_layer_cli::wasm_stream::passes::reactor_initialize::INITIALIZE_EXPORT;
    let mut initialize = None;
    let mut start = None;
    let mut index = 0;
    let mut calls = 0;
    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        match payload? {
            wasmparser::Payload::ImportSection(s) => {
                for group in s {
                    for import in group? {
                        if matches!(import?.1.ty, wasmparser::TypeRef::Func(_)) {
                            index += 1;
                        }
                    }
                }
            }
            wasmparser::Payload::ExportSection(s) => {
                for e in s {
                    let e = e?;
                    if e.name == INITIALIZE_EXPORT {
                        initialize = Some(e.index);
                    }
                }
            }
            wasmparser::Payload::StartSection { func, .. } => start = Some(func),
            wasmparser::Payload::CodeSectionEntry(body) => {
                if start == Some(index) {
                    for op in body.get_operators_reader()? {
                        if matches!(op?, wasmparser::Operator::Call { function_index } if Some(function_index) == initialize)
                        {
                            calls += 1;
                        }
                    }
                }
                index += 1;
            }
            _ => {}
        }
    }
    Ok(calls)
}

#[test]
fn official_initializer_call_survives_repeated_production_optimization() -> eyre::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = camino::Utf8PathBuf::from_path_buf(dir.path().join("input.wasm")).unwrap();
    std::fs::write(&path, fold(REACTOR, true)?)?;
    let first = wasi_virt_layer_cli::compile::optimize_wasm(
        &path,
        &["--always-inline-max-function-size=100"],
        true,
        false,
    )?;
    assert_eq!(initializer_calls_in_start(&std::fs::read(&first)?)?, 1);
    let second = wasi_virt_layer_cli::compile::optimize_wasm(&first, &[], true, false)?;
    assert_eq!(initializer_calls_in_start(&std::fs::read(&second)?)?, 1);
    Ok(())
}

#[test]
fn unmarked_target_initializer_is_not_folded_as_a_vfs_initializer() -> eyre::Result<()> {
    let output = PostCombineStreamPass::new("vfs".into(), vec![], vec![8], true)
        .run(&wat::parse_str(REACTOR)?)?;
    assert!(exports(&output)?.iter().any(|s| s == "_initialize"));
    Ok(())
}

#[test]
fn reactor_rejects_indirect_initializer_references() {
    let input = REACTOR.replace(
        "(func (export \"hook\"))",
        "(table 1 funcref) (elem (i32.const 0) $initialize) (func (export \"hook\"))",
    );
    assert!(fold(&input, true).is_err());
}

#[test]
fn target_cannot_forge_vfs_reactor_provenance() -> eyre::Result<()> {
    use wasi_virt_layer_cli::wasm_stream::passes::{
        reactor_initialize::{INITIALIZE_EXPORT, PREPARED_STATE_EXPORT},
        starts_pre::StartsPreStreamPass,
    };
    for marker in [INITIALIZE_EXPORT, PREPARED_STATE_EXPORT] {
        let input = wat::parse_str(format!("(module (func (export \"{marker}\")))"))?;
        assert!(
            StartsPreStreamPass::new(false, true, "__flesh_target_start".into())
                .run(&input)
                .is_err()
        );
    }
    Ok(())
}
