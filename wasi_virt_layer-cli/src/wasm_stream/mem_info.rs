/// Describes how many value operands sit atop the address on the wasm value stack
/// and the types involved, so the multi-memory lowering pass knows how to
/// save/restore them when it rewrites the address.
///
/// The tuple is `(memory_index, kind, val_types)` where:
/// - `memory_index`: which memory the instruction references
/// - `kind`: the category of memory operation
/// - `val_types`: the types of the value operands above the address, in
///   stack order (bottom to top). For a load this is empty; for a store/rmw
///   it is `[val_ty]`; for cmpxchg it is `[expected_ty, replacement_ty]`;
///   for wait it is `[expected_ty, timeout_ty]`; for notify it is `[count_ty]`.
///
/// The result type (what the instruction pushes) is given separately as the last
/// element so the lowering pass can save it into the right temp local after the
/// instruction executes (needed for lock-release in threaded mode).
pub fn memory_op_info<'a>(op: &wasmparser::Operator<'a>) -> Option<MemoryOpInfo> {
    use wasm_encoder::ValType::*;
    match op {
        // --- plain loads: [addr] -> [val] ---
        wasmparser::Operator::I32Load { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64Load { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::F32Load { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(F32),
        }),
        wasmparser::Operator::F64Load { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(F64),
        }),
        wasmparser::Operator::I32Load8S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32Load8U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32Load16S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32Load16U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64Load8S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64Load8U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64Load16S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64Load16U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64Load32S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64Load32U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),

        // --- plain stores: [addr, val] -> [] ---
        wasmparser::Operator::I32Store { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: None,
        }),
        wasmparser::Operator::I64Store { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),
        wasmparser::Operator::F32Store { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![F32],
            result_type: None,
        }),
        wasmparser::Operator::F64Store { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![F64],
            result_type: None,
        }),
        wasmparser::Operator::I32Store8 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: None,
        }),
        wasmparser::Operator::I32Store16 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: None,
        }),
        wasmparser::Operator::I64Store8 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),
        wasmparser::Operator::I64Store16 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),
        wasmparser::Operator::I64Store32 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),

        // --- memory.atomic.notify: [addr, count:i32] -> [result:i32] ---
        wasmparser::Operator::MemoryAtomicNotify { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),

        // --- memory.atomic.wait32: [addr, expected:i32, timeout:i64] -> [result:i32] ---
        wasmparser::Operator::MemoryAtomicWait32 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32, I64],
            result_type: Some(I32),
        }),
        // --- memory.atomic.wait64: [addr, expected:i64, timeout:i64] -> [result:i32] ---
        wasmparser::Operator::MemoryAtomicWait64 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64, I64],
            result_type: Some(I32),
        }),

        // --- atomic loads: [addr] -> [val] ---
        wasmparser::Operator::I32AtomicLoad { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicLoad { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicLoad8U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicLoad16U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicLoad8U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicLoad16U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicLoad32U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(I64),
        }),

        // --- atomic stores: [addr, val] -> [] ---
        wasmparser::Operator::I32AtomicStore { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: None,
        }),
        wasmparser::Operator::I64AtomicStore { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),
        wasmparser::Operator::I32AtomicStore8 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: None,
        }),
        wasmparser::Operator::I32AtomicStore16 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: None,
        }),
        wasmparser::Operator::I64AtomicStore8 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),
        wasmparser::Operator::I64AtomicStore16 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),
        wasmparser::Operator::I64AtomicStore32 { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: None,
        }),

        // --- atomic RMW (read-modify-write): [addr, val] -> [old_val] ---
        wasmparser::Operator::I32AtomicRmwAdd { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwAdd { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8AddU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16AddU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8AddU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16AddU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32AddU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmwSub { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwSub { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8SubU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16SubU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8SubU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16SubU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32SubU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmwAnd { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwAnd { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8AndU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16AndU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8AndU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16AndU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32AndU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmwOr { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwOr { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8OrU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16OrU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8OrU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16OrU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32OrU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmwXor { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwXor { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8XorU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16XorU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8XorU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16XorU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32XorU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmwXchg { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwXchg { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8XchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16XchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8XchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16XchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32XchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64],
            result_type: Some(I64),
        }),

        // --- atomic cmpxchg: [addr, expected, replacement] -> [old_val] ---
        wasmparser::Operator::I32AtomicRmwCmpxchg { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32, I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmwCmpxchg { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64, I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I32AtomicRmw8CmpxchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32, I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I32AtomicRmw16CmpxchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I32, I32],
            result_type: Some(I32),
        }),
        wasmparser::Operator::I64AtomicRmw8CmpxchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64, I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw16CmpxchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64, I64],
            result_type: Some(I64),
        }),
        wasmparser::Operator::I64AtomicRmw32CmpxchgU { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![I64, I64],
            result_type: Some(I64),
        }),

        // --- SIMD loads: [addr] -> [v128] ---
        wasmparser::Operator::V128Load { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load8x8S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load8x8U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load16x4S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load16x4U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load32x2S { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load32x2U { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load8Splat { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load16Splat { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load32Splat { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load64Splat { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load32Zero { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),
        wasmparser::Operator::V128Load64Zero { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![],
            result_type: Some(V128),
        }),

        // --- SIMD store: [addr, v128] -> [] ---
        wasmparser::Operator::V128Store { memarg } => Some(MemoryOpInfo {
            memory: memarg.memory,
            value_operands: vec![V128],
            result_type: None,
        }),

        _ => None,
    }
}

/// Information about a memory-accessing instruction's operand layout.
pub struct MemoryOpInfo {
    /// Which memory index this instruction references.
    pub memory: u32,
    /// The types of value operands above the address on the stack,
    /// ordered bottom-to-top.  Empty for pure loads.
    pub value_operands: Vec<wasm_encoder::ValType>,
    /// The result type pushed onto the stack, if any.
    /// `None` for pure stores.
    pub result_type: Option<wasm_encoder::ValType>,
}

/// Given a `wasm_encoder::Instruction` that references a memory, set its memory
/// index to 0.  Used after multi-memory lowering so that all instructions target
/// the single remaining memory.
pub fn clear_memory_index(op: &mut wasm_encoder::Instruction) {
    match op {
        wasm_encoder::Instruction::I32Load(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::F32Load(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::F64Load(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Load8S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Load8U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Load16S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Load16U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load8S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load8U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load16S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load16U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load32S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Load32U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Store(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Store(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::F32Store(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::F64Store(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Store8(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32Store16(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Store8(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Store16(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64Store32(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::MemoryAtomicNotify(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::MemoryAtomicWait32(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::MemoryAtomicWait64(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicLoad(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicLoad(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicLoad8U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicLoad16U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicLoad8U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicLoad16U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicLoad32U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicStore(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicStore(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicStore8(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicStore16(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicStore8(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicStore16(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicStore32(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwAdd(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwAdd(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8AddU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16AddU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8AddU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16AddU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32AddU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwSub(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwSub(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8SubU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16SubU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8SubU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16SubU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32SubU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwAnd(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwAnd(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8AndU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16AndU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8AndU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16AndU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32AndU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwOr(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwOr(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8OrU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16OrU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8OrU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16OrU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32OrU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwXor(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwXor(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8XorU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16XorU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8XorU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16XorU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32XorU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwXchg(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwXchg(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8XchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16XchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8XchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16XchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32XchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmwCmpxchg(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmwCmpxchg(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw8CmpxchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I32AtomicRmw16CmpxchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw8CmpxchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw16CmpxchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::I64AtomicRmw32CmpxchgU(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load8x8S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load8x8U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load16x4S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load16x4U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load32x2S(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load32x2U(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load8Splat(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load16Splat(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load32Splat(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load64Splat(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load32Zero(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Load64Zero(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::V128Store(memarg) => memarg.memory_index = 0,
        wasm_encoder::Instruction::MemorySize(mem) => *mem = 0,
        wasm_encoder::Instruction::MemoryGrow(mem) => *mem = 0,
        wasm_encoder::Instruction::MemoryInit { mem, .. } => *mem = 0,
        wasm_encoder::Instruction::MemoryCopy { dst_mem, src_mem } => {
            *dst_mem = 0;
            *src_mem = 0;
        }
        wasm_encoder::Instruction::MemoryFill(mem) => *mem = 0,
        wasm_encoder::Instruction::MemoryDiscard(mem) => *mem = 0,
        _ => {}
    }
}

/// Returns the temp local index for a given ValType, using the temp local
/// allocation layout from the multi-memory lowering pass.
pub fn temp_local_for_type(
    val_ty: wasm_encoder::ValType,
    tmp_i32: u32,
    tmp_i64: u32,
    tmp_f32: u32,
    tmp_f64: u32,
    tmp_v128: u32,
) -> u32 {
    match val_ty {
        wasm_encoder::ValType::I32 => tmp_i32,
        wasm_encoder::ValType::I64 => tmp_i64,
        wasm_encoder::ValType::F32 => tmp_f32,
        wasm_encoder::ValType::F64 => tmp_f64,
        wasm_encoder::ValType::V128 => tmp_v128,
        _ => unreachable!(),
    }
}
