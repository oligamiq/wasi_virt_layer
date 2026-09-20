//! Keep the official reactor function distinct through every wasm-opt invocation.
use std::collections::BTreeMap;

use eyre::{Result, ensure};
use wasm_encoder::{ExportKind, ExportSection, Module, NameMap, NameSection, RawSection};
use wasmparser::{BinaryReader, ExternalKind, Operator, Parser, Payload, TypeRef};

use super::reactor_initialize::INITIALIZE_EXPORT;

/// Rediscover the anchor after index-changing passes instead of persisting indices.
fn initializer_index(wasm: &[u8]) -> Result<Option<u32>> {
    let mut found = None;
    for payload in Parser::new(0).parse_all(wasm) {
        if let Payload::ExportSection(s) = payload? {
            for export in s {
                let e = export?;
                if e.name == INITIALIZE_EXPORT {
                    ensure!(
                        found.is_none() && e.kind == ExternalKind::Func,
                        "invalid reactor optimizer anchor"
                    );
                    found = Some(e.index);
                }
            }
        }
    }
    Ok(found)
}

/// Assign a collision-free Binaryen function name; `--no-inline` matches internal
/// names, not export names. Names can be stripped/reindexed between invocations.
/// Preserve the other name subsections and all DWARF/custom sections.
pub fn optimizer_input(wasm: &[u8]) -> Result<Option<(Vec<u8>, String)>> {
    let Some(index) = initializer_index(wasm)? else {
        return Ok(None);
    };
    let mut function_names = BTreeMap::new();
    let mut other_names = BTreeMap::new();
    let mut module = Module::new();
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload?;
        if let Payload::CustomSection(s) = &payload {
            if s.name() == "name" {
                let mut reader = BinaryReader::new(s.data(), s.data_offset());
                while !reader.eof() {
                    let id = reader.read_u8()?;
                    let size = reader.read_var_u32()? as usize;
                    let data = reader.read_bytes(size)?;
                    if id == 1 {
                        let mut names = BinaryReader::new(data, 0);
                        for _ in 0..names.read_var_u32()? {
                            function_names
                                .insert(names.read_var_u32()?, names.read_string()?.to_owned());
                        }
                    } else {
                        other_names.entry(id).or_insert_with(|| data.to_vec());
                    }
                }
                continue;
            }
        }
        if let Some((id, range)) = payload.as_section() {
            module.section(&RawSection {
                id,
                data: &wasm[range],
            });
        }
    }
    function_names.remove(&index);
    let mut name = INITIALIZE_EXPORT.to_owned();
    while function_names.values().any(|existing| existing == &name) {
        name.push('_');
    }
    function_names.insert(index, name.clone());
    let mut names = NameSection::new();
    if let Some(data) = other_names.remove(&0) {
        names.raw(0, &data);
    }
    let mut functions = NameMap::new();
    for (index, name) in function_names {
        functions.append(index, &name);
    }
    names.functions(&functions);
    for (id, data) in other_names {
        names.raw(id, &data);
    }
    module.section(&names);
    Ok(Some((module.finish(), name)))
}

/// Remove the private optimizer anchor only after the final optimization. Fail
/// closed if any downstream transformation has removed the required direct call.
pub fn finish(wasm: &[u8]) -> Result<Vec<u8>> {
    let Some(initialize) = initializer_index(wasm)? else {
        return Ok(wasm.to_vec());
    };
    let mut module = Module::new();
    let mut function_index = 0;
    let mut start = None;
    let mut calls = 0;
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload?;
        match &payload {
            Payload::ImportSection(s) => {
                for group in s.clone() {
                    for i in group? {
                        if matches!(i?.1.ty, TypeRef::Func(_)) {
                            function_index += 1;
                        }
                    }
                }
            }
            Payload::ExportSection(s) => {
                let mut exports = ExportSection::new();
                for e in s.clone() {
                    let e = e?;
                    if e.name == INITIALIZE_EXPORT {
                        continue;
                    }
                    let kind = match e.kind {
                        ExternalKind::Func => ExportKind::Func,
                        ExternalKind::Table => ExportKind::Table,
                        ExternalKind::Memory => ExportKind::Memory,
                        ExternalKind::Global => ExportKind::Global,
                        ExternalKind::Tag => ExportKind::Tag,
                        _ => eyre::bail!("unexpected export kind during reactor finalization"),
                    };
                    exports.export(e.name, kind, e.index);
                }
                module.section(&exports);
                continue;
            }
            Payload::StartSection { func, .. } => start = Some(*func),
            Payload::CodeSectionEntry(body) => {
                if start == Some(function_index) {
                    for op in body.get_operators_reader()? {
                        if matches!(op?, Operator::Call { function_index } if function_index == initialize)
                        {
                            calls += 1;
                        }
                    }
                }
                function_index += 1;
            }
            _ => {}
        }
        if let Some((id, range)) = payload.as_section() {
            module.section(&RawSection {
                id,
                data: &wasm[range],
            });
        }
    }
    ensure!(
        calls == 1,
        "final core start must directly call official _initialize exactly once (found {calls})"
    );
    Ok(module.finish())
}
