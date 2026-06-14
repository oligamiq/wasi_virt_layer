use std::collections::HashMap;

use eyre::{ContextCompat as _, Result};
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, ElementSection, Elements, EntityType, ExportKind,
    ExportSection, Function, FunctionSection, GlobalSection, GlobalType, ImportSection,
    Instruction, MemArg, Module, RawSection, StartSection, TypeSection, ValType,
};
use wasmparser::{CompositeInnerType, ExternalKind, KnownCustom, Name, Payload, TypeRef};

use crate::wasm_stream::{
    pipeline::StreamPass,
    translator::{
        DefaultRebinder, Rebind, translate, translate_global_type, translate_memory_type,
        translate_sub_type, translate_table_type, translate_tag_type, translate_val_type,
    },
};

const HANDOFF_EXPORT: &str = "__wasip1_vfs_stack_handoff_vfs";
const TARGET_HANDOFF_EXPORT: &str = "__wasip1_vfs_stack_handoff_targets";
const ENSURE_EXPORT: &str = "__wasip1_vfs_stack_ensure_vfs";
const CLAIM_TARGET_EXPORT: &str = "__wasip1_vfs_stack_claim_target";
const READY: i32 = 2;
const THREAD_MANAGED: i32 = 3;
const STANDBY_FAILED: i32 = -1;

/// Adds stackless dynamic-stack handoff wrappers to VFS function exports.
pub struct ExportStackPreVfsStreamPass {
    stack_size: Option<u32>,
}

/// Adds dynamic-stack handoff wrappers to a single-memory target module.
pub struct ExportStackPreTargetStreamPass {
    target_name: String,
    vfs_name: String,
    target_index: u32,
    stack_size: Option<u32>,
}

impl ExportStackPreTargetStreamPass {
    /// Creates a target export-stack pass. `None` leaves the module unchanged.
    pub fn new(
        target_name: String,
        vfs_name: String,
        target_index: u32,
        stack_size: Option<u32>,
    ) -> Self {
        Self {
            target_name,
            vfs_name,
            target_index,
            stack_size,
        }
    }
}

impl ExportStackPreVfsStreamPass {
    /// Creates a VFS export-stack pass. `None` leaves the module unchanged.
    pub const fn new(stack_size: Option<u32>) -> Self {
        Self { stack_size }
    }
}

#[derive(Clone)]
struct ExportedFunction {
    name: String,
    original_index: u32,
    type_index: u32,
    thread_start: bool,
}

fn is_thread_start(name: &str) -> bool {
    name == "wasi_thread_start"
        || name.ends_with("#wasi-thread-start")
        || name.contains("_wasi_thread_start")
}

fn should_protect(name: &str) -> bool {
    !is_thread_start(name)
        && name != "cabi_realloc"
        && !name.starts_with("cabi_realloc_")
        && !name.starts_with("__wasip1_vfs_stack_")
        && !name.starts_with("__flesh_")
        && !name.contains("reset_globals")
        && !name.ends_with("_resetter")
}

fn wasm_export_kind(kind: ExternalKind) -> ExportKind {
    match kind {
        ExternalKind::Func | ExternalKind::FuncExact => ExportKind::Func,
        ExternalKind::Table => ExportKind::Table,
        ExternalKind::Memory => ExportKind::Memory,
        ExternalKind::Global => ExportKind::Global,
        ExternalKind::Tag => ExportKind::Tag,
    }
}

fn atomic_memarg(offset: u64) -> MemArg {
    MemArg {
        offset,
        align: 2,
        memory_index: 0,
    }
}

fn emit_unlock(func: &mut Function, handoff_global: u32) {
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(0)));
}

