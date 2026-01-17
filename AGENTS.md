# Agent Playbook for `wasi_virt_layer`
# Purpose
- Give repo-ready instructions for autonomous agents working here.
- Keep this file in sync with project conventions; update when rules change.

# Quick References
- Workspace root: `D:/projects/wasi_virt_layer` (Rust 2024 edition, rustc >= 1.89.0).
- Core crates: `wasi_virt_layer` (library), `wasi_virt_layer-cli` (CLI), examples under `examples/`.
- Ref docs: `GEMINI.md` (this file must stay consistent), `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` for generator refactors.
- No Cursor/Copilot repo rules found.

# Toolchain & Env
- Rust stable, edition 2024; prefer `rustup override set stable` if needed.
- Uses `color-eyre` + `eyre`; install hooks via `color_eyre::install()` when adding binaries/tests.
- Default features: workspace-wide; crate features: `std`, `alloc`, `threads`, `multi_memory`, `unstable_print_debug` (opt-in, unstable).

# Build & Check
- Workspace check (release profile): `cargo check -r`.
- Library only: `cargo check -p wasi_virt_layer`.
- CLI only: `cargo check -p wasi_virt_layer-cli`.
- Release build CLI (preferred for actual runs): `cargo r -r -- <args>` from `wasi_virt_layer-cli` binary.
- Add `RUSTFLAGS="-Dwarnings"` if you need strictness.

# Format & Lint
- Formatting: default rustfmt (no custom config). Run `cargo fmt` or check with `cargo fmt -- --check`.
- Linting: `cargo clippy --all-targets --all-features -D warnings` (if time is tight, run per-package).
- Keep imports rustfmt-ordered; avoid manual grouping unless rustfmt disagrees.

# Tests
- Full suite (release): `cargo test -r`.
- Package-specific: `cargo test -r -p wasi_virt_layer` or `cargo test -r -p wasi_virt_layer-cli`.
- Single integration test file: `cargo test -r -p wasi_virt_layer-cli --test <file_stem>`.
- Single test case filter: `cargo test -r -p wasi_virt_layer-cli --test <file_stem> <test_fn_substring>`.
- Short run for examples (if needed): `cargo test -r -p wasi_virt_layer --lib <filter>`.
- Tests live mainly in `wasi_virt_layer-cli/tests/`; helpers in `wasi_virt_layer-cli/tests/utils.rs`.

# CLI Usage & Outputs
- Main binary: `wasi_virt_layer` (from `wasi_virt_layer-cli`).
- Typical run: `cargo r -r -- -p <vfs_package_or_component> <wasm_path_or_component>`.
- Flags of note: `--threads <true|false>`, `-t/--target-memory <single|multi>`, `--no-transpile`, `--adjust-abi`, `--dwarf <bool>`, `--keep-build-artifacts`.
- Outputs: components/files under chosen `--out-dir` (default `dist`); may emit JS runner unless `--no-transpile`.

# Workspace Conventions
- Avoid changing workspace `Cargo.toml` (deps, profiles) without explicit user confirmation (per `GEMINI.md`).
- When editing generator logic, consult `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` to avoid ABI/name drift.
- Release workflow uses `cargo-dist`; no need to touch `.github/workflows/release.yml` unless asked.

# Error Handling
- Use `eyre::Result` / `color-eyre` for CLI and tests; prefer `wrap_err` / `Context` for additional detail.
- For library code, prefer lightweight errors; avoid panics except for impossible states.
- In CLI flows, fail fast with contextual messages; surface actionable guidance when skipping steps (e.g., `--no-transpile`).

# Logging & Output
- Logging via `env_logger` + `log`; default filter `Info`. Respect existing levels.
- Keep stdout for user-facing info; use `println!` sparingly, prefer `log::{info, warn, error, debug}`.

