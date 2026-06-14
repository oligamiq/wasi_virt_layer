# Export Stack Handoff Design

## Status

Draft design for protecting exported Wasm functions from stack collisions when
multiple instances share linear memory.

This document records the current design discussion. It is not an implementation
specification yet. In particular, the single-memory `memory.grow` restrictions
must be resolved before implementation is considered complete.

## Problem

Rust/LLVM normally emits a mutable `__stack_pointer` global. Separate Wasm
instances have separate globals, but instances may share the same linear memory.

Before `wasi_thread_start` runs, multiple instances can therefore hold the same
initial stack-pointer value and access the same physical stack area. Protected
exports need an instance-specific stack before entering ordinary Rust code.

The generated wrapper itself must remain stackless until the stack switch has
completed.

## Central Rule

This design does not let every instance independently allocate its own first
stack from the original shared stack.

Instead, each dynamically managed module has a shared `next_stack`:

1. A caller atomically takes the prepared `next_stack`.
2. It switches to that stack.
3. While running on the newly acquired valid stack, it allocates and publishes
   the next caller's `next_stack`.
4. The acquired stack remains assigned to that instance until explicitly
   released.

Every time a prepared stack is consumed, its replacement is prepared
immediately. This handoff and replenishment rule is the main invariant.

Subsequent protected exports from an instance that already owns a valid stack do
not allocate another stack.

## Terminology

- **Bootstrap stack**: The compiler-provided initial stack. It may be shared
  physically by multiple instances and may only be used inside the serialized
  bootstrap path.
- **Current stack**: The stack currently assigned to one Wasm instance.
- **Next stack**: A fully allocated standby stack waiting for the next instance.
- **Dynamic stack**: A stack allocated by the VFS allocator.
- **Fixed arena**: A build-time reserved area in a target memory.
- **Slot**: One stack-sized section of a fixed arena.
- **Thread-managed stack**: A stack installed by compiler-generated
  `wasi_thread_start`.
- **Protected export**: An export whose generated wrapper ensures a valid stack
  before calling the original function.

## Shared and Instance-Local State

### Shared state

Dynamic management requires one shared record per Wasm module:

```text
bootstrap_lock
next_stack_base
next_stack_end
next_stack_size
generation
```

The record resides in shared VFS memory. Access to `next_stack` is serialized.
Publication must use atomic operations with acquire/release ordering.

Multi-memory target arenas additionally require:

```text
slot_bitmap
slot_count
slot_size
```

### Instance-local state

Each Wasm instance receives mutable globals:

```text
stack_state
current_stack_base
current_stack_end
active_export_depth
generation
slot_index       // multi-memory target only
```

`stack_state` has the following values:

```text
UNINITIALIZED
INITIALIZING
READY
THREAD_MANAGED
RELEASING
```

These values must not be stored only in shared linear memory. Different
instances sharing memory need different current-stack state.

## Stackless Wrapper Requirement

Code before the stack switch must be generated directly as Wasm instructions.
It must not call a normal Rust function or use the Rust allocator.

The pre-switch path may only:

- Read and write globals.
- Execute atomic loads, stores, compare-and-swap, wait, and notify.
- Load the prepared stack coordinates.
- Set `__stack_pointer`.
- Branch to generated helpers that are proven stackless.

Allocator calls are permitted only:

1. In the serialized initial bootstrap case, where the original VFS stack is
   known to exist.
2. After switching to an already prepared valid stack.

## Dynamic Handoff Algorithm

The VFS and single-memory targets use the following conceptual algorithm.

```text
ensure_stack(module):
    if state == THREAD_MANAGED or state == READY:
        return success

    acquire module.bootstrap_lock

    if state changed to THREAD_MANAGED or READY:
        release lock
        return success

    state = INITIALIZING

    if module.next_stack is empty:
        // Only the initial serialized bootstrap path reaches this case.
        current = allocate_with_vfs_allocator(configured_stack_size)
    else:
        current = take(module.next_stack)

    if allocation failed:
        state = UNINITIALIZED
        release lock
        return allocation failure

    install current as this instance's stack
    state = READY

    // Execution from this point uses the newly installed valid stack.
    replacement = allocate_with_vfs_allocator(configured_stack_size)

    if replacement failed:
        // Do not publish an invalid next stack.
        // Keep the current instance valid, but report exhaustion so another
        // instance is not allowed to fall back to the shared bootstrap stack.
        module.next_stack = empty
        release lock
        return standby allocation failure

    publish replacement as module.next_stack
    release lock
    return success
```