#[allow(clippy::too_many_arguments)]
fn build_ensure_function(
    stack_size: u32,
    handoff_global: u32,
    stack_pointer: u32,
    realloc: u32,
    state_global: u32,
    current_base_global: u32,
    current_end_global: u32,
    generation_global: u32,
) -> Function {
    // Locals: current base, current end, replacement base, generation.
    let mut func = Function::new([(4, ValType::I32)]);

    for accepted_state in [READY, THREAD_MANAGED] {
        func.instruction(&Instruction::GlobalGet(state_global));
        func.instruction(&Instruction::I32Const(accepted_state));
        func.instruction(&Instruction::I32Eq);
        func.instruction(&Instruction::If(BlockType::Empty));
        func.instruction(&Instruction::I32Const(0));
        func.instruction(&Instruction::Return);
        func.instruction(&Instruction::End);
    }

    // Serialize every possible use of the compiler-provided bootstrap stack.
    func.instruction(&Instruction::Block(BlockType::Empty));
    func.instruction(&Instruction::Loop(BlockType::Empty));
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::I32AtomicRmwCmpxchg(atomic_memarg(0)));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::BrIf(1));
    func.instruction(&Instruction::Br(0));
    func.instruction(&Instruction::End);
    func.instruction(&Instruction::End);

    // Another re-entrant path may have initialized this instance while it
    // waited. This check also keeps explicit ensure calls idempotent.
    for accepted_state in [READY, THREAD_MANAGED] {
        func.instruction(&Instruction::GlobalGet(state_global));
        func.instruction(&Instruction::I32Const(accepted_state));
        func.instruction(&Instruction::I32Eq);
        func.instruction(&Instruction::If(BlockType::Empty));
        emit_unlock(&mut func, handoff_global);
        func.instruction(&Instruction::I32Const(0));
        func.instruction(&Instruction::Return);
        func.instruction(&Instruction::End);
    }

    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::GlobalSet(state_global));

    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32AtomicLoad(atomic_memarg(4)));
    func.instruction(&Instruction::LocalTee(0));

    // A failed standby allocation permanently blocks new consumers. Existing
    // READY instances continue to use their already assigned stack.
    func.instruction(&Instruction::I32Const(STANDBY_FAILED));
    func.instruction(&Instruction::I32Eq);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::GlobalSet(state_global));
    emit_unlock(&mut func, handoff_global);
    func.instruction(&Instruction::I32Const(2));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));

    // Initial bootstrap allocation. The lock makes the otherwise shared
    // compiler stack exclusive for this allocator call.
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(16));
    func.instruction(&Instruction::I32Const(stack_size as i32));
    func.instruction(&Instruction::Call(realloc));
    func.instruction(&Instruction::LocalTee(0));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::GlobalSet(state_global));
    emit_unlock(&mut func, handoff_global);
    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);
    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::I32Const(stack_size as i32));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::LocalSet(1));

    func.instruction(&Instruction::Else);
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32AtomicLoad(atomic_memarg(8)));
    func.instruction(&Instruction::LocalSet(1));
    // Consume the published standby stack while still holding the lock.
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(8)));
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::GlobalSet(current_base_global));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::GlobalSet(current_end_global));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::GlobalSet(stack_pointer));
    func.instruction(&Instruction::I32Const(READY));
    func.instruction(&Instruction::GlobalSet(state_global));

    // This allocation runs after the stack switch and prepares the next
    // instance's stack.
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(16));
    func.instruction(&Instruction::I32Const(stack_size as i32));
    func.instruction(&Instruction::Call(realloc));
    func.instruction(&Instruction::LocalTee(2));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32Const(STANDBY_FAILED));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    emit_unlock(&mut func, handoff_global);
    func.instruction(&Instruction::I32Const(2));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::LocalGet(2));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::LocalGet(2));
    func.instruction(&Instruction::I32Const(stack_size as i32));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(8)));

    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::I32AtomicLoad(atomic_memarg(12)));
    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::LocalSet(3));
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::LocalGet(3));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(12)));
    func.instruction(&Instruction::LocalGet(3));
    func.instruction(&Instruction::GlobalSet(generation_global));

    emit_unlock(&mut func, handoff_global);
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::End);
    func
}

fn build_claim_target_function(handoff_global: u32, realloc: u32) -> Function {
    // Params: target index, stack size.
    // Locals: base, end, replacement base, record address, generation.
    let mut func = Function::new([(5, ValType::I32)]);
    func.instruction(&Instruction::GlobalGet(handoff_global));
    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::I32Const(4));
    func.instruction(&Instruction::I32Shl);
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::LocalSet(5));

    func.instruction(&Instruction::Block(BlockType::Empty));
    func.instruction(&Instruction::Loop(BlockType::Empty));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::I32AtomicRmwCmpxchg(atomic_memarg(0)));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::BrIf(1));
    func.instruction(&Instruction::Br(0));
    func.instruction(&Instruction::End);
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32AtomicLoad(atomic_memarg(4)));
    func.instruction(&Instruction::LocalTee(2));
    func.instruction(&Instruction::I32Const(STANDBY_FAILED));
    func.instruction(&Instruction::I32Eq);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(0)));
    func.instruction(&Instruction::I64Const(0));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::LocalGet(2));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(16));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::Call(realloc));
    func.instruction(&Instruction::LocalTee(2));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(STANDBY_FAILED));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(0)));
    func.instruction(&Instruction::I64Const(0));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);
    func.instruction(&Instruction::LocalGet(2));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::LocalSet(3));
    func.instruction(&Instruction::Else);
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32AtomicLoad(atomic_memarg(8)));
    func.instruction(&Instruction::LocalSet(3));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(8)));
    func.instruction(&Instruction::End);

    // Allocate the next standby while still executing on the VFS stack.
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32Const(16));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::Call(realloc));
    func.instruction(&Instruction::LocalTee(4));
    func.instruction(&Instruction::I32Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));
    // Restore the claimed stack so a later call can retry replenishment.
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::LocalGet(2));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::LocalGet(3));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(8)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(0)));
    func.instruction(&Instruction::I64Const(0));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::LocalGet(4));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(4)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::LocalGet(4));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(8)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32AtomicLoad(atomic_memarg(12)));
    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::LocalSet(6));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::LocalGet(6));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(12)));
    func.instruction(&Instruction::LocalGet(5));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::I32AtomicStore(atomic_memarg(0)));

    func.instruction(&Instruction::LocalGet(3));
    func.instruction(&Instruction::I64ExtendI32U);
    func.instruction(&Instruction::I64Const(32));
    func.instruction(&Instruction::I64Shl);
    func.instruction(&Instruction::LocalGet(2));
    func.instruction(&Instruction::I64ExtendI32U);
    func.instruction(&Instruction::I64Or);
    func.instruction(&Instruction::End);
    func
}

