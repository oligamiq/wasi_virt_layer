use crate::wasm_stream::pipeline::{StreamPass, par_process_code_section};
use eyre::{ContextCompat, Result};
use std::collections::HashMap;
use wasm_encoder::{Function, Instruction, Module, RawSection};
use wasmparser::{Parser, Payload};

#[derive(Debug, Default)]
pub struct SharedGlobalStreamPass {
    pub threads: bool,
    pub target_names: Vec<String>,
}

impl SharedGlobalStreamPass {
    pub fn new(threads: bool, target_names: Vec<String>) -> Self {
        Self {
            threads,
            target_names,
        }
    }
}

impl StreamPass for SharedGlobalStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        if !self.threads {
            return Ok(input_wasm.to_vec());
        }

        let mut encoder = Module::new();
        let mut global_count = 0;
        let mut global_inits = HashMap::new();
        let mut exports = HashMap::new();
        let mut start_func_id = None;
        let mut func_count = 0;
        let mut imported_func_count = 0;

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::ImportSection(s) => {
                    for group in s.clone() {
                        for i in group?.into_iter() {
                            let (_, i) = i?;
                            if let wasmparser::TypeRef::Func(_) = i.ty {
                                imported_func_count += 1;
                            }
                        }
                    }
                    let range = s.range();
                    encoder.section(&RawSection {
                        id: 2,
                        data: &input_wasm[range.start..range.end],
                    });
                }
                Payload::FunctionSection(s) => {
                    func_count = s.count() + imported_func_count;
                    let range = s.range();
                    encoder.section(&RawSection {
                        id: 3,
                        data: &input_wasm[range.start..range.end],
                    });
                }
                Payload::GlobalSection(s) => {
                    global_count = s.count();
                    for (i, g) in s.clone().into_iter().enumerate() {
                        let g = g?;
                        let mut reader = g.init_expr.get_operators_reader();
                        let op = reader.read()?;
                        if let wasmparser::Operator::I32Const { value } = op {
                            global_inits.insert(i as u32, value);
                        }
                    }
                    let range = s.range();
                    encoder.section(&RawSection {
                        id: 6,
                        data: &input_wasm[range.start..range.end],
                    });
                }
                Payload::ExportSection(s) => {
                    for e in s.clone() {
                        let e = e?;
                        exports.insert(e.name.to_string(), (e.kind, e.index));
                    }
                    let range = s.range();
                    encoder.section(&RawSection {
                        id: 7,
                        data: &input_wasm[range.start..range.end],
                    });
                }
                Payload::StartSection { func, range } => {
                    start_func_id = Some(func);
                    encoder.section(&RawSection {
                        id: 8,
                        data: &input_wasm[range.start..range.start + range.end - range.start],
                    });
                }
                Payload::CodeSectionStart {
                    count: _,
                    range,
                    size: _,
                } => {
                    // Extract export IDs
                    let get_alt = |name: &str| -> eyre::Result<u32> {
                        exports
                            .get(name)
                            .filter(|(k, _)| *k == wasmparser::ExternalKind::Func)
                            .map(|(_, idx)| *idx)
                            .context(format!("Export {} not found", name))
                    };

                    let orig_global_count =
                        global_count.saturating_sub(self.target_names.len() as u32);
                    let mut global_mappings = HashMap::new();
                    let mut init_once_funcs = Vec::new();

                    for (i, target_name) in self.target_names.iter().enumerate() {
                        let g_id = orig_global_count + i as u32;

                        let normalized_target = target_name.replace('-', "_");
                        let prefix = format!("__wasip1_vfs_{normalized_target}_memory_grow_");

                        let get_fn_name = format!("{}global_alt_get", prefix);
                        let set_fn_name = format!("{}global_alt_set_with_lock", prefix);
                        let init_fn_name = format!("{}global_alt_init_once", prefix);

                        let get_fn = get_alt(&get_fn_name).ok();
                        let set_fn = get_alt(&set_fn_name).ok();
                        let init_fn = get_alt(&init_fn_name).ok();

                        if let (Some(get_fn), Some(set_fn)) = (get_fn, set_fn) {
                            global_mappings.insert(g_id, (get_fn, set_fn));
                        }
                        if let Some(init_fn) = init_fn {
                            init_once_funcs.push((g_id, init_fn));
                        }
                    }

                    // find lockers
                    let mut lockers = HashMap::new();
                    for (name, (kind, idx)) in &exports {
                        if *kind == wasmparser::ExternalKind::Func
                            && name.starts_with("__wasip1_vfs_memory_grow_locker_")
                        {
                            let mem_id_str = &name["__wasip1_vfs_memory_grow_locker_".len()..];
                            if let Ok(mem_id) = mem_id_str.parse::<u32>() {
                                lockers.insert(mem_id, *idx);
                            }
                        }
                    }

                    let init_offset_global_fid = exports
                        .get("__init_offset_global")
                        .filter(|(k, _)| *k == wasmparser::ExternalKind::Func)
                        .map(|(_, idx)| *idx);

                    let reader = wasmparser::BinaryReader::new(
                        &input_wasm[range.start..range.end],
                        range.start,
                    );
                    let s = wasmparser::CodeSectionReader::new(reader)?;
                    let new_code_sec = par_process_code_section(s, |i, func_body| {
                        let fid = imported_func_count + i as u32;

                        let mut locals = Vec::new();
                        let mut locals_reader = func_body.get_locals_reader()?;
                        for _ in 0..locals_reader.get_count() {
                            let (count, ty) = locals_reader.read()?;
                            let enc_ty = crate::wasm_stream::translator::translate_val_type(
                                ty,
                                &crate::wasm_stream::translator::DefaultRebinder,
                            );
                            locals.push((count, enc_ty));
                        }

                        let mut func = Function::new(locals);

                        // If this is the start function or __init_offset_global, prepend init_once calls
                        if Some(fid) == start_func_id || Some(fid) == init_offset_global_fid {
                            for (g_id, init_fn) in &init_once_funcs {
                                if let Some(&init_val) = global_inits.get(g_id) {
                                    func.instruction(&Instruction::I32Const(init_val));
                                    func.instruction(&Instruction::Call(*init_fn));
                                }
                            }
                        }

                        let mut reader = func_body.get_operators_reader()?;
                        while !reader.eof() {
                            let op = reader.read()?;
                            match op {
                                wasmparser::Operator::MemoryGrow { mem, .. } => {
                                    if let Some(locker_fn) = lockers.get(&mem) {
                                        func.instruction(&Instruction::Call(*locker_fn));
                                    } else {
                                        func.instruction(&Instruction::MemoryGrow(mem));
                                    }
                                }
                                wasmparser::Operator::GlobalGet { global_index } => {
                                    if let Some(&(get_fn, _)) = global_mappings.get(&global_index) {
                                        func.instruction(&Instruction::Call(get_fn));
                                    } else {
                                        func.instruction(&Instruction::GlobalGet(global_index));
                                    }
                                }
                                wasmparser::Operator::GlobalSet { global_index } => {
                                    if let Some(&(_, set_fn)) = global_mappings.get(&global_index) {
                                        func.instruction(&Instruction::Call(set_fn));
                                    } else {
                                        func.instruction(&Instruction::GlobalSet(global_index));
                                    }
                                }
                                _ => {
                                    func.instruction(&crate::wasm_stream::translator::translate(
                                        &op,
                                        &crate::wasm_stream::translator::DefaultRebinder,
                                    ));
                                }
                            }
                        }
                        Ok(func)
                    })?;
                    encoder.section(&new_code_sec);
                }
                wasmparser::Payload::CustomSection(c) => {
                    encoder.section(&wasm_encoder::CustomSection {
                        name: c.name().into(),
                        data: std::borrow::Cow::Borrowed(c.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        encoder.section(&RawSection {
                            id,
                            data: &input_wasm[range.start..range.end],
                        });
                    }
                }
            }
        }

        Ok(encoder.finish())
    }
}
