use std::collections::{HashMap, HashSet};
use std::path::Path;

use eyre::{Context as _, ContextCompat as _};
use wasm_encoder::{
    CodeSection, DataCountSection, DataSection, ElementSection, EntityType, ExportKind,
    ExportSection, Function, FunctionSection, GlobalSection, ImportSection, MemorySection, Module,
    TableSection, TagSection, TypeSection,
};
use wasmparser::{ExternalKind, Payload, TypeRef};

use crate::unique_name::UniqueName;
use crate::wasm_stream::translator::{self, Rebind};

/// A module participating in the in-process merge.
pub struct MergeInput<'a> {
    /// Alias used for cross-module imports.
    pub alias: String,
    /// Path to the prepared core Wasm module.
    pub path: &'a Path,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum IndexKind {
    Func,
    Table,
    Memory,
    Global,
    Tag,
}

#[derive(Clone, Copy, Debug)]
struct ExportRef {
    module: usize,
    kind: IndexKind,
    index: u32,
}

#[derive(Clone, Debug)]
struct ImportInfo {
    module: String,
    name: String,
    ty: TypeRef,
    kind: IndexKind,
    old_index: u32,
    resolved: Option<ExportRef>,
    final_index: Option<u32>,
}

#[derive(Default, Clone, Copy, Debug)]
struct Counts {
    types: u32,
    func_imports: u32,
    table_imports: u32,
    memory_imports: u32,
    global_imports: u32,
    tag_imports: u32,
    funcs: u32,
    tables: u32,
    memories: u32,
    globals: u32,
    tags: u32,
    elems: u32,
    data: u32,
}

#[derive(Default, Clone, Copy, Debug)]
struct Offsets {
    types: u32,
    funcs: u32,
    tables: u32,
    memories: u32,
    globals: u32,
    tags: u32,
    elems: u32,
    data: u32,
}

#[derive(Debug)]
struct ParsedModule {
    alias: String,
    bytes: Vec<u8>,
    imports: Vec<ImportInfo>,
    exports: Vec<(String, ExternalKind, u32)>,
    counts: Counts,
    offsets: Offsets,
    has_data_count: bool,
}

#[derive(Clone, Copy)]
struct ResolvedIndex {
    value: u32,
    resolving: bool,
}

#[derive(Default)]
struct IndexMaps {
    funcs: Vec<Vec<ResolvedIndex>>,
    tables: Vec<Vec<ResolvedIndex>>,
    memories: Vec<Vec<ResolvedIndex>>,
    globals: Vec<Vec<ResolvedIndex>>,
    tags: Vec<Vec<ResolvedIndex>>,
}

struct MergeRebinder<'a> {
    module: usize,
    modules: &'a [ParsedModule],
    maps: &'a IndexMaps,
}

impl Rebind for MergeRebinder<'_> {
    fn function(&self, index: u32) -> u32 {
        self.maps.funcs[self.module][index as usize].value
    }

    fn global(&self, index: u32) -> u32 {
        self.maps.globals[self.module][index as usize].value
    }

    fn memory(&self, index: u32) -> u32 {
        self.maps.memories[self.module][index as usize].value
    }

    fn table(&self, index: u32) -> u32 {
        self.maps.tables[self.module][index as usize].value
    }

    fn ty(&self, index: u32) -> u32 {
        self.modules[self.module].offsets.types + index
    }

    fn data(&self, index: u32) -> u32 {
        self.modules[self.module].offsets.data + index
    }

    fn elem(&self, index: u32) -> u32 {
        self.modules[self.module].offsets.elems + index
    }

    fn tag(&self, index: u32) -> u32 {
        self.maps.tags[self.module][index as usize].value
    }
}