The implementation may split this into a stackless pre-switch helper and a
normal post-switch replenishment helper.

No path may silently fall back to the original shared stack after standby
allocation fails.

## Protected Export Algorithm

Every selected export except `wasi_thread_start` is renamed and wrapped:

```text
protected_export(args...):
    if state != READY and state != THREAD_MANAGED:
        ensure_vfs_stack()
        ensure_this_module_stack()

    active_export_depth += 1
    result = original_export(args...)
    active_export_depth -= 1
    return result
```

For VFS exports, `ensure_this_module_stack()` is the VFS operation itself and is
not called twice.

The fast path consists of an instance-local state check, depth accounting, and
the original call.

Trap and `proc_exit` paths may skip depth cleanup. Explicit release must reject
such state unless the user invokes an unsafe force-release operation.

## `wasi_thread_start`

`wasi_thread_start` is never wrapped by the normal export protection wrapper.
The compiler-generated function already installs a stack pointer and TLS.

The generated thread-entry wrapper must mark the instance as thread-managed at
the point where the compiler-generated thread start takes ownership:

```text
state = THREAD_MANAGED
call compiler_generated_wasi_thread_start(thread_id, data_ptr)
```

No dynamic stack or fixed-arena slot is acquired for a thread-managed instance.

## Multi-Memory Design

### VFS memory

The VFS memory has no fixed stack arena:

```text
[ existing VFS data and heap ]
```

VFS stacks use the dynamic handoff algorithm and the existing VFS allocator.
The first allocation is serialized on the bootstrap stack. Each acquired stack
then prepares the next VFS stack.

### Target memory

Each target memory has a fixed arena before its original logical memory:

```text
[ fixed stack arena ][ original target memory ]
```

The arena is divided into fixed-size slots:

```text
[ slot 0 ][ slot 1 ] ... [ slot N-1 ]
```

`slots` applies only to multi-memory target modules. It is invalid for:

- VFS configuration.
- Any single-memory configuration.

All original target memory accesses receive the fixed arena offset:

```text
physical_address = logical_address + arena_size
```

The target stack pointer is stored in the target's logical coordinate system.
For a physical slot end:

```text
logical_stack_pointer = physical_slot_end - arena_size
```

A lowered stack access then reaches the slot:

```text
(logical_stack_pointer - frame_size) + arena_size
    = physical_slot_end - frame_size
```

The arena allocator is a stackless bitmap allocator. Slot acquisition and
release use atomic compare-and-swap. No VFS heap allocation is needed for
multi-memory targets.

`memory.size` exposed to target code must exclude arena pages. `memory.grow`
must preserve the fixed prefix and report only the logical target size.

## Single-Memory Design

### VFS memory

The VFS has no fixed stack arena. Its stacks use the VFS allocator and the
dynamic handoff algorithm.

### Target stacks

Single-memory target stacks also use memory returned by the VFS allocator.
They do not use slots.

If the target's current physical base is `target_base` and the allocated
physical stack end is `stack_end`, the target-facing stack pointer is:

```text
logical_stack_pointer = stack_end - target_base
```

The existing single-memory lowering adds `target_base` to target memory
accesses, producing the allocated physical address.

### Required layout stability

The current lowering can move target bases when an earlier logical memory grows.
An active target function may retain stack-derived logical pointers in locals.
Changing `target_base` during that call invalidates those pointers.

Therefore the initial implementation must enforce:

1. Module base offsets are frozen while protected target calls are active.
2. VFS allocator operations used for stack handoff must not trigger a physical
   layout move during an active target call.
3. If the allocator cannot satisfy a request without such a move, stack
   acquisition fails instead of continuing on the bootstrap stack.

