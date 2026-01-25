# Agent Playbook for `wasi_virt_layer`

This file aggregates all development instructions, workflows, and rules for agents working on `wasi_virt_layer`. It combines information from `GEMINI.md` and developer notes from `README.md`.

## 1. Project Overview
`wasi_virt_layer` provides a virtualization layer for WASI modules, combining a virtual file system (VFS) WASM module with main WASM modules.
- **Core crates:** `wasi_virt_layer` (library), `wasi_virt_layer-cli` (CLI).
- **Workspace root:** `D:/projects/wasi_virt_layer` (Rust 2024, rustc >= 1.89.0).

## 2. Core Mandates & Constraints
- **Conventions First:** Adhere strictly to existing code style, naming, and architecture.
- **No Assumptions:** Verify library usage before importing.
- **Sparse Comments:** Explain *why*, not *what*.
- **Git Safety:**
  - No direct `git add/commit` commands. Propose changes and messages instead.
  - Check `git status`, `git diff`, and `git log` before proposing.
  - Never push without explicit request.
  - Confirm changes to `Cargo.toml`, CI files, or shared configs.
- **Existing Warnings:** Do not fix pre-existing warnings. Only address warnings introduced by your changes.
- **Documentation:** Keep this file (`AGENTS.md`) and `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` updated.

## 3. Build, Run, & Test Commands

### Basic Workflow
| Action | Command | Notes |
| :--- | :--- | :--- |
| **Check** | `cargo check -r` | Workspace-wide, release profile |
| **Test (Nextest)** | `cargo nextest run -r --fail-fast` | Recommended for fast fail verification |
| **Run CLI** | `cargo r -r -- <args>` | Preferred method for running the tool |
| **Format** | `cargo fmt` | Standard rustfmt |
| **Lint** | `cargo clippy --all-targets --all-features -D warnings` | |

### Specific Test Patterns
- **Library only:** `cargo check -p wasi_virt_layer`
- **CLI only:** `cargo check -p wasi_virt_layer-cli`
- **Single Integration Test:** `cargo test -r -p wasi_virt_layer-cli --test <file_stem>`
- **Filter Test Case:** `cargo test -r -p wasi_virt_layer-cli --test <file_stem> <test_fn_substring>`
- **Examples (Short run):** `cargo test -r -p wasi_virt_layer --lib <filter>`

### Example Runs (from README)
```bash
# Threads example (single memory)
cargo r -r -- -p threads_vfs test_threads -t single --threads true

# Threads example (multi memory)
cargo r -- -p threads_vfs test_threads -t multi --threads true
```

## 4. Development Workflow

### 4.1. Task Execution Strategy
1.  **Understand:** Use `grep`, `glob`, and `read` to analyze context.
2.  **Plan:** Create a step-by-step plan.
3.  **Implement:** Edit files, strictly following `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` if touching generators.
4.  **Verify:** Run tests (`cargo nextest run -r --fail-fast`) and linters.
5.  **Refine:** Fix any issues found during verification.

### 4.2. Refactoring Generators
**CRITICAL:** When modifying `generator` modules or ABI logic:
- Consult `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`.
- Ensure import/export names match exactly at each stage.
- Preserve debug generators (`Debug*`) unless explicitly pruned.

### 4.3. Integration Testing Details
Tests live in `wasi_virt_layer-cli/tests/` and use `utils.rs`.
- Use `run_wasi_virt_layer` helper to abstract CLI execution.
- **Output Isolation:** Use `OutDir::Random` to avoid collisions during parallel tests.
- **Artifacts:** Use `keep_build_artifacts: true` in `run_wasi_virt_layer` if you need to debug intermediate WASM files.

## 5. Technical Notes & Troubleshooting

### Feature Flags
- **Workspace:** `std`, `alloc`, `threads`, `multi_memory`, `unstable_print_debug`.
- **Consistency:** Ensure CLI args (e.g., `--threads`) match the manifest feature flags checked by `config_checker`.

### Known Issues & Debugging (from README)
- **Threads & Memory:**
  - `single_memory` may fail where `multi_memory` succeeds in threaded contexts.
  - Pay attention to the order of thread creation:
    - VFS -> Body: (Check status)
    - Body -> Body: Success
    - VFS -> VFS: Success
- **Reset Behavior:**
  - Without `_reset()`, both single and multi memory modes might appear to succeed.
  - With `test_threads::_reset(); _start(); _main();`: Multi-memory succeeds; Single-memory might panic with `unreachable`.
- **Build Caching:** Watch out for build target directory caching (`--no-cache` might be needed).
- **Recursion:** Self-calling binaries (fallback mechanisms) are tricky.
- **CLI Arguments:** Extremely long arguments might cause failures.

### Deno Usage
Generated artifacts (in `dist/`) can often be run with Deno:
```bash
deno run dist/test_run.ts
```

## 6. Roadmap & TODOs
- [ ] Support non-binary Wasm modules
- [ ] Enable specifying multiple Wasm modules
- [ ] Support `self` not passed in `plug_thread!`
- [ ] Support `self` binary
- [ ] Support flush sync to file system
- [ ] Fake global allocator / center allocator merged with VFS
- [ ] Access Time Trait
- [ ] Multiple LFS file systems (VFS)
- [ ] Static file system
- [ ] Separate mode (connect function by javascript)
- [ ] Threading VFS with non-threading WASM
- [ ] Validator with error on threads
- [ ] Unicode support
- [ ] Async WIT support

## 7. Non-Goals & Limitations
- **wasm-bindgen:** Not supported because it cannot use WASI.