/// Merge prepared VFS and target modules without invoking an external merger.
pub fn merge_modules(
    inputs: &[MergeInput<'_>],
    output: impl AsRef<Path>,
    dwarf: bool,
) -> eyre::Result<()> {
    let mut modules = inputs
        .iter()
        .map(parse_module)
        .collect::<eyre::Result<Vec<_>>>()?;

    resolve_imports(&mut modules)?;
    assign_offsets(&mut modules);
    let maps = build_index_maps(&modules)?;
    let bytes = emit_merged_module(&modules, &maps, dwarf)?;
    std::fs::write(output, bytes).wrap_err("Failed to write merged Wasm")?;
    Ok(())
}

fn parse_module(input: &MergeInput<'_>) -> eyre::Result<ParsedModule> {
    let bytes = std::fs::read(input.path)
        .wrap_err_with(|| format!("Failed to read Wasm module {}", input.path.display()))?;
    let mut counts = Counts::default();
    let mut imports = Vec::new();
    let mut exports = Vec::new();
    let mut has_data_count = false;

    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        match payload.wrap_err_with(|| format!("Failed to parse {}", input.path.display()))? {
            Payload::TypeSection(section) => {
                for ty in section {
                    counts.types += ty?.into_types().count() as u32;
                }
            }
            Payload::ImportSection(section) => {
                for group in section {
                    for item in group?.into_iter() {
                        let (_, import) = item?;
                        let (kind, old_index) = match import.ty {
                            TypeRef::Func(_) | TypeRef::FuncExact(_) => {
                                let idx = counts.func_imports;
                                counts.func_imports += 1;
                                (IndexKind::Func, idx)
                            }
                            TypeRef::Table(_) => {
                                let idx = counts.table_imports;
                                counts.table_imports += 1;
                                (IndexKind::Table, idx)
                            }
                            TypeRef::Memory(_) => {
                                let idx = counts.memory_imports;
                                counts.memory_imports += 1;
                                (IndexKind::Memory, idx)
                            }
                            TypeRef::Global(_) => {
                                let idx = counts.global_imports;
                                counts.global_imports += 1;
                                (IndexKind::Global, idx)
                            }
                            TypeRef::Tag(_) => {
                                let idx = counts.tag_imports;
                                counts.tag_imports += 1;
                                (IndexKind::Tag, idx)
                            }
                        };
                        imports.push(ImportInfo {
                            module: import.module.to_string(),
                            name: import.name.to_string(),
                            ty: import.ty,
                            kind,
                            old_index,
                            resolved: None,
                            final_index: None,
                        });
                    }
                }
            }
            Payload::FunctionSection(section) => counts.funcs = section.count(),
            Payload::TableSection(section) => counts.tables = section.count(),
            Payload::MemorySection(section) => counts.memories = section.count(),
            Payload::GlobalSection(section) => counts.globals = section.count(),
            Payload::TagSection(section) => counts.tags = section.count(),
            Payload::ElementSection(section) => counts.elems = section.count(),
            Payload::DataSection(section) => counts.data = section.count(),
            Payload::DataCountSection { count, .. } => {
                counts.data = counts.data.max(count);
                has_data_count = true;
            }
            Payload::ExportSection(section) => {
                for export in section {
                    let export = export?;
                    exports.push((export.name.to_string(), export.kind, export.index));
                }
            }
            Payload::StartSection { .. } => {
                eyre::bail!(
                    "Prepared module `{}` still has a start section; expected StartsPreStreamPass output",
                    input.alias
                );
            }
            _ => {}
        }
    }

    Ok(ParsedModule {
        alias: input.alias.clone(),
        bytes,
        imports,
        exports,
        counts,
        offsets: Offsets::default(),
        has_data_count,
    })
}