Possible later solutions include a non-moving single-memory layout or a
whole-call layout read lock combined with a reentrancy-safe growth protocol.
Per-memory-operation locking alone is insufficient because stack-derived locals
survive between memory operations.

## Explicit Release

Stacks are permanent by default. Release is an explicit user operation.

### Dynamic VFS stack release

The VFS cannot free the stack on which the release function itself is running.
Release must perform another handoff:

```text
acquire bootstrap_lock
old = current_stack
replacement = take(next_stack)
install replacement as current_stack
allocate and publish a new next_stack
free old using the VFS allocator
release bootstrap_lock
```

The switch must occur before calling the allocator to free `old`.

### Single-memory target release

Release is allowed only when:

```text
state == READY
active_export_depth == 0
generation matches
```

The target is marked uninitialized and its dynamic allocation is returned to the
VFS allocator from a valid VFS stack.

### Multi-memory target release

The same state checks apply. The target's slot bit is cleared atomically. The
fixed arena itself remains allocated as part of the module memory.

### Force release

Force release is unsafe. It may only be used when the caller externally
guarantees that no execution can still access the stack.

## Configuration

Proposed CLI configuration:

```text
--stack-size vfs=1MiB
--stack-size target_a=2MiB
--stack-size target_b=512KiB

# Valid only for multi-memory targets.
--stack-slots target_a=32
--stack-slots target_b=8
```

Module configuration also controls:

```text
enabled
stack_size
protected_exports
excluded_exports
allow_explicit_release
```

Defaults:

- `wasi_thread_start` is always excluded.
- Acquired stacks are not automatically released.
- An instance that already has `READY` or `THREAD_MANAGED` state does not
  acquire another stack.

Supplying `--stack-slots` for VFS or single-memory mode is a configuration
error.

## Macro Configuration

Proposed macro for module defaults:

```rust
configure_wasm_stack! {
    vfs => {
        size: 1 * 1024 * 1024,
        allow_release: true,
    },
    target_a => {
        size: 2 * 1024 * 1024,
        slots: 32, // Multi-memory target only.
        allow_release: true,
    },
}
```

Proposed export selection:

```rust
protect_wasm_exports! {
    target_a => ["run", "reset", "_main"],
}
```

Proposed exclusions:

```rust
exclude_wasm_stack_exports! {
    target_a => ["wasi_thread_start", "known_stackless_export"],
}
```

Proposed forced selection:

```rust
force_wasm_stack_export! {
    target_a => "custom_entry",
}
```

Macros should emit a custom section consumed by the CLI pipeline. They must not
implement stack switching in ordinary Rust code.

## Public Rust API

The intended high-level API is:

```rust
pub fn ensure_vfs_stack() -> Result<(), StackError>;

pub fn ensure_wasm_stack(
    module: StackModule<'_>,
) -> Result<(), StackError>;

pub fn release_vfs_stack() -> Result<(), StackError>;

pub fn release_wasm_stack(
    module: StackModule<'_>,
    generation: u32,
) -> Result<(), StackError>;

pub fn vfs_stack_info() -> StackInfo;

pub fn wasm_stack_info(
    module: StackModule<'_>,
) -> Result<StackInfo, StackError>;

pub unsafe fn force_release_vfs_stack() -> Result<(), StackError>;

pub unsafe fn force_release_wasm_stack(
    module: StackModule<'_>,
) -> Result<(), StackError>;
```

Supporting types:

```rust
pub enum StackModule<'a> {
    Vfs,
    Target(&'a str),
}

pub enum StackState {
    Uninitialized,
    Initializing,
    Ready,
    ThreadManaged,
    Releasing,
}

pub struct StackInfo {
    pub state: StackState,
    pub stack_size: u32,
    pub active_export_depth: u32,
    pub generation: u32,
    pub current_stack_base: u32,
    pub current_stack_end: u32,
    pub next_stack_ready: bool,
    pub slot_index: Option<u32>,
}

pub enum StackError {
    UnknownModule,
    StackPointerNotFound,
    AllocationFailed,
    StandbyAllocationFailed,
    NoFreeSlot,
    InUse,
    ThreadManaged,
    GenerationMismatch,
    LayoutMoveRequired,
    InvalidConfiguration,
}
```

