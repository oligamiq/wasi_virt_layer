# Gemini CLI Agent Workflow for wasi_virt_layer

This document outlines the general workflow and tools used by the Gemini CLI agent when working on the `wasi_virt_layer` project.
- Upon receiving instructions, first check whether there is anything that should be included in GEMINI.md; if so, update it.

## Constraints
- All updates to configuration files such as Cargo.toml must be confirmed.
- Changes such as `git add commit restore` will be rejected. Changes such as `git diff status` will be permitted.

## Core Mandates

-   **Adherence to Conventions:** Always prioritize existing project conventions (code style, naming, architecture).
-   **No Assumptions on Libraries:** Verify library/framework usage within the project before introducing new ones.
-   **Idiomatic Changes:** Ensure changes integrate naturally with the local code context.
-   **Sparse Comments:** Add comments only for *why* something is done, not *what* is done.
-   **Proactive Test Creation:** New features/bug fixes include relevant tests.
-   **Confirm Ambiguity:** Clarify with the user before taking significant actions beyond the clear scope of the request.
-   **No Summaries (Unless Asked):** Avoid summarizing changes after completion unless explicitly requested.

## Primary Workflow: Software Engineering Tasks

1.  **Understand & Strategize:** Analyze the request and codebase. For complex tasks, `codebase_investigator` is used. For simple searches, `search_file_content` or `glob`.
2.  **Plan:** Develop a coherent plan based on understanding. For complex tasks, `write_todos` is used to break down and track subtasks. Share concise plan with the user if beneficial.
3.  **Implement:** Use tools like `replace`, `write_file`, `run_shell_command` while adhering to project conventions.
4.  **Verify (Tests):** Execute project's testing procedures.
5.  **Verify (Standards):** Run project-specific build, linting, and type-checking commands (e.g., `cargo check`, `cargo fmt`).
6.  **Finalize:** Update `GEMINI.md` to reflect the changes made, then await next instructions.

## Test Helper Abstraction

To simplify integration tests, a helper function `run_wasi_virt_layer` is provided in `wasi_virt_layer-cli/tests/utils.rs`. This function abstracts the process of building and running the `wasi_virt_layer` command with various arguments.

### `run_wasi_virt_layer`

This is the primary helper function for integration tests.

**Signature:**
```rust
pub fn run_wasi_virt_layer(
    p_vfs: Option<&str>,
    wasm: Option<&str>,
    t_single: Option<bool>,
    threads: bool,
    out_dir: OutDir,
    keep_build_artifacts: bool,
    other_args: &[&str],
) -> color_eyre::Result<TestDir>
```

**Parameters:**
- `keep_build_artifacts`: A `bool` indicating whether intermediate build artifacts should be kept. If `true`, files like `*.adjusted.wasm` and `*.opt.wasm` will remain in the output directory. If `false`, these intermediate files are deleted.

### `OutDir` Enum

The `out_dir` parameter uses the `OutDir` enum to specify the output directory strategy:

-   **`OutDir::Default`**: Uses the default output directory (`tests/dist`).
-   **`OutDir::Path(&str)`**: Specifies a custom output directory path.
-   **`OutDir::Random`**: Creates a new unique directory under `tests/` for the output. This is useful for tests that need to run in isolation to avoid conflicts.

**Example Usage:**
```rust
// Run with default output directory
run_wasi_virt_layer(Some("my_vfs"), Some("my_wasm"), Some(true), false, OutDir::Default, &[])?;

// Run with a specific output directory
run_wasi_virt_layer(Some("my_vfs"), Some("my_wasm"), Some(true), false, OutDir::Path("tests/custom_dist"), &[])?;

// Run with a random output directory for isolated testing
run_wasi_virt_layer(Some("threads_vfs"), Some("test_threads"), None, true, OutDir::Random, &[])?;
```

## Git Operations

-   `git status`, `git diff HEAD`, `git log -n 3` are used to understand the current state before committing.
-   Draft commit messages are always proposed.
-   Changes are never pushed without explicit user request.

## Operational Guidelines

-   When performing a build, use `cargo r -r -- ...`
-   When performing a check, use `cargo check -r`
-   When running tests, use `cargo test -r`

This document serves as a guide for the agent's operation within this repository.