fn resolve_imports(modules: &mut [ParsedModule]) -> eyre::Result<()> {
    let alias_to_module = modules
        .iter()
        .enumerate()
        .map(|(idx, module)| (module.alias.clone(), idx))
        .collect::<HashMap<_, _>>();
    let mut exports = HashMap::new();
    for (module_idx, module) in modules.iter().enumerate() {
        for (name, kind, index) in &module.exports {
            let Some(kind) = kind_from_external(*kind) else {
                continue;
            };
            let export_ref = ExportRef {
                module: module_idx,
                kind,
                index: *index,
            };
            for export_name in alias_export_names(&module.alias, name) {
                exports
                    .entry((module.alias.clone(), export_name, kind))
                    .or_insert(export_ref);
            }
        }
    }

    for module_idx in 0..modules.len() {
        let importing_alias = modules[module_idx].alias.clone();
        for import in &mut modules[module_idx].imports {
            let Some(&target_module_idx) = alias_to_module.get(&import.module) else {
                continue;
            };
            let key = (import.module.clone(), import.name.clone(), import.kind);
            if let Some(export) = exports.get(&key).copied() {
                import.resolved = Some(export);
            } else if target_module_idx == module_idx
                && importing_alias == UniqueName::WASIP1_ABI_MODULE
            {
                import.module = "__wasip1_vfs-host".to_string();
                import.name = format!("__wasip1_vfs___self_{}", import.name);
            } else {
                eyre::bail!(
                    "Module `{}` imports `{}::{}` from merge alias `{}`, but no matching export exists",
                    importing_alias,
                    import.module,
                    import.name,
                    import.module
                );
            }
        }
    }

    Ok(())
}

fn assign_offsets(modules: &mut [ParsedModule]) {
    let mut type_offset = 0;
    for module in modules.iter_mut() {
        module.offsets.types = type_offset;
        type_offset += module.counts.types;
    }

    let mut unresolved = Counts::default();
    for module in modules.iter_mut() {
        for import in &mut module.imports {
            if import.resolved.is_some() {
                continue;
            }
            let final_index = match import.kind {
                IndexKind::Func => {
                    let idx = unresolved.func_imports;
                    unresolved.func_imports += 1;
                    idx
                }
                IndexKind::Table => {
                    let idx = unresolved.table_imports;
                    unresolved.table_imports += 1;
                    idx
                }
                IndexKind::Memory => {
                    let idx = unresolved.memory_imports;
                    unresolved.memory_imports += 1;
                    idx
                }
                IndexKind::Global => {
                    let idx = unresolved.global_imports;
                    unresolved.global_imports += 1;
                    idx
                }
                IndexKind::Tag => {
                    let idx = unresolved.tag_imports;
                    unresolved.tag_imports += 1;
                    idx
                }
            };
            import.final_index = Some(final_index);
        }
    }

    let mut offsets = Offsets {
        funcs: unresolved.func_imports,
        tables: unresolved.table_imports,
        memories: unresolved.memory_imports,
        globals: unresolved.global_imports,
        tags: unresolved.tag_imports,
        ..Offsets::default()
    };

    for module in modules.iter_mut() {
        module.offsets.funcs = offsets.funcs;
        offsets.funcs += module.counts.funcs;
        module.offsets.tables = offsets.tables;
        offsets.tables += module.counts.tables;
        module.offsets.memories = offsets.memories;
        offsets.memories += module.counts.memories;
        module.offsets.globals = offsets.globals;
        offsets.globals += module.counts.globals;
        module.offsets.tags = offsets.tags;
        offsets.tags += module.counts.tags;
        module.offsets.elems = offsets.elems;
        offsets.elems += module.counts.elems;
        module.offsets.data = offsets.data;
        offsets.data += module.counts.data;
    }
}