## Generated Low-Level Exports

Required generic exports:

```text
__wasip1_vfs_stack_ensure_vfs() -> errno
__wasip1_vfs_stack_ensure(module_id) -> errno

__wasip1_vfs_stack_release_vfs(generation) -> errno
__wasip1_vfs_stack_release(module_id, generation) -> errno

__wasip1_vfs_stack_info_vfs(result_ptr) -> errno
__wasip1_vfs_stack_info(module_id, result_ptr) -> errno

__wasip1_vfs_stack_force_release_vfs() -> errno
__wasip1_vfs_stack_force_release(module_id) -> errno
```

Optional module-specific convenience exports:

```text
__wasip1_vfs_<module>_stack_ensure() -> errno
__wasip1_vfs_<module>_stack_release(generation) -> errno
__wasip1_vfs_<module>_stack_info(result_ptr) -> errno
```

The force-release exports should only be emitted when explicitly enabled.

## Stack-Pointer Detection

Detection should attempt:

1. An explicitly exported `__stack_pointer`.
2. Linking or name custom-section metadata.
3. Compiler-generated `wasi_thread_start` instruction patterns.
4. Explicit user configuration.

If no stack pointer is found, the CLI warns and leaves the module unchanged, as
in the current behavior requested for compatibility. A future strict option may
turn this warning into a build error.

## Validated Preconditions

The following baseline checks were performed on 2026-06-14.

### Negative `i32` stack pointers

A negative `i32` value is not intrinsically invalid as a Wasm pointer. Wasm32
uses the 32-bit value as an address bit pattern. The generated lowering uses
`i32.add`, whose result wraps modulo 2^32.

For a module base `B` and physical stack address `P`:

```text
logical_stack_pointer = P - B mod 2^32
physical_address = logical_stack_pointer + B mod 2^32
```

When `P < B`, the logical value is displayed as a negative signed `i32`, but the
second expression still recovers `P`.

This was verified with:

1. A minimal single-memory Wasm using a negative stack pointer, a frame
   subtraction, base addition, store, and load.
2. A generated single-memory Rust core Wasm whose target stack pointer was
   changed from `1,048,576` to `-589,824`. With target base `1,114,112`, this
   selected physical stack address `524,288`.
3. The modified generated core successfully executed Rust `_start`, stack-based
   WASI `fd_write`, thread creation, and the subsequent compiler-generated
   `wasi_thread_start`.
4. A minimal two-memory Wasm using a fixed one-page arena, a negative logical
   target stack pointer, direct target access, and cross-memory copy with the
   arena offset.

### Conditions for negative logical pointers

The successful tests do not prove that negative logical stack addresses are
transparent to every Wasm program. The implementation must enforce or document
the following conditions:

1. This design currently applies only to wasm32. The lowering emits
   `memory64: false` and uses `i32` addresses.
2. Every target dereference must translate the logical address before the
   memory instruction.
3. Translation must use wrapping `i32.add`; it must not reject the logical
   pointer because its signed representation is negative.
4. Bulk-memory operations, atomic operations, SIMD memory operations, data
   initialization, and generated memory helpers must use the same translation.
5. Multi-memory cross-memory helpers such as `memory_copy_from`,
   `memory_copy_to`, and pointer-director functions must add the target arena
   offset. Rewriting only instructions inside the original target module is
   insufficient.
6. The physical stack frame must remain inside its allocated stack region.
   Wrapping must occur only in the logical-coordinate calculation, not across
   the physical beginning or end of the allocated region.
7. Logical pointer plus length calculations must not be validated with a signed
   comparison before translation.

There is also an observable semantic difference: casting a negative logical
stack pointer to Rust `usize` produces a value near 4 GiB. Code that:

- Compares stack and heap addresses.
- Exposes stack addresses as integers.
- Computes pointer differences between unrelated allocations.
- Implements custom stack-bound checks.
- Assumes all valid pointers are below `i32::MAX`.

may behave differently even though dereferences are translated correctly.