# Imports & Modules
- Follow rustfmt ordering; group std, third-party, crate-local in natural order when rustfmt allows.
- Prefer module-level `use` blocks; keep `pub use` re-exports minimal and intentional.
- Macros: helper macros (e.g., `add_generator!`) live near use-sites; keep macro visibility private unless required.

# Types & Naming
- Use descriptive type aliases sparingly; rely on concrete types for clarity.
- Enum/struct names are PascalCase; functions snake_case; constants SCREAMING_SNAKE; modules snake_case.
- Feature flags: keep names in sync with `Cargo.toml` (`threads`, `multi_memory`, `unstable_print_debug`).
- Avoid abbreviated arg names unless already common in file (e.g., `vfs`, `wasm`).

# Formatting & Style
- Derive defaults where possible; manual impls only when needed.
- Keep comments minimal and purpose-driven (why > what), per `GEMINI.md` sparse-comment rule.
- Prefer iterator/functional style when readable; otherwise straightforward loops.
- Favor early returns for validation; avoid deep nesting in CLI orchestration.

# Feature & Config Handling
- Feature checks occur via `config_checker::FeatureChecker`; keep new feature toggles consistent.
- Restorers (`TomlRestorers`) must be updated when you mutate manifests during CLI runs; ensure cleanup paths are balanced.
- Threads/memory flags must stay consistent between CLI args and manifest feature checks; use `TargetMemoryType` helpers.

# Testing Patterns
- Integration helpers: `run_wasi_virt_layer` in `wasi_virt_layer-cli/tests/utils.rs` sets up CLI runs and cleans temp dirs (via `TestDir` drop).
- Use `OutDir::{Default, Path(&str), Random}` to isolate outputs per test and avoid collisions.
- When adding tests that need temp artifacts, honor `keep_build_artifacts` flag semantics.
- Add new dev-deps under `[dev-dependencies]` after user confirmation (see `Cargo.toml`).

# Performance & Safety
- Avoid unnecessary cloning of wasm/module buffers; pass references where possible.
- Watch for `threads` feature interactions: ensure ABI adjustments cover both single/multi memory paths.
- Preserve `non-recursive` ABI adjustments when touching `abi_connect` logic.

# Git & Process Rules (for agents)
- Before proposing commits, inspect `git status`, `git diff`, `git log -n 3` to match style.
- Always propose a draft commit message; never push without explicit request.
- Direct git commands for staging/committing/restoring are disallowed per `GEMINI.md`; rely on provided tooling/user approval.
- Confirm with user before changing shared configs (workspace `Cargo.toml`, CI, release files).

# Documentation & Examples
- README at root has CLI walkthrough; keep examples aligned when updating flags.
- Example runs (from README):
  - `cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm`
  - `cargo r -r -- -p threads_vfs test_threads -t single --threads true`

# When Refactoring Generators
- Always cross-check names/ABI expectations with `IMPORTS_EXPORTS_EVOLUTION_DETAILED.md`.
- Maintain `add_generators_by_type!` ordering and coverage; adding/removing generators requires reviewing downstream consumers.
- Preserve debug generators (`Debug*`) unless explicitly pruned; they are used for diagnostics.

# Adding New Files
- Prefer existing style; keep filenames snake_case for Rust modules.
- Avoid adding configs like rustfmt/clippy unless requested.

# PR/CI Awareness
- Release workflow triggered by tags; uses `cargo-dist`. Do not depend on GH secrets locally.
- No other workflows found; run local checks manually (fmt, clippy, test).

# Quick Start for New Agents
- Run `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -D warnings`, then `cargo test -r`.
- For targeted work: `cargo check -r -p <crate>` plus focused test filters.
- Keep error contexts rich with `wrap_err`; prefer `?` propagation.

# Maintenance Notes
- Keep this file ~150 lines; update when commands/rules change.
- If you change workflow or add features/flags, document the new usage here and in `GEMINI.md`.
- If you add Cursor/Copilot rules, reference them explicitly here.

# End