fn build_index_maps(modules: &[ParsedModule]) -> eyre::Result<IndexMaps> {
    let mut maps = IndexMaps {
        funcs: allocate_maps(modules, IndexKind::Func),
        tables: allocate_maps(modules, IndexKind::Table),
        memories: allocate_maps(modules, IndexKind::Memory),
        globals: allocate_maps(modules, IndexKind::Global),
        tags: allocate_maps(modules, IndexKind::Tag),
    };

    for module_idx in 0..modules.len() {
        fill_direct_map(modules, &mut maps, module_idx, IndexKind::Func);
        fill_direct_map(modules, &mut maps, module_idx, IndexKind::Table);
        fill_direct_map(modules, &mut maps, module_idx, IndexKind::Memory);
        fill_direct_map(modules, &mut maps, module_idx, IndexKind::Global);
        fill_direct_map(modules, &mut maps, module_idx, IndexKind::Tag);
    }

    for module_idx in 0..modules.len() {
        resolve_all_kind(modules, &mut maps, module_idx, IndexKind::Func)?;
        resolve_all_kind(modules, &mut maps, module_idx, IndexKind::Table)?;
        resolve_all_kind(modules, &mut maps, module_idx, IndexKind::Memory)?;
        resolve_all_kind(modules, &mut maps, module_idx, IndexKind::Global)?;
        resolve_all_kind(modules, &mut maps, module_idx, IndexKind::Tag)?;
    }

    Ok(maps)
}

fn allocate_maps(modules: &[ParsedModule], kind: IndexKind) -> Vec<Vec<ResolvedIndex>> {
    modules
        .iter()
        .map(|module| {
            let len = imported_count(module, kind) + defined_count(module, kind);
            vec![
                ResolvedIndex {
                    value: u32::MAX,
                    resolving: false,
                };
                len as usize
            ]
        })
        .collect()
}

fn fill_direct_map(
    modules: &[ParsedModule],
    maps: &mut IndexMaps,
    module_idx: usize,
    kind: IndexKind,
) {
    let module = &modules[module_idx];
    for import in module.imports.iter().filter(|import| import.kind == kind) {
        if let Some(final_index) = import.final_index {
            kind_map_mut(maps, kind)[module_idx][import.old_index as usize].value = final_index;
        }
    }

    let import_count = imported_count(module, kind);
    let def_offset = offset_for(module, kind);
    for local_idx in 0..defined_count(module, kind) {
        kind_map_mut(maps, kind)[module_idx][(import_count + local_idx) as usize].value =
            def_offset + local_idx;
    }
}

fn resolve_all_kind(
    modules: &[ParsedModule],
    maps: &mut IndexMaps,
    module_idx: usize,
    kind: IndexKind,
) -> eyre::Result<()> {
    let len = kind_map(maps, kind)[module_idx].len();
    for index in 0..len {
        resolve_index(modules, maps, module_idx, kind, index as u32)?;
    }
    Ok(())
}

fn resolve_index(
    modules: &[ParsedModule],
    maps: &mut IndexMaps,
    module_idx: usize,
    kind: IndexKind,
    index: u32,
) -> eyre::Result<u32> {
    let slot = &kind_map(maps, kind)[module_idx][index as usize];
    if slot.value != u32::MAX {
        return Ok(slot.value);
    }
    if slot.resolving {
        eyre::bail!(
            "Cyclic merge import resolution for module `{}` index {:?} {}",
            modules[module_idx].alias,
            kind,
            index
        );
    }
    kind_map_mut(maps, kind)[module_idx][index as usize].resolving = true;

    let import = modules[module_idx]
        .imports
        .iter()
        .find(|import| import.kind == kind && import.old_index == index)
        .with_context(|| {
            format!(
                "Missing merge import mapping for module `{}` index {:?} {}",
                modules[module_idx].alias, kind, index
            )
        })?;
    let resolved = import
        .resolved
        .with_context(|| format!("Unresolved import `{}` has no final index", import.name))?;
    let value = resolve_index(
        modules,
        maps,
        resolved.module,
        resolved.kind,
        resolved.index,
    )?;
    let slot = &mut kind_map_mut(maps, kind)[module_idx][index as usize];
    slot.value = value;
    slot.resolving = false;
    Ok(value)
}