Before enabling this mechanism by default, an eligibility scan or explicit
opt-in is required. At minimum, generated/runtime stack-bound code and supported
Rust standard-library versions must be tested. A strict mode should reject
known signed pointer checks involving the detected stack pointer.

### Existing test baseline

The unmodified repository baseline passed:

```text
cargo check -r
cargo nextest run -r --fail-fast
cargo test -r --doc
```

Results:

```text
cargo check: success
nextest: 103 passed, 0 failed, 2 skipped
doctests: 12 passed, 0 failed, 2 ignored
```

These are baseline results only. The handoff implementation does not exist yet,
so the complete suite has not tested the proposed transformation.

### Bootstrap serialization

The first VFS allocation is safe only if every possible use of the shared
bootstrap stack obeys the same serialization rule.

The implementation must guarantee:

1. The combined StartSection is stackless for every transformed module.
2. No ordinary export can enter Rust code before passing through the generated
   stack wrapper.
3. The root instance also acquires its managed VFS stack before executing
   ordinary VFS code concurrently with worker instantiation.
4. The bootstrap lock is acquired by stackless Wasm code.
5. The initial allocator call and stack switch remain inside the locked region.
6. After the switch, no instance returns to the bootstrap stack except through
   the explicit, serialized release protocol.

The generated single-memory threads fixture examined during this validation had
a stackless synthesized StartSection. Its calls performed memory/TLS
initialization, offset initialization, and debug-state initialization without
subtracting either the VFS or target stack pointer.

This result is fixture-specific. The build must analyze the final StartSection
and fail or warn if its transitive call graph can modify or use a detected stack
pointer before stack management is active.

## Pipeline Placement

The likely implementation points are:

1. Pre-target pass:
   - Detect `__stack_pointer`.
   - Add instance-local state globals.
   - Rename selected exports and add wrappers.
   - Add multi-memory target arena metadata.
2. Pre-VFS pass:
   - Add VFS state and stackless handoff anchors.
3. Post-combine pass:
   - Connect VFS allocator calls.
   - Generate shared records and generic public exports.
   - Mark `wasi_thread_start` paths as thread-managed.
4. Single-memory lowering:
   - Apply target base offsets.
   - Preserve dynamic stack coordinate conversion.
   - Enforce layout stability.
5. Multi-memory handling:
   - Insert the fixed target arena.
   - Offset original target accesses.
   - Virtualize target `memory.size` and `memory.grow`.

Import and export names must be documented in
`wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` when implementation
starts.

## Validation Plan

Required tests:

1. Two instances enter the same protected VFS export concurrently before
   `wasi_thread_start`.
2. Each consumer receives a distinct dynamic stack.
3. Consuming a stack publishes a distinct next stack before another consumer
   proceeds.
4. Repeated exports from one instance use its existing current stack.
5. `wasi_thread_start` instances do not consume dynamic stacks or slots.
6. Multi-memory targets acquire distinct fixed-arena slots.
7. Multi-memory slot exhaustion returns an error or traps without bootstrap
   fallback.
8. Single-memory targets translate dynamic physical stack addresses correctly.
9. Single-memory layout movement is rejected while a protected call is active.
10. Explicit release followed by reacquisition works.
11. VFS release switches stacks before freeing the old stack.
12. Trap and `proc_exit` paths prevent normal release.
13. Missing `__stack_pointer` emits a warning and preserves existing behavior.
14. Recursive and nested protected exports do not acquire additional stacks.
15. Negative logical stack pointers survive stack-to-`usize` conversions used
    by supported Rust/WASI runtime code, or the build rejects the module.
16. Multi-memory cross-memory helpers translate negative logical stack
    pointers with the arena offset.
17. Pointer-plus-length checks at ABI boundaries operate on translated physical
    addresses or use wrapping logical arithmetic correctly.
18. The final StartSection and its transitive callees are stackless before stack
    management becomes active.
19. Root and worker instances cannot use the bootstrap stack concurrently.

## Non-Goals

- Copying a live stack to another address.
- Automatically freeing stacks after every export call.
- Using multi-memory target slots for VFS.
- Using slots in single-memory mode.
- Falling back to a potentially shared bootstrap stack after allocation or slot
  exhaustion.
