# AGENTS.md

## Scope
- Workspace default members: `wasi_virt_layer` (lib) and `wasi_virt_layer-cli` (CLI). See `Cargo.toml` for full workspace members.
- Rust edition is 2024. `wasi_virt_layer` has MSRV 1.89.0; `wasi_virt_layer-cli` has MSRV 1.93.0 because of its current dependency graph.

## Commands
- Check workspace (release profile): `cargo check -r`
- Run CLI (preferred): `cargo r -r -- <args>`
- Nextest (recommended): `cargo nextest run -r --fail-fast` (install via `cargo binstall cargo-nextest -y` per `README.md`)
- Lint: `cargo clippy --all-targets --all-features -D warnings`
- Format: `cargo fmt`

## Testing and examples
- CLI integration tests live in `wasi_virt_layer-cli/tests/`; use `utils.rs::run_wasi_virt_layer` helper.
- Use `OutDir::Random` in tests to avoid parallel collisions; set `keep_build_artifacts: true` to keep intermediates for debugging.
- Example runs from `README.md`:
  - `cargo r -r -- -p example_vfs examples/test_wasm/example/test_wasm_opt.wasm`
  - `cargo r -r -- -p threads_vfs test_threads -t single --threads true`
  - `cargo r -- -p threads_vfs test_threads -t multi --threads true`

## Generator / ABI changes
- If touching generator modules or ABI naming, consult `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` and keep import/export names in sync across stages; keep `Debug*` generators unless intentionally removed.

## Known quirks (from `README.md`)
- Threads + memory mode: `single_memory` can fail where `multi_memory` succeeds; reset sequence `test_threads::_reset(); _start(); _main();` exposes this.
- Build cache: target dir caching may require `--no-cache`.
- Very long CLI args and self-calling binaries are known to be tricky.

## Conventions
- Doc comments: add them as much as possible.
- `wasm-bindgen` is not supported (README “cannot”).
