use crate::args::TargetMemoryType;
use crate::generator::{ComponentCtx, GeneratorCtx};
use compact_str::CompactString;
use wasm_encoder::{CustomSection, Module};
use wasmparser::{Parser, Payload};

/// Appends component context information as custom sections to the module.
pub fn append_component_ctx(wasm_bytes: &[u8], ctx: &GeneratorCtx) -> eyre::Result<Vec<u8>> {
    // We don't need a full pipeline, we just append to the existing bytes.
    // However, to keep it structurally valid, custom sections should ideally be at the end.
    // We can just append them directly since custom sections can appear anywhere after the module header.
    // Even easier: just append them!

    // Actually, `wasm_encoder::Module` from raw bytes isn't a thing, but we can just
    // write the raw bytes and append custom sections manually.
    let mut out = wasm_bytes.to_vec();

    let add_custom = |out: &mut Vec<u8>, name: &str, data: &[u8]| {
        let mut tmp_module = Module::new();
        tmp_module.section(&CustomSection {
            name: name.into(),
            data: data.into(),
        });
        let bytes = tmp_module.finish();
        out.extend_from_slice(&bytes[8..]);
    };

    let vfs_name = ctx.vfs_name.to_string();
    let target_names = ctx
        .target_names
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    eprintln!(
        "[DEBUG COMPONENT_CTX] Appending context. threads={:?}, adjust_abi={}",
        ctx.threads, ctx.adjust_abi
    );
    add_custom(&mut out, "vfs_name", &serde_json::to_vec(&vfs_name)?);
    add_custom(
        &mut out,
        "target_names",
        &serde_json::to_vec(&target_names)?,
    );
    add_custom(
        &mut out,
        "target_memory_type",
        &serde_json::to_vec(&ctx.target_memory_type)?,
    );
    add_custom(
        &mut out,
        "unstable_print_debug",
        &serde_json::to_vec(&ctx.unstable_print_debug)?,
    );
    add_custom(&mut out, "dwarf", &serde_json::to_vec(&ctx.dwarf)?);
    add_custom(&mut out, "threads", &serde_json::to_vec(&ctx.threads)?);
    add_custom(
        &mut out,
        "adjust_abi",
        &serde_json::to_vec(&ctx.adjust_abi)?,
    );

    Ok(out)
}

/// Reads component context information from custom sections.
pub fn read_component_ctx(wasm_bytes: &[u8]) -> eyre::Result<ComponentCtx> {
    let mut vfs_name = None;
    let mut target_names = None;
    let mut target_memory_type = None;
    let mut unstable_print_debug = None;
    let mut dwarf = None;
    let mut threads = None;
    let mut adjust_abi = None;

    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_bytes) {
        if let Payload::CustomSection(s) = payload? {
            let data = s.data();
            match s.name() {
                "vfs_name" => vfs_name = Some(serde_json::from_slice::<String>(data)?),
                "target_names" => {
                    let cn = serde_json::from_slice::<Vec<String>>(data)?;
                    target_names =
                        Some(cn.into_iter().map(CompactString::from).collect::<Box<_>>());
                }
                "target_memory_type" => {
                    target_memory_type = Some(serde_json::from_slice::<TargetMemoryType>(data)?)
                }
                "unstable_print_debug" => {
                    unstable_print_debug = Some(serde_json::from_slice::<bool>(data)?)
                }
                "dwarf" => dwarf = Some(serde_json::from_slice::<bool>(data)?),
                "threads" => {
                    let t = serde_json::from_slice::<bool>(data)?;
                    eprintln!("[DEBUG COMPONENT_CTX] threads = {}", t);
                    threads = Some(t);
                }
                "adjust_abi" => adjust_abi = Some(serde_json::from_slice::<bool>(data)?),
                _ => {
                    eprintln!(
                        "[DEBUG COMPONENT_CTX] unhandled custom section: {}",
                        s.name()
                    );
                }
            }
        }
    }

    Ok(ComponentCtx {
        vfs_name: vfs_name.map(|s| CompactString::from(s)),
        target_names,
        target_memory_type,
        unstable_print_debug,
        dwarf: dwarf.unwrap_or(false),
        threads,
        adjust_abi: adjust_abi.unwrap_or(false),
        ..Default::default()
    })
}
