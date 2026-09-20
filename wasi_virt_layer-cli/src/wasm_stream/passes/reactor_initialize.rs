//! Resolve and fold the official reactor initializer without copying its body.
use eyre::{Result, ensure};
use wasm_encoder::{BlockType, Function, Instruction as I, MemArg};
use wasmparser::{CompositeInnerType, ExternalKind, Operator, Parser, Payload, TypeRef, ValType};

pub const STATE_EXPORT: &str = "__wasip1_vfs_reactor_init_state";
/// Reserved provenance/optimization anchor, never a Component reactor export.
pub const INITIALIZE_EXPORT: &str = "__wvl_reactor_initialize";
pub const PREPARED_STATE_EXPORT: &str = "__wvl_reactor_init_state";

/// Validated original indices; only the function index needs post-combine rebinding.
#[derive(Debug)]
pub struct ReactorInitializer {
    pub function_index: u32,
    state_global: u32,
}

impl ReactorInitializer {
    /// Validate a `() -> ()` reactor with WVL's shared state ABI. Call on the VFS
    /// alone before merging (`prepared = false`), then on its reserved markers
    /// after merging (`prepared = true`). No stale function-count ownership test
    /// is used. The state word is linker-allocated, never borrowed from libc.
    pub fn resolve(wasm: &[u8], prepared: bool) -> Result<Option<Self>> {
        let initialize_name = if prepared {
            INITIALIZE_EXPORT
        } else {
            "_initialize"
        };
        let state_name = if prepared {
            PREPARED_STATE_EXPORT
        } else {
            STATE_EXPORT
        };
        let mut initialize = None;
        let mut state = None;
        let mut command = false;
        let mut function_types = Vec::new();
        let mut void_types = Vec::new();
        let mut imported_functions = 0;
        let mut globals = Vec::new();
        let mut memory_bytes = None;
        let mut other_exports = Vec::new();

        for payload in Parser::new(0).parse_all(wasm) {
            match payload? {
                Payload::TypeSection(s) => {
                    for group in s {
                        for ty in group?.into_types() {
                            void_types.push(matches!(ty.composite_type.inner,
                                CompositeInnerType::Func(ref f) if f.params().is_empty() && f.results().is_empty()));
                        }
                    }
                }
                Payload::ImportSection(s) => {
                    for group in s {
                        for import in group? {
                            match import?.1.ty {
                                TypeRef::Func(ty) => {
                                    function_types.push(ty);
                                    imported_functions += 1;
                                }
                                TypeRef::Global(_) => globals.push(None),
                                TypeRef::Memory(m) => {
                                    memory_bytes
                                        .get_or_insert(m.initial << m.page_size_log2.unwrap_or(16));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Payload::FunctionSection(s) => {
                    for ty in s {
                        function_types.push(ty?);
                    }
                }
                Payload::MemorySection(s) => {
                    for m in s {
                        let m = m?;
                        memory_bytes.get_or_insert(m.initial << m.page_size_log2.unwrap_or(16));
                    }
                }
                Payload::GlobalSection(s) => {
                    for g in s {
                        let g = g?;
                        let mut ops = g.init_expr.get_operators_reader();
                        let address = match ops.read()? {
                            Operator::I32Const { value }
                                if !g.ty.mutable
                                    && g.ty.content_type == ValType::I32
                                    && matches!(ops.read()?, Operator::End) =>
                            {
                                Some(value as u32)
                            }
                            _ => None,
                        };
                        globals.push(address);
                    }
                }
                Payload::ExportSection(s) => {
                    for e in s {
                        let e = e?;
                        match e.name {
                            name if name == initialize_name => {
                                ensure!(initialize.is_none(), "duplicate _initialize export");
                                ensure!(
                                    e.kind == ExternalKind::Func,
                                    "_initialize must be a function export"
                                );
                                initialize = Some(e.index);
                            }
                            name if name == state_name => {
                                ensure!(
                                    state.is_none(),
                                    "duplicate reactor initialization state export"
                                );
                                ensure!(
                                    e.kind == ExternalKind::Global,
                                    "reactor initialization state must be a global address"
                                );
                                state = Some(e.index);
                            }
                            "_start" => command = true,
                            _ if e.kind == ExternalKind::Func => other_exports.push(e.index),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        let Some(function_index) = initialize else {
            return Ok(None);
        };
        ensure!(!command, "unexpected _initialize alongside command _start");
        ensure!(
            function_index >= imported_functions,
            "_initialize must be a defined VFS function"
        );
        let ty = function_types
            .get(function_index as usize)
            .and_then(|ty| void_types.get(*ty as usize));
        ensure!(
            ty == Some(&true),
            "_initialize must have signature () -> ()"
        );
        ensure!(
            !other_exports.contains(&function_index),
            "unexpected alias of _initialize"
        );
        let state_global = state.ok_or_else(|| eyre::eyre!(
            "folding _initialize requires {STATE_EXPORT}; rebuild the VFS with the matching wasi_virt_layer library"))?;
        let address = globals
            .get(state_global as usize)
            .copied()
            .flatten()
            .ok_or_else(|| {
                eyre::eyre!("reactor initialization state must be a defined immutable i32 address")
            })?;
        ensure!(
            address % 4 == 0 && memory_bytes.is_some_and(|size| u64::from(address) + 4 <= size),
            "reactor initialization state must be aligned and inside VFS memory"
        );

        // Reject pre-existing calls rather than inserting a second start call.
        // This also makes re-running the pass on an unexpected input fail closed.
        for payload in Parser::new(0).parse_all(wasm) {
            let payload = payload?;
            if let Payload::ElementSection(s) = &payload {
                for element in s.clone() {
                    match element?.items {
                        wasmparser::ElementItems::Functions(indices) => {
                            for index in indices {
                                ensure!(
                                    index? != function_index,
                                    "unexpected table reference to _initialize"
                                );
                            }
                        }
                        wasmparser::ElementItems::Expressions(_, expressions) => {
                            for expression in expressions {
                                for op in expression?.get_operators_reader() {
                                    ensure!(
                                        !matches!(op?, Operator::RefFunc { function_index: i } if i == function_index),
                                        "unexpected table reference to _initialize"
                                    );
                                }
                            }
                        }
                    }
                }
            }
            if let Payload::CodeSectionEntry(body) = payload {
                for op in body.get_operators_reader()? {
                    ensure!(
                        !matches!(op?, Operator::Call { function_index: i }
                        | Operator::ReturnCall { function_index: i }
                        | Operator::RefFunc { function_index: i } if i == function_index),
                        "unexpected existing reference to _initialize; cannot fold twice"
                    );
                }
            }
        }
        Ok(Some(Self {
            function_index,
            state_global,
        }))
    }

    /// Emit one direct call, elected across shared instances. Publish completion
    /// only after constructors return; workers must not enter target starts early.
    /// A trapping initializer poisons this shared runtime (state remains 1).
    pub fn emit(&self, f: &mut Function, rebind: impl Fn(u32) -> u32) {
        let mem = MemArg {
            offset: 0,
            align: 2,
            memory_index: 0,
        };
        for i in [
            I::GlobalGet(self.state_global),
            I::I32Const(0),
            I::I32Const(1),
            I::I32AtomicRmwCmpxchg(mem),
            I::I32Eqz,
            I::If(BlockType::Empty),
            I::Call(rebind(self.function_index)),
            I::GlobalGet(self.state_global),
            I::I32Const(2),
            I::I32AtomicStore(mem),
            I::GlobalGet(self.state_global),
            I::I32Const(-1),
            I::MemoryAtomicNotify(mem),
            I::Drop,
            I::Else,
            I::Block(BlockType::Empty),
            I::Loop(BlockType::Empty),
            I::GlobalGet(self.state_global),
            I::I32AtomicLoad(mem),
            I::I32Const(2),
            I::I32Eq,
            I::BrIf(1),
            I::GlobalGet(self.state_global),
            I::I32Const(1),
            I::I64Const(-1),
            I::MemoryAtomicWait32(mem),
            I::Drop,
            I::Br(0),
            I::End,
            I::End,
            I::End,
        ] {
            f.instruction(&i);
        }
    }
}