#[allow(clippy::too_many_arguments)]
fn build_target_ensure_function(
    stack_size: u32,
    target_index: u32,
    stack_pointer: u32,
    director: u32,
    ensure_vfs: u32,
    claim_target: u32,
    state_global: u32,
    current_base_global: u32,
    current_end_global: u32,
    generation_global: u32,
) -> Function {
    // Locals: packed physical range, target physical base.
    let mut func = Function::new([(1, ValType::I64), (1, ValType::I32)]);
    func.instruction(&Instruction::Call(ensure_vfs));
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::I32Const(3));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);

    for accepted_state in [READY, THREAD_MANAGED] {
        func.instruction(&Instruction::GlobalGet(state_global));
        func.instruction(&Instruction::I32Const(accepted_state));
        func.instruction(&Instruction::I32Eq);
        func.instruction(&Instruction::If(BlockType::Empty));
        func.instruction(&Instruction::I32Const(0));
        func.instruction(&Instruction::Return);
        func.instruction(&Instruction::End);
    }

    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::GlobalSet(state_global));
    func.instruction(&Instruction::I32Const(target_index as i32));
    func.instruction(&Instruction::I32Const(stack_size as i32));
    func.instruction(&Instruction::Call(claim_target));
    func.instruction(&Instruction::LocalTee(0));
    func.instruction(&Instruction::I64Eqz);
    func.instruction(&Instruction::If(BlockType::Empty));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::GlobalSet(state_global));
    func.instruction(&Instruction::I32Const(1));
    func.instruction(&Instruction::Return);
    func.instruction(&Instruction::End);

    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::Call(director));
    func.instruction(&Instruction::LocalSet(1));
    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::I32WrapI64);
    func.instruction(&Instruction::GlobalSet(current_base_global));
    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::I64Const(32));
    func.instruction(&Instruction::I64ShrU);
    func.instruction(&Instruction::I32WrapI64);
    func.instruction(&Instruction::GlobalSet(current_end_global));
    func.instruction(&Instruction::GlobalGet(current_end_global));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::I32Sub);
    func.instruction(&Instruction::GlobalSet(stack_pointer));
    func.instruction(&Instruction::I32Const(READY));
    func.instruction(&Instruction::GlobalSet(state_global));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::GlobalSet(generation_global));
    func.instruction(&Instruction::I32Const(0));
    func.instruction(&Instruction::End);
    func
}

fn build_wrapper(
    export: &ExportedFunction,
    function_type: &wasmparser::FuncType,
    ensure_index: u32,
    state_global: u32,
    depth_global: u32,
) -> Function {
    let params = function_type.params();
    let results = function_type.results();
    let result_locals = results
        .iter()
        .copied()
        .map(|ty| (1, translate_val_type(ty, &DefaultRebinder)))
        .collect::<Vec<_>>();
    let first_result_local = params.len() as u32;
    let mut func = Function::new(result_locals);

    if export.thread_start {
        func.instruction(&Instruction::I32Const(THREAD_MANAGED));
        func.instruction(&Instruction::GlobalSet(state_global));
    } else {
        func.instruction(&Instruction::Call(ensure_index));
        func.instruction(&Instruction::If(BlockType::Empty));
        func.instruction(&Instruction::Unreachable);
        func.instruction(&Instruction::End);
        func.instruction(&Instruction::GlobalGet(depth_global));
        func.instruction(&Instruction::I32Const(1));
        func.instruction(&Instruction::I32Add);
        func.instruction(&Instruction::GlobalSet(depth_global));
    }

    for index in 0..params.len() as u32 {
        func.instruction(&Instruction::LocalGet(index));
    }
    func.instruction(&Instruction::Call(export.original_index));

    if !export.thread_start {
        for result_index in (0..results.len() as u32).rev() {
            func.instruction(&Instruction::LocalSet(first_result_local + result_index));
        }
        func.instruction(&Instruction::GlobalGet(depth_global));
        func.instruction(&Instruction::I32Const(1));
        func.instruction(&Instruction::I32Sub);
        func.instruction(&Instruction::GlobalSet(depth_global));
        for result_index in 0..results.len() as u32 {
            func.instruction(&Instruction::LocalGet(first_result_local + result_index));
        }
    }

    func.instruction(&Instruction::End);
    func
}

struct TargetRebinder {
    imported_functions: u32,
}

impl Rebind for TargetRebinder {
    fn function(&self, index: u32) -> u32 {
        if index < self.imported_functions {
            index
        } else {
            index + 3
        }
    }

    fn global(&self, index: u32) -> u32 {
        index
    }
}

fn encode_import_type(ty: TypeRef) -> EntityType {
    match ty {
        TypeRef::Func(index) | TypeRef::FuncExact(index) => EntityType::Function(index),
        TypeRef::Table(table) => EntityType::Table(translate_table_type(table, &DefaultRebinder)),
        TypeRef::Memory(memory) => EntityType::Memory(translate_memory_type(memory)),
        TypeRef::Global(global) => {
            EntityType::Global(translate_global_type(global, &DefaultRebinder))
        }
        TypeRef::Tag(tag) => EntityType::Tag(translate_tag_type(tag)),
    }
}

fn find_function_type(
    types: &[wasmparser::SubType],
    params: &[wasmparser::ValType],
    results: &[wasmparser::ValType],
) -> Option<u32> {
    types
        .iter()
        .position(|ty| {
            matches!(
                &ty.composite_type.inner,
                CompositeInnerType::Func(func)
                    if func.params() == params && func.results() == results
            )
        })
        .map(|index| index as u32)
}

