use crate::wasm_stream::pipeline::{StreamPass, par_process_code_section};
use eyre::{ContextCompat, Result};
use std::collections::HashMap;
use wasm_encoder::{ExportKind, ExportSection, Function, Instruction, Module, RawSection};
use wasmparser::{Parser, Payload};

const LOWERING_HELPERS_SECTION: &str = "wvl.multi_memory_lowering.helpers.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoweringHelperKind {
    Size,
    Grow,
}

#[derive(Clone, Copy, Debug)]
struct LoweringHelpers {
    size_start: u32,
    grow_start: u32,
    grow_end: u32,
}

impl LoweringHelpers {
    fn parse(data: &[u8]) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let orig_func_count = u32::from_le_bytes(data[0..4].try_into().ok()?);
        let memory_count = u32::from_le_bytes(data[4..8].try_into().ok()?);
        Some(Self {
            size_start: orig_func_count,
            grow_start: orig_func_count + memory_count,
            grow_end: orig_func_count + memory_count * 2,
        })
    }

    fn kind(self, fid: u32) -> Option<LoweringHelperKind> {
        if (self.size_start..self.grow_start).contains(&fid) {
            Some(LoweringHelperKind::Size)
        } else if (self.grow_start..self.grow_end).contains(&fid) {
            Some(LoweringHelperKind::Grow)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SharedGlobalFns {
    get_with_lock: u32,
    set_with_lock: u32,
    get_no_wait: u32,
    set_no_lock: u32,
}

#[derive(Debug, Default)]
pub struct SharedGlobalStreamPass {
    pub threads: bool,
    pub own_memory: bool,
    pub target_names: Vec<String>,
}

impl SharedGlobalStreamPass {
    pub fn new(threads: bool, own_memory: bool, target_names: Vec<String>) -> Self {
        Self {
            threads,
            own_memory,
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
        let mut lowering_helpers: Option<LoweringHelpers> = None;

        for payload in Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::ImportSection(s) => {
                    for group in s.clone() {
                        for i in group?.into_iter() {
                            let (_, i) = i?;
                            match i.ty {
                                wasmparser::TypeRef::Func(_)
                                | wasmparser::TypeRef::FuncExact(_) => {
                                    imported_func_count += 1;
                                }
                                wasmparser::TypeRef::Global(_) => {
                                    global_count += 1;
                                }
                                _ => {}
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
                    let global_index_base = global_count;
                    global_count += s.count();
                    for (i, g) in s.clone().into_iter().enumerate() {
                        let g = g?;
                        let mut reader = g.init_expr.get_operators_reader();
                        let op = reader.read()?;
                        if let wasmparser::Operator::I32Const { value } = op {
                            global_inits.insert(global_index_base + i as u32, value);
                        }
                    }
                    let range = s.range();
                    encoder.section(&RawSection {
                        id: 6,
                        data: &input_wasm[range.start..range.end],
                    });
                }
                Payload::ExportSection(s) => {
                    let mut new_exports = ExportSection::new();
                    for e in s.clone() {
                        let e = e?;
                        exports.insert(e.name.to_string(), (e.kind, e.index));
                        if e.name == "__init_offset_global" {
                            continue;
                        }
                        let kind = match e.kind {
                            wasmparser::ExternalKind::Func
                            | wasmparser::ExternalKind::FuncExact => ExportKind::Func,
                            wasmparser::ExternalKind::Table => ExportKind::Table,
                            wasmparser::ExternalKind::Memory => ExportKind::Memory,
                            wasmparser::ExternalKind::Global => ExportKind::Global,
                            wasmparser::ExternalKind::Tag => ExportKind::Tag,
                        };
                        new_exports.export(e.name, kind, e.index);
                    }
                    encoder.section(&new_exports);
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
                    let mut global_mappings: HashMap<u32, SharedGlobalFns> = HashMap::new();
                    let mut init_once_funcs = Vec::new();

                    for (i, target_name) in self.target_names.iter().enumerate() {
                        let g_id = orig_global_count + i as u32;

                        let normalized_target = target_name.replace('-', "_");
                        let prefix = format!("__wasip1_vfs_{normalized_target}_memory_grow_");

                        let get_with_lock_name = format!("{}global_alt_get", prefix);
                        let set_with_lock_name = format!("{}global_alt_set_with_lock", prefix);
                        let get_no_wait_name = format!("{}global_alt_get_no_wait", prefix);
                        let set_no_lock_name = format!("{}global_alt_set", prefix);
                        let init_fn_name = format!("{}global_alt_init_once", prefix);

                        let get_with_lock = get_alt(&get_with_lock_name).ok();
                        let set_with_lock = get_alt(&set_with_lock_name).ok();
                        let get_no_wait = get_alt(&get_no_wait_name).ok();
                        let set_no_lock = get_alt(&set_no_lock_name).ok();
                        let init_fn = get_alt(&init_fn_name).ok();

                        if let (
                            Some(get_with_lock),
                            Some(set_with_lock),
                            Some(get_no_wait),
                            Some(set_no_lock),
                        ) = (get_with_lock, set_with_lock, get_no_wait, set_no_lock)
                        {
                            global_mappings.insert(
                                g_id,
                                SharedGlobalFns {
                                    get_with_lock,
                                    set_with_lock,
                                    get_no_wait,
                                    set_no_lock,
                                },
                            );
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
                        let helper_kind = lowering_helpers.and_then(|helpers| helpers.kind(fid));

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

                        // Prefer the explicit helper when it exists so callers control ordering.
                        if Some(fid) == init_offset_global_fid
                            || (init_offset_global_fid.is_none() && Some(fid) == start_func_id)
                        {
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
                                    if let Some(fns) = global_mappings.get(&global_index) {
                                        let get_fn = if helper_kind
                                            == Some(LoweringHelperKind::Grow)
                                            || self.own_memory
                                        {
                                            fns.get_no_wait
                                        } else {
                                            fns.get_with_lock
                                        };
                                        func.instruction(&Instruction::Call(get_fn));
                                    } else {
                                        func.instruction(&Instruction::GlobalGet(global_index));
                                    }
                                }
                                wasmparser::Operator::GlobalSet { global_index } => {
                                    if let Some(fns) = global_mappings.get(&global_index) {
                                        let set_fn = if helper_kind
                                            == Some(LoweringHelperKind::Grow)
                                            || self.own_memory
                                        {
                                            fns.set_no_lock
                                        } else {
                                            fns.set_with_lock
                                        };
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
                    if c.name() == LOWERING_HELPERS_SECTION {
                        lowering_helpers = LoweringHelpers::parse(c.data());
                        continue;
                    }
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

#[cfg(test)]
mod tests {
    use super::{LoweringHelperKind, LoweringHelpers};

    #[test]
    fn lowering_helpers_decode_size_and_grow_ranges() {
        let mut data = Vec::new();
        data.extend_from_slice(&100_u32.to_le_bytes());
        data.extend_from_slice(&3_u32.to_le_bytes());

        let helpers = LoweringHelpers::parse(&data).unwrap();

        assert_eq!(helpers.kind(99), None);
        assert_eq!(helpers.kind(100), Some(LoweringHelperKind::Size));
        assert_eq!(helpers.kind(102), Some(LoweringHelperKind::Size));
        assert_eq!(helpers.kind(103), Some(LoweringHelperKind::Grow));
        assert_eq!(helpers.kind(105), Some(LoweringHelperKind::Grow));
        assert_eq!(helpers.kind(106), None);
    }

    #[test]
    fn lowering_helpers_reject_malformed_metadata() {
        assert!(LoweringHelpers::parse(&[0; 7]).is_none());
        assert!(LoweringHelpers::parse(&[0; 9]).is_none());
    }
}