fn emit_merged_module(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    dwarf: bool,
) -> eyre::Result<Vec<u8>> {
    let mut module = Module::new();
    emit_type_section(modules, maps, &mut module)?;
    emit_import_section(modules, maps, &mut module)?;
    emit_function_section(modules, &mut module)?;
    emit_table_section(modules, maps, &mut module)?;
    emit_memory_section(modules, &mut module)?;
    emit_tag_section(modules, maps, &mut module)?;
    emit_global_section(modules, maps, &mut module)?;
    emit_export_section(modules, maps, &mut module)?;
    emit_element_section(modules, maps, &mut module)?;

    let has_data_count = modules.iter().any(|m| m.has_data_count);
    let total_data = modules.iter().map(|m| m.counts.data).sum::<u32>();
    if has_data_count {
        module.section(&DataCountSection { count: total_data });
    }

    emit_code_section(modules, maps, &mut module)?;
    emit_data_section(modules, maps, &mut module)?;
    emit_custom_sections(modules, dwarf, &mut module)?;

    Ok(module.finish())
}

fn emit_type_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = TypeSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::TypeSection(types) = payload? {
                for ty in types {
                    let ty = ty?;
                    if ty.is_explicit_rec_group() {
                        let rec_types = ty
                            .into_types()
                            .map(|sub_ty| translator::translate_sub_type(&sub_ty, &rebinder))
                            .collect::<Vec<_>>();
                        section.ty().rec(rec_types);
                    } else {
                        for sub_ty in ty.into_types() {
                            section
                                .ty()
                                .subtype(&translator::translate_sub_type(&sub_ty, &rebinder));
                        }
                    }
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_import_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = ImportSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for import in parsed
            .imports
            .iter()
            .filter(|import| import.resolved.is_none())
        {
            section.import(
                &import.module,
                &import.name,
                translate_type_ref(import.ty, &rebinder),
            );
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_function_section(modules: &[ParsedModule], module: &mut Module) -> eyre::Result<()> {
    let mut section = FunctionSection::new();
    for parsed in modules {
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::FunctionSection(functions) = payload? {
                for ty in functions {
                    section.function(parsed.offsets.types + ty?);
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_table_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = TableSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::TableSection(tables) = payload? {
                for table in tables {
                    section.table(translator::translate_table_type(table?.ty, &rebinder));
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_memory_section(modules: &[ParsedModule], module: &mut Module) -> eyre::Result<()> {
    let mut section = MemorySection::new();
    for parsed in modules {
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::MemorySection(memories) = payload? {
                for memory in memories {
                    section.memory(translator::translate_memory_type(memory?));
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_tag_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = TagSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::TagSection(tags) = payload? {
                for tag in tags {
                    let tag = translator::translate_tag_type(tag?);
                    section.tag(wasm_encoder::TagType {
                        kind: tag.kind,
                        func_type_idx: rebinder.ty(tag.func_type_idx),
                    });
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_global_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = GlobalSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::GlobalSection(globals) = payload? {
                for global in globals {
                    let global = global?;
                    let init = translate_const_expr_extended(&global.init_expr, &rebinder)?;
                    section.global(
                        translator::translate_global_type(global.ty, &rebinder),
                        &init,
                    );
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_export_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = ExportSection::new();
    let mut used_names = HashSet::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for (name, kind, index) in &parsed.exports {
            let final_index = rebind_external_index(*kind, *index, &rebinder);
            let export_name = unique_export_name(&mut used_names, &parsed.alias, name);
            section.export(&export_name, export_kind(*kind), final_index);
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_element_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = ElementSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::ElementSection(elements) = payload? {
                for elem in elements {
                    let elem = elem?;
                    let elements = translate_elements(elem.items, &rebinder)?;
                    match elem.kind {
                        wasmparser::ElementKind::Passive => {
                            section.passive(elements);
                        }
                        wasmparser::ElementKind::Active {
                            table_index,
                            offset_expr,
                        } => {
                            let table = Some(rebinder.table(table_index.unwrap_or(0)));
                            let offset = translate_const_expr_extended(&offset_expr, &rebinder)?;
                            section.active(table, &offset, elements);
                        }
                        wasmparser::ElementKind::Declared => {
                            section.declared(elements);
                        }
                    }
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_code_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = CodeSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::CodeSectionEntry(body) = payload? {
                let mut locals = Vec::new();
                for local in body.get_locals_reader()? {
                    let local = local?;
                    locals.push((local.0, translator::translate_val_type(local.1, &rebinder)));
                }
                let mut func = Function::new(locals);
                for op in body.get_operators_reader()? {
                    let op = op?;
                    func.instruction(&translator::translate(&op, &rebinder));
                }
                section.function(&func);
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_data_section(
    modules: &[ParsedModule],
    maps: &IndexMaps,
    module: &mut Module,
) -> eyre::Result<()> {
    let mut section = DataSection::new();
    for (module_idx, parsed) in modules.iter().enumerate() {
        let rebinder = MergeRebinder {
            module: module_idx,
            modules,
            maps,
        };
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::DataSection(data_section) = payload? {
                for data in data_section {
                    let data = data?;
                    match data.kind {
                        wasmparser::DataKind::Passive => {
                            section.passive(data.data.iter().copied());
                        }
                        wasmparser::DataKind::Active {
                            memory_index,
                            offset_expr,
                        } => {
                            let offset = translate_const_expr_extended(&offset_expr, &rebinder)?;
                            section.active(
                                rebinder.memory(memory_index),
                                &offset,
                                data.data.iter().copied(),
                            );
                        }
                    }
                }
            }
        }
    }
    if !section.is_empty() {
        module.section(&section);
    }
    Ok(())
}

fn emit_custom_sections(
    modules: &[ParsedModule],
    dwarf: bool,
    module: &mut Module,
) -> eyre::Result<()> {
    for parsed in modules {
        for payload in wasmparser::Parser::new(0).parse_all(&parsed.bytes) {
            if let Payload::CustomSection(custom) = payload? {
                if !dwarf && (custom.name() == "name" || custom.name().starts_with(".debug_")) {
                    continue;
                }
                module.section(&wasm_encoder::CustomSection {
                    name: custom.name().into(),
                    data: std::borrow::Cow::Borrowed(custom.data()),
                });
            }
        }
    }
    Ok(())
}

fn translate_elements<'a>(
    items: wasmparser::ElementItems<'a>,
    rebinder: &impl Rebind,
) -> eyre::Result<wasm_encoder::Elements<'a>> {
    match items {
        wasmparser::ElementItems::Functions(functions) => {
            let funcs = functions
                .into_iter()
                .map(|idx| Ok(rebinder.function(idx?)))
                .collect::<eyre::Result<Vec<_>>>()?;
            Ok(wasm_encoder::Elements::Functions(std::borrow::Cow::Owned(
                funcs,
            )))
        }
        wasmparser::ElementItems::Expressions(ty, expressions) => {
            let ty = translator::translate_ref_type(ty, rebinder);
            let expressions = expressions
                .into_iter()
                .map(|expr| translate_const_expr_extended(&expr?, rebinder))
                .collect::<eyre::Result<Vec<_>>>()?;
            Ok(wasm_encoder::Elements::Expressions(
                ty,
                std::borrow::Cow::Owned(expressions),
            ))
        }
    }
}

fn translate_const_expr_extended(
    expr: &wasmparser::ConstExpr,
    rebinder: &impl Rebind,
) -> eyre::Result<wasm_encoder::ConstExpr> {
    let mut instrs = Vec::new();
    for op in expr.get_operators_reader() {
        let op = op?;
        if matches!(op, wasmparser::Operator::End) {
            continue;
        }
        let instr = match op {
            wasmparser::Operator::RefNull { hty } => {
                wasm_encoder::Instruction::RefNull(translator::translate_heap_type(hty, rebinder))
            }
            _ => translator::translate(&op, rebinder),
        };
        instrs.push(instr);
    }
    Ok(wasm_encoder::ConstExpr::extended(instrs))
}

fn translate_type_ref(ty: TypeRef, rebinder: &impl Rebind) -> EntityType {
    match ty {
        TypeRef::Func(idx) => EntityType::Function(rebinder.ty(idx)),
        TypeRef::FuncExact(idx) => EntityType::FunctionExact(rebinder.ty(idx)),
        TypeRef::Table(table) => {
            EntityType::Table(translator::translate_table_type(table, rebinder))
        }
        TypeRef::Memory(memory) => EntityType::Memory(translator::translate_memory_type(memory)),
        TypeRef::Global(global) => {
            EntityType::Global(translator::translate_global_type(global, rebinder))
        }
        TypeRef::Tag(tag) => EntityType::Tag(wasm_encoder::TagType {
            kind: match tag.kind {
                wasmparser::TagKind::Exception => wasm_encoder::TagKind::Exception,
            },
            func_type_idx: rebinder.ty(tag.func_type_idx),
        }),
    }
}

fn rebind_external_index(kind: ExternalKind, index: u32, rebinder: &impl Rebind) -> u32 {
    match kind {
        ExternalKind::Func | ExternalKind::FuncExact => rebinder.function(index),
        ExternalKind::Table => rebinder.table(index),
        ExternalKind::Memory => rebinder.memory(index),
        ExternalKind::Global => rebinder.global(index),
        ExternalKind::Tag => rebinder.tag(index),
    }
}

fn export_kind(kind: ExternalKind) -> ExportKind {
    match kind {
        ExternalKind::Func | ExternalKind::FuncExact => ExportKind::Func,
        ExternalKind::Table => ExportKind::Table,
        ExternalKind::Memory => ExportKind::Memory,
        ExternalKind::Global => ExportKind::Global,
        ExternalKind::Tag => ExportKind::Tag,
    }
}

fn kind_from_external(kind: ExternalKind) -> Option<IndexKind> {
    match kind {
        ExternalKind::Func | ExternalKind::FuncExact => Some(IndexKind::Func),
        ExternalKind::Table => Some(IndexKind::Table),
        ExternalKind::Memory => Some(IndexKind::Memory),
        ExternalKind::Global => Some(IndexKind::Global),
        ExternalKind::Tag => Some(IndexKind::Tag),
    }
}

fn alias_export_names(alias: &str, export_name: &str) -> Vec<String> {
    let mut names = vec![export_name.to_string()];

    if alias == UniqueName::WASIP1_ABI_MODULE {
        for prefix in ["__wasip1_vfs___self_", "__wasip1_vfs_self_"] {
            if let Some(name) = export_name.strip_prefix(prefix) {
                names.push(name.to_string());
            }
        }
    }

    if let Some(target_name) = alias.strip_prefix("wasip1_vfs_") {
        let normalized_prefix = format!("__wasip1_vfs_{target_name}_");
        if let Some(name) = export_name.strip_prefix(&normalized_prefix) {
            names.push(name.to_string());
        }
    }

    names
}

fn unique_export_name(used: &mut HashSet<String>, alias: &str, name: &str) -> String {
    if used.insert(name.to_string()) {
        return name.to_string();
    }
    let mut candidate = format!("__{alias}_{name}");
    let mut suffix = 1;
    while !used.insert(candidate.clone()) {
        candidate = format!("__{alias}_{name}_{suffix}");
        suffix += 1;
    }
    candidate
}

fn imported_count(module: &ParsedModule, kind: IndexKind) -> u32 {
    match kind {
        IndexKind::Func => module.counts.func_imports,
        IndexKind::Table => module.counts.table_imports,
        IndexKind::Memory => module.counts.memory_imports,
        IndexKind::Global => module.counts.global_imports,
        IndexKind::Tag => module.counts.tag_imports,
    }
}

fn defined_count(module: &ParsedModule, kind: IndexKind) -> u32 {
    match kind {
        IndexKind::Func => module.counts.funcs,
        IndexKind::Table => module.counts.tables,
        IndexKind::Memory => module.counts.memories,
        IndexKind::Global => module.counts.globals,
        IndexKind::Tag => module.counts.tags,
    }
}

fn offset_for(module: &ParsedModule, kind: IndexKind) -> u32 {
    match kind {
        IndexKind::Func => module.offsets.funcs,
        IndexKind::Table => module.offsets.tables,
        IndexKind::Memory => module.offsets.memories,
        IndexKind::Global => module.offsets.globals,
        IndexKind::Tag => module.offsets.tags,
    }
}

fn kind_map(maps: &IndexMaps, kind: IndexKind) -> &[Vec<ResolvedIndex>] {
    match kind {
        IndexKind::Func => &maps.funcs,
        IndexKind::Table => &maps.tables,
        IndexKind::Memory => &maps.memories,
        IndexKind::Global => &maps.globals,
        IndexKind::Tag => &maps.tags,
    }
}

fn kind_map_mut(maps: &mut IndexMaps, kind: IndexKind) -> &mut [Vec<ResolvedIndex>] {
    match kind {
        IndexKind::Func => &mut maps.funcs,
        IndexKind::Table => &mut maps.tables,
        IndexKind::Memory => &mut maps.memories,
        IndexKind::Global => &mut maps.globals,
        IndexKind::Tag => &mut maps.tags,
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use wasm_encoder::{
        CodeSection, ExportKind, ExportSection, Function, FunctionSection, ImportSection,
        Instruction, Module, TypeSection,
    };

    use super::*;

    #[test]
    fn resolves_cross_module_function_import() -> eyre::Result<()> {
        let dir = tempdir()?;
        let a = dir.path().join("a.wasm");
        let b = dir.path().join("b.wasm");
        let out = dir.path().join("merged.wasm");

        std::fs::write(&a, exported_const_module("answer", 7))?;
        std::fs::write(&b, importing_module("a", "answer"))?;

        merge_modules(
            &[
                MergeInput {
                    alias: "a".to_string(),
                    path: &a,
                },
                MergeInput {
                    alias: "b".to_string(),
                    path: &b,
                },
            ],
            &out,
            false,
        )?;

        let bytes = std::fs::read(out)?;
        let mut import_count = 0;
        let mut has_call = false;
        for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
            match payload? {
                Payload::ImportSection(section) => import_count += section.count(),
                Payload::CodeSectionEntry(body) => {
                    for op in body.get_operators_reader()? {
                        if matches!(op?, wasmparser::Operator::Call { function_index: 0 }) {
                            has_call = true;
                        }
                    }
                }
                _ => {}
            }
        }
        assert_eq!(import_count, 0);
        assert!(has_call);
        Ok(())
    }

    fn exported_const_module(export: &str, value: i32) -> Vec<u8> {
        let mut module = Module::new();
        let mut types = TypeSection::new();
        types.ty().function([], [wasm_encoder::ValType::I32]);
        module.section(&types);
        let mut funcs = FunctionSection::new();
        funcs.function(0);
        module.section(&funcs);
        let mut exports = ExportSection::new();
        exports.export(export, ExportKind::Func, 0);
        module.section(&exports);
        let mut code = CodeSection::new();
        let mut func = Function::new([]);
        func.instruction(&Instruction::I32Const(value));
        func.instruction(&Instruction::End);
        code.function(&func);
        module.section(&code);
        module.finish()
    }

    fn importing_module(module_name: &str, name: &str) -> Vec<u8> {
        let mut module = Module::new();
        let mut types = TypeSection::new();
        types.ty().function([], [wasm_encoder::ValType::I32]);
        module.section(&types);
        let mut imports = ImportSection::new();
        imports.import(module_name, name, EntityType::Function(0));
        module.section(&imports);
        let mut funcs = FunctionSection::new();
        funcs.function(0);
        module.section(&funcs);
        let mut code = CodeSection::new();
        let mut func = Function::new([]);
        func.instruction(&Instruction::Call(0));
        func.instruction(&Instruction::Drop);
        func.instruction(&Instruction::End);
        code.function(&func);
        module.section(&code);
        module.finish()
    }
}