impl StreamPass for ExportStackPreTargetStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let Some(stack_size) = self.stack_size else {
            return Ok(input_wasm.to_vec());
        };

        let mut types = Vec::new();
        let mut function_types = Vec::new();
        let mut imported_functions = 0_u32;
        let mut imported_globals = 0_u32;
        let mut local_globals = 0_u32;
        let mut defined_functions = 0_u32;
        let mut stack_pointer = None;
        let mut has_import_section = false;
        let mut raw_exports = Vec::new();

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(section) => {
                    for group in section {
                        types.extend(group?.into_types());
                    }
                }
                Payload::ImportSection(section) => {
                    has_import_section = true;
                    for group in section {
                        for import in group?.into_iter() {
                            let (_, import) = import?;
                            match import.ty {
                                TypeRef::Func(index) | TypeRef::FuncExact(index) => {
                                    imported_functions += 1;
                                    function_types.push(index);
                                }
                                TypeRef::Global(_) => imported_globals += 1,
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(section) => {
                    for index in section {
                        function_types.push(index?);
                        defined_functions += 1;
                    }
                }
                Payload::GlobalSection(section) => local_globals = section.count(),
                Payload::ExportSection(section) => {
                    for export in section {
                        let export = export?;
                        if export.name == "__stack_pointer" && export.kind == ExternalKind::Global {
                            stack_pointer = Some(export.index);
                        }
                        raw_exports.push((export.name.to_string(), export.kind, export.index));
                    }
                }
                Payload::CustomSection(section) => {
                    if stack_pointer.is_none()
                        && let KnownCustom::Name(names) = section.as_known()
                    {
                        for subsection in names {
                            if let Name::Global(map) = subsection? {
                                for naming in map {
                                    let naming = naming?;
                                    if naming.name == "__stack_pointer" {
                                        stack_pointer = Some(naming.index);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let Some(stack_pointer) = stack_pointer else {
            log::warn!(
                "export-stack isolation requested for target `{}`, but `__stack_pointer` was not found; leaving its exports unchanged",
                self.target_name
            );
            return Ok(input_wasm.to_vec());
        };
        if !has_import_section {
            log::warn!(
                "export-stack isolation requested for target `{}`, but it has no import section; leaving its exports unchanged",
                self.target_name
            );
            return Ok(input_wasm.to_vec());
        }

        let i32_result = [wasmparser::ValType::I32];
        let director_params = [wasmparser::ValType::I32];
        let empty_params = [];
        let claim_params = [wasmparser::ValType::I32, wasmparser::ValType::I32];
        let i64_result = [wasmparser::ValType::I64];

        let mut appended_types = Vec::new();
        let director_type = find_function_type(&types, &director_params, &i32_result)
            .unwrap_or_else(|| {
                let index = (types.len() + appended_types.len()) as u32;
                appended_types.push((director_params.to_vec(), i32_result.to_vec()));
                index
            });
        let ensure_type =
            find_function_type(&types, &empty_params, &i32_result).unwrap_or_else(|| {
                let index = (types.len() + appended_types.len()) as u32;
                appended_types.push((Vec::new(), i32_result.to_vec()));
                index
            });
        let claim_type =
            find_function_type(&types, &claim_params, &i64_result).unwrap_or_else(|| {
                let index = (types.len() + appended_types.len()) as u32;
                appended_types.push((claim_params.to_vec(), i64_result.to_vec()));
                index
            });

        let rebinder = TargetRebinder { imported_functions };
        let mut exports = Vec::new();
        for (name, kind, index) in &raw_exports {
            if matches!(kind, ExternalKind::Func | ExternalKind::FuncExact)
                && (should_protect(name) || is_thread_start(name))
            {
                let type_index = *function_types
                    .get(*index as usize)
                    .wrap_err_with(|| format!("missing function type for export `{name}`"))?;
                exports.push(ExportedFunction {
                    name: name.clone(),
                    original_index: rebinder.function(*index),
                    type_index,
                    thread_start: is_thread_start(name),
                });
            }
        }
        if exports.is_empty() {
            log::warn!(
                "export-stack isolation requested for target `{}`, but no protectable exports exist",
                self.target_name
            );
            return Ok(input_wasm.to_vec());
        }

        let director_index = imported_functions;
        let ensure_vfs_index = imported_functions + 1;
        let claim_target_index = imported_functions + 2;
        let first_new_global = imported_globals + local_globals;
        let state_global = first_new_global;
        let current_base_global = first_new_global + 1;
        let current_end_global = first_new_global + 2;
        let depth_global = first_new_global + 3;
        let generation_global = first_new_global + 4;
        let ensure_index = imported_functions + 3 + defined_functions;
        let wrapper_indices = exports
            .iter()
            .enumerate()
            .map(|(index, export)| (export.name.clone(), ensure_index + 1 + index as u32))
            .collect::<HashMap<_, _>>();
        let stack_pointer = rebinder.global(stack_pointer);
        let director_name = format!("__wasip1_vfs_{}_memory_director", self.target_name);
        let ensure_export = format!("__wasip1_vfs_{}_stack_ensure", self.target_name);

        let mut module = Module::new();
        let mut saw_code = false;
        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::TypeSection(section) => {
                    let mut output = TypeSection::new();
                    for group in section {
                        let group = group?;
                        if group.is_explicit_rec_group() {
                            output.ty().rec(
                                group
                                    .into_types()
                                    .map(|ty| translate_sub_type(&ty, &DefaultRebinder)),
                            );
                        } else {
                            for ty in group.into_types() {
                                output
                                    .ty()
                                    .subtype(&translate_sub_type(&ty, &DefaultRebinder));
                            }
                        }
                    }
                    for (params, results) in &appended_types {
                        output.ty().function(
                            params
                                .iter()
                                .copied()
                                .map(|ty| translate_val_type(ty, &DefaultRebinder)),
                            results
                                .iter()
                                .copied()
                                .map(|ty| translate_val_type(ty, &DefaultRebinder)),
                        );
                    }
                    module.section(&output);
                }
                Payload::ImportSection(section) => {
                    let mut output = ImportSection::new();
                    for group in section {
                        for import in group?.into_iter() {
                            let (_, import) = import?;
                            output.import(
                                import.module,
                                import.name,
                                encode_import_type(import.ty),
                            );
                        }
                    }
                    output.import(
                        "wasip1-vfs",
                        &director_name,
                        EntityType::Function(director_type),
                    );
                    output.import(
                        &self.vfs_name,
                        ENSURE_EXPORT,
                        EntityType::Function(ensure_type),
                    );
                    output.import(
                        &self.vfs_name,
                        CLAIM_TARGET_EXPORT,
                        EntityType::Function(claim_type),
                    );
                    module.section(&output);
                }
                Payload::FunctionSection(section) => {
                    let mut output = FunctionSection::new();
                    for function in section {
                        output.function(function?);
                    }
                    output.function(ensure_type);
                    for export in &exports {
                        output.function(export.type_index);
                    }
                    module.section(&output);
                }
                Payload::GlobalSection(section) => {
                    let mut output = GlobalSection::new();
                    for global in section {
                        let global = global?;
                        let mut instructions = Vec::new();
                        for operator in global.init_expr.get_operators_reader() {
                            let operator = operator?;
                            if !matches!(operator, wasmparser::Operator::End) {
                                instructions.push(translate(&operator, &rebinder));
                            }
                        }
                        output.global(
                            translate_global_type(global.ty, &DefaultRebinder),
                            &ConstExpr::extended(instructions),
                        );
                    }
                    for _ in 0..5 {
                        output.global(
                            GlobalType {
                                val_type: ValType::I32,
                                mutable: true,
                                shared: false,
                            },
                            &ConstExpr::i32_const(0),
                        );
                    }
                    module.section(&output);
                }
                Payload::ExportSection(section) => {
                    let mut output = ExportSection::new();
                    for export in section {
                        let export = export?;
                        let index = match export.kind {
                            ExternalKind::Func | ExternalKind::FuncExact => wrapper_indices
                                .get(export.name)
                                .copied()
                                .unwrap_or_else(|| rebinder.function(export.index)),
                            ExternalKind::Global => rebinder.global(export.index),
                            _ => export.index,
                        };
                        output.export(export.name, wasm_export_kind(export.kind), index);
                    }
                    output.export(&ensure_export, ExportKind::Func, ensure_index);
                    module.section(&output);
                }
                Payload::StartSection { func, .. } => {
                    module.section(&StartSection {
                        function_index: rebinder.function(func),
                    });
                }
                Payload::ElementSection(section) => {
                    let mut output = ElementSection::new();
                    for element in section {
                        let element = element?;
                        let function_storage;
                        let expression_storage;
                        let items = match element.items {
                            wasmparser::ElementItems::Functions(functions) => {
                                function_storage = functions
                                    .into_iter()
                                    .map(|index| Ok(rebinder.function(index?)))
                                    .collect::<Result<Vec<_>>>()?;
                                Elements::Functions(std::borrow::Cow::Borrowed(&function_storage))
                            }
                            wasmparser::ElementItems::Expressions(ref_type, expressions) => {
                                expression_storage = expressions
                                    .into_iter()
                                    .map(|expression| {
                                        let expression = expression?;
                                        let mut instructions = Vec::new();
                                        for operator in expression.get_operators_reader() {
                                            let operator = operator?;
                                            if !matches!(operator, wasmparser::Operator::End) {
                                                instructions.push(translate(&operator, &rebinder));
                                            }
                                        }
                                        Ok(ConstExpr::extended(instructions))
                                    })
                                    .collect::<Result<Vec<_>>>()?;
                                Elements::Expressions(
                                    crate::wasm_stream::translator::translate_ref_type(
                                        ref_type,
                                        &DefaultRebinder,
                                    ),
                                    std::borrow::Cow::Borrowed(&expression_storage),
                                )
                            }
                        };
                        match element.kind {
                            wasmparser::ElementKind::Passive => {
                                output.passive(items);
                            }
                            wasmparser::ElementKind::Declared => {
                                output.declared(items);
                            }
                            wasmparser::ElementKind::Active {
                                table_index,
                                offset_expr,
                            } => {
                                let mut instructions = Vec::new();
                                for operator in offset_expr.get_operators_reader() {
                                    let operator = operator?;
                                    if !matches!(operator, wasmparser::Operator::End) {
                                        instructions.push(translate(&operator, &rebinder));
                                    }
                                }
                                output.active(
                                    table_index,
                                    &ConstExpr::extended(instructions),
                                    items,
                                );
                            }
                        }
                    }
                    module.section(&output);
                }
                Payload::CodeSectionStart { .. } => {
                    saw_code = true;
                    let mut output = CodeSection::new();
                    for nested in wasmparser::Parser::new(0).parse_all(input_wasm) {
                        if let Payload::CodeSectionEntry(body) = nested? {
                            let locals = body
                                .get_locals_reader()?
                                .into_iter()
                                .map(|local| {
                                    let (count, ty) = local?;
                                    Ok((count, translate_val_type(ty, &DefaultRebinder)))
                                })
                                .collect::<Result<Vec<_>>>()?;
                            let mut function = Function::new(locals);
                            for operator in body.get_operators_reader()? {
                                function.instruction(&translate(&operator?, &rebinder));
                            }
                            output.function(&function);
                        }
                    }
                    output.function(&build_target_ensure_function(
                        stack_size,
                        self.target_index,
                        stack_pointer,
                        director_index,
                        ensure_vfs_index,
                        claim_target_index,
                        state_global,
                        current_base_global,
                        current_end_global,
                        generation_global,
                    ));
                    for export in &exports {
                        let function_type =
                            match &types[export.type_index as usize].composite_type.inner {
                                CompositeInnerType::Func(function_type) => function_type,
                                _ => {
                                    eyre::bail!(
                                        "export `{}` does not have a function type",
                                        export.name
                                    )
                                }
                            };
                        output.function(&build_wrapper(
                            export,
                            function_type,
                            ensure_index,
                            state_global,
                            depth_global,
                        ));
                    }
                    module.section(&output);
                }
                Payload::CodeSectionEntry(_) => {}
                Payload::DataSection(section) => {
                    let mut output = wasm_encoder::DataSection::new();
                    for data in section {
                        let data = data?;
                        match data.kind {
                            wasmparser::DataKind::Passive => {
                                output.passive(data.data.iter().copied());
                            }
                            wasmparser::DataKind::Active {
                                memory_index,
                                offset_expr,
                            } => {
                                let mut instructions = Vec::new();
                                for operator in offset_expr.get_operators_reader() {
                                    let operator = operator?;
                                    if !matches!(operator, wasmparser::Operator::End) {
                                        instructions.push(translate(&operator, &rebinder));
                                    }
                                }
                                output.active(
                                    memory_index,
                                    &ConstExpr::extended(instructions),
                                    data.data.iter().copied(),
                                );
                            }
                        }
                    }
                    module.section(&output);
                }
                Payload::CustomSection(section) => {
                    module.section(&wasm_encoder::CustomSection {
                        name: section.name().into(),
                        data: std::borrow::Cow::Borrowed(section.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        module.section(&RawSection {
                            id,
                            data: &input_wasm[range],
                        });
                    }
                }
            }
        }
        if !saw_code {
            eyre::bail!("target `{}` has no code section", self.target_name);
        }
        Ok(module.finish())
    }
}

impl StreamPass for ExportStackPreVfsStreamPass {
    fn run(&mut self, input_wasm: &[u8]) -> Result<Vec<u8>> {
        let Some(stack_size) = self.stack_size else {
            return Ok(input_wasm.to_vec());
        };

        let mut types = Vec::new();
        let mut function_types = Vec::new();
        let mut import_func_count = 0_u32;
        let mut import_global_count = 0_u32;
        let mut local_global_count = 0_u32;
        let mut defined_func_count = 0_u32;
        let mut stack_pointer = None;
        let mut handoff_global = None;
        let mut target_handoff_global = None;
        let mut realloc = None;
        let mut shared_memory = false;
        let mut raw_exports = Vec::new();

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            match payload? {
                Payload::TypeSection(section) => {
                    for group in section {
                        types.extend(group?.into_types());
                    }
                }
                Payload::ImportSection(section) => {
                    for group in section {
                        for import in group?.into_iter() {
                            let (_, import) = import?;
                            match import.ty {
                                TypeRef::Func(index) | TypeRef::FuncExact(index) => {
                                    import_func_count += 1;
                                    function_types.push(index);
                                }
                                TypeRef::Global(_) => import_global_count += 1,
                                TypeRef::Memory(memory) => shared_memory |= memory.shared,
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(section) => {
                    for index in section {
                        function_types.push(index?);
                        defined_func_count += 1;
                    }
                }
                Payload::MemorySection(section) => {
                    for memory in section {
                        shared_memory |= memory?.shared;
                    }
                }
                Payload::GlobalSection(section) => local_global_count = section.count(),
                Payload::ExportSection(section) => {
                    for export in section {
                        let export = export?;
                        if export.name == "__stack_pointer" && export.kind == ExternalKind::Global {
                            stack_pointer = Some(export.index);
                        } else if export.name == HANDOFF_EXPORT
                            && export.kind == ExternalKind::Global
                        {
                            handoff_global = Some(export.index);
                        } else if export.name == TARGET_HANDOFF_EXPORT
                            && export.kind == ExternalKind::Global
                        {
                            target_handoff_global = Some(export.index);
                        } else if export.name == "cabi_realloc"
                            && matches!(export.kind, ExternalKind::Func | ExternalKind::FuncExact)
                        {
                            realloc = Some(export.index);
                        }
                        raw_exports.push((export.name.to_string(), export.kind, export.index));
                    }
                }
                Payload::CustomSection(section) => {
                    if stack_pointer.is_none()
                        && let KnownCustom::Name(names) = section.as_known()
                    {
                        for subsection in names {
                            if let Name::Global(map) = subsection? {
                                for naming in map {
                                    let naming = naming?;
                                    if naming.name == "__stack_pointer" {
                                        stack_pointer = Some(naming.index);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let Some(stack_pointer) = stack_pointer else {
            log::warn!(
                "export-stack isolation requested for VFS, but `__stack_pointer` was not found; leaving VFS exports unchanged"
            );
            return Ok(input_wasm.to_vec());
        };
        let Some(handoff_global) = handoff_global else {
            log::warn!(
                "export-stack isolation requested for VFS, but `{HANDOFF_EXPORT}` was not found; leaving VFS exports unchanged"
            );
            return Ok(input_wasm.to_vec());
        };
        let Some(target_handoff_global) = target_handoff_global else {
            log::warn!(
                "export-stack isolation requested for VFS, but `{TARGET_HANDOFF_EXPORT}` was not found; leaving VFS exports unchanged"
            );
            return Ok(input_wasm.to_vec());
        };
        let Some(realloc) = realloc else {
            log::warn!(
                "export-stack isolation requested for VFS, but `cabi_realloc` was not found; leaving VFS exports unchanged"
            );
            return Ok(input_wasm.to_vec());
        };
        if !shared_memory {
            log::warn!(
                "export-stack isolation requested for VFS with non-shared memory; leaving VFS exports unchanged"
            );
            return Ok(input_wasm.to_vec());
        }

        let mut exports = Vec::new();
        for (name, kind, index) in &raw_exports {
            if matches!(kind, ExternalKind::Func | ExternalKind::FuncExact)
                && (should_protect(name) || is_thread_start(name))
            {
                let type_index = *function_types
                    .get(*index as usize)
                    .wrap_err_with(|| format!("missing function type for export `{name}`"))?;
                exports.push(ExportedFunction {
                    name: name.clone(),
                    original_index: *index,
                    type_index,
                    thread_start: is_thread_start(name),
                });
            }
        }

        if exports.is_empty() {
            log::warn!(
                "export-stack isolation requested for VFS, but no protectable exports exist"
            );
            return Ok(input_wasm.to_vec());
        }

        let ensure_type = types
            .iter()
            .position(|ty| {
                matches!(
                    &ty.composite_type.inner,
                    CompositeInnerType::Func(func)
                        if func.params().is_empty()
                            && func.results() == [wasmparser::ValType::I32]
                )
            })
            .map(|index| index as u32)
            .unwrap_or(types.len() as u32);
        let append_ensure_type = ensure_type == types.len() as u32;
        let claim_type = types
            .iter()
            .position(|ty| {
                matches!(
                    &ty.composite_type.inner,
                    CompositeInnerType::Func(func)
                        if func.params()
                            == [wasmparser::ValType::I32, wasmparser::ValType::I32]
                            && func.results() == [wasmparser::ValType::I64]
                )
            })
            .map(|index| index as u32)
            .unwrap_or(types.len() as u32 + u32::from(append_ensure_type));
        let append_claim_type = claim_type >= types.len() as u32;

        let first_new_global = import_global_count + local_global_count;
        let state_global = first_new_global;
        let current_base_global = first_new_global + 1;
        let current_end_global = first_new_global + 2;
        let depth_global = first_new_global + 3;
        let generation_global = first_new_global + 4;

        let first_new_func = import_func_count + defined_func_count;
        let ensure_index = first_new_func;
        let claim_index = first_new_func + 1;
        let wrapper_indices = exports
            .iter()
            .enumerate()
            .map(|(index, export)| (export.name.clone(), first_new_func + 2 + index as u32))
            .collect::<HashMap<_, _>>();

        let mut module = Module::new();
        let mut saw_global_section = false;
        let mut saw_export_section = false;
        let mut saw_code_section = false;

        for payload in wasmparser::Parser::new(0).parse_all(input_wasm) {
            let payload = payload?;
            match payload {
                Payload::TypeSection(section) => {
                    let mut output = TypeSection::new();
                    for group in section {
                        let group = group?;
                        if group.is_explicit_rec_group() {
                            output.ty().rec(
                                group
                                    .into_types()
                                    .map(|ty| translate_sub_type(&ty, &DefaultRebinder)),
                            );
                        } else {
                            for ty in group.into_types() {
                                output
                                    .ty()
                                    .subtype(&translate_sub_type(&ty, &DefaultRebinder));
                            }
                        }
                    }
                    if append_ensure_type {
                        output.ty().function([], [ValType::I32]);
                    }
                    if append_claim_type {
                        output
                            .ty()
                            .function([ValType::I32, ValType::I32], [ValType::I64]);
                    }
                    module.section(&output);
                }
                Payload::FunctionSection(section) => {
                    let mut output = FunctionSection::new();
                    for function in section {
                        output.function(function?);
                    }
                    output.function(ensure_type);
                    output.function(claim_type);
                    for export in &exports {
                        output.function(export.type_index);
                    }
                    module.section(&output);
                }
                Payload::GlobalSection(section) => {
                    saw_global_section = true;
                    let mut output = GlobalSection::new();
                    for global in section {
                        let global = global?;
                        let mut instructions = Vec::new();
                        for operator in global.init_expr.get_operators_reader() {
                            let operator = operator?;
                            if !matches!(operator, wasmparser::Operator::End) {
                                instructions.push(translate(&operator, &DefaultRebinder));
                            }
                        }
                        output.global(
                            translate_global_type(global.ty, &DefaultRebinder),
                            &ConstExpr::extended(instructions),
                        );
                    }
                    for _ in 0..5 {
                        output.global(
                            GlobalType {
                                val_type: ValType::I32,
                                mutable: true,
                                shared: false,
                            },
                            &ConstExpr::i32_const(0),
                        );
                    }
                    module.section(&output);
                }
                Payload::ExportSection(section) => {
                    saw_export_section = true;
                    let mut output = ExportSection::new();
                    for export in section {
                        let export = export?;
                        let index = wrapper_indices
                            .get(export.name)
                            .copied()
                            .unwrap_or(export.index);
                        output.export(export.name, wasm_export_kind(export.kind), index);
                    }
                    output.export(ENSURE_EXPORT, ExportKind::Func, ensure_index);
                    output.export(CLAIM_TARGET_EXPORT, ExportKind::Func, claim_index);
                    module.section(&output);
                }
                Payload::CodeSectionStart { .. } => {
                    saw_code_section = true;
                    let mut output = CodeSection::new();
                    // Entries are consumed by the following parser payloads.
                    for nested in wasmparser::Parser::new(0).parse_all(input_wasm) {
                        if let Payload::CodeSectionEntry(body) = nested? {
                            output.raw(&input_wasm[body.range().start..body.range().end]);
                        }
                    }
                    output.function(&build_ensure_function(
                        stack_size,
                        handoff_global,
                        stack_pointer,
                        realloc,
                        state_global,
                        current_base_global,
                        current_end_global,
                        generation_global,
                    ));
                    output.function(&build_claim_target_function(target_handoff_global, realloc));
                    for export in &exports {
                        let function_type =
                            match &types[export.type_index as usize].composite_type.inner {
                                CompositeInnerType::Func(function_type) => function_type,
                                _ => eyre::bail!(
                                    "export `{}` does not have a function type",
                                    export.name
                                ),
                            };
                        output.function(&build_wrapper(
                            export,
                            function_type,
                            ensure_index,
                            state_global,
                            depth_global,
                        ));
                    }
                    module.section(&output);
                }
                Payload::CodeSectionEntry(_) => {}
                Payload::CustomSection(section) => {
                    module.section(&wasm_encoder::CustomSection {
                        name: section.name().into(),
                        data: std::borrow::Cow::Borrowed(section.data()),
                    });
                }
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        module.section(&RawSection {
                            id,
                            data: &input_wasm[range],
                        });
                    }
                }
            }
        }

        if !saw_global_section || !saw_export_section || !saw_code_section {
            eyre::bail!("VFS module is missing a required global, export, or code section");
        }

        Ok(module.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::{MemorySection, MemoryType};

    fn fixture() -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.ty().function(
            [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
            [ValType::I32],
        );
        types.ty().function([ValType::I32], [ValType::I32]);
        types.ty().function([ValType::I32, ValType::I32], []);
        module.section(&types);

        let mut functions = FunctionSection::new();
        functions.function(0);
        functions.function(1);
        functions.function(2);
        module.section(&functions);

        let mut memories = MemorySection::new();
        memories.memory(MemoryType {
            minimum: 2,
            maximum: Some(8),
            memory64: false,
            shared: true,
            page_size_log2: None,
        });
        module.section(&memories);

        let mut globals = GlobalSection::new();
        globals.global(
            GlobalType {
                val_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            &ConstExpr::i32_const(65536),
        );
        globals.global(
            GlobalType {
                val_type: ValType::I32,
                mutable: false,
                shared: false,
            },
            &ConstExpr::i32_const(64),
        );
        globals.global(
            GlobalType {
                val_type: ValType::I32,
                mutable: false,
                shared: false,
            },
            &ConstExpr::i32_const(0),
        );
        module.section(&globals);

        let mut exports = ExportSection::new();
        exports.export("__stack_pointer", ExportKind::Global, 0);
        exports.export(HANDOFF_EXPORT, ExportKind::Global, 1);
        exports.export(TARGET_HANDOFF_EXPORT, ExportKind::Global, 2);
        exports.export("cabi_realloc", ExportKind::Func, 0);
        exports.export("run", ExportKind::Func, 1);
        exports.export("wasi_thread_start", ExportKind::Func, 2);
        module.section(&exports);

        let mut code = CodeSection::new();
        let mut realloc = Function::new([]);
        realloc.instruction(&Instruction::I32Const(1024));
        realloc.instruction(&Instruction::End);
        code.function(&realloc);
        let mut run = Function::new([]);
        run.instruction(&Instruction::LocalGet(0));
        run.instruction(&Instruction::I32Const(1));
        run.instruction(&Instruction::I32Add);
        run.instruction(&Instruction::End);
        code.function(&run);
        let mut thread_start = Function::new([]);
        thread_start.instruction(&Instruction::End);
        code.function(&thread_start);
        module.section(&code);

        module.finish()
    }

    #[test]
    fn generated_vfs_handoff_module_validates() -> Result<()> {
        let output = ExportStackPreVfsStreamPass::new(Some(65536)).run(&fixture())?;
        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
            .validate_all(&output)?;

        let mut run_index = None;
        let mut thread_start_index = None;
        let mut ensure_index = None;
        for payload in wasmparser::Parser::new(0).parse_all(&output) {
            if let Payload::ExportSection(section) = payload? {
                for export in section {
                    let export = export?;
                    match export.name {
                        "run" => run_index = Some(export.index),
                        "wasi_thread_start" => thread_start_index = Some(export.index),
                        ENSURE_EXPORT => ensure_index = Some(export.index),
                        _ => {}
                    }
                }
            }
        }

        assert_eq!(ensure_index, Some(3));
        assert_eq!(run_index, Some(5));
        assert_eq!(thread_start_index, Some(6));
        Ok(())
    }
}
