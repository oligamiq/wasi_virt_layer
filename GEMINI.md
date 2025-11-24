# Gemini CLI Agent Workflow for wasi_virt_layer

This document outlines the general workflow, conventions, and tools used by the Gemini CLI agent when working on the `wasi_virt_layer` project.

-   **Upon receiving instructions, first check whether there is anything that should be included in this document; if so, update it.**

## 1. Project Overview

`wasi_virt_layer` is a crate that provides a virtualization layer for WASI. It allows for combining a virtual file system (VFS) WASM module with one or more main WASM modules, creating a single component that uses the VFS for file operations.

The project consists of two main crates:
-   `wasi_virt_layer`: The core library implementing the virtualization logic.
-   `wasi_virt_layer-cli`: A command-line interface for applying the virtualization layer to WASM files.

## 2. Core Mandates & Constraints

### Core Mandates
-   **Adherence to Conventions:** Always prioritize existing project conventions (code style, naming, architecture).
-   **No Assumptions on Libraries:** Verify library/framework usage within the project before introducing new ones.
-   **Idiomatic Changes:** Ensure changes integrate naturally with the local code context.
-   **Sparse Comments:** Add comments only for *why* something is done, not *what* is done.
-   **Proactive Test Creation:** New features/bug fixes must include relevant tests.
-   **Confirm Ambiguity:** Clarify with the user before taking significant actions beyond the clear scope of the request.
-   **No Summaries (Unless Asked):** Avoid summarizing changes after completion unless explicitly requested.

### Constraints
-   All updates to configuration files such as `Cargo.toml` must be confirmed with the user. This includes adding or removing dependencies.
-   Direct git commands for staging (`git add`), committing (`git commit`), or restoring (`git restore`) files are disallowed. Agent must rely on its internal tools and user confirmation for each version control. Read-only commands like `git status` and `git diff` are permitted.

## 3. Development Workflow

### 3.1. Operational Guidelines

-   **Building & Running:** Use `cargo r -r -- ...` for release builds of the CLI.
-   **Checking:** Use `cargo check -r` to check the project in release mode.
-   **Testing:** Use `cargo test -r` to run the test suite. To run a specific test, use `cargo test -r --test <test_name> <specific_test_function>`.

### 3.2. Primary Workflow for Tasks

1.  **Understand & Strategize:** Analyze the request and codebase. For complex tasks, `codebase_investigator` is used. For simple searches, `search_file_content` or `glob`.
2.  **Plan:** Develop a coherent plan. For complex tasks, use `write_todos` to track subtasks.
3.  **Implement:** Use tools like `replace`, `write_file`, `run_shell_command` while adhering to project conventions.
4.  **Verify (Tests):** Execute project's testing procedures as described above.
5.  **Verify (Standards):** Run project-specific build, linting, and type-checking commands (`cargo check`, `cargo fmt`).
6.  **Finalize:** Update this document (`GEMINI.md`) if the changes affect the workflow or project structure, then await the next instruction.

### 3.3. Refactoring Guidelines

-   **Consult the Evolution Document:** When refactoring, especially code within the `generator` modules, you **must** consult the `wasi_virt_layer-cli/IMPORTS_EXPORTS_EVOLUTION_DETAILED.md` file. This document exhaustively lists the expected import and export names at each stage of the build process. Cross-reference your changes against this document to ensure that function and module names are not accidentally altered, as this has been a common source of bugs.

### 3.4. Git Operations

-   Before proposing a commit, use `git status`, `git diff HEAD`, and `git log -n 3` to understand the current state and match commit style.
-   Always propose a draft commit message.
-   Never push changes without an explicit user request.

## 4. Integration Testing

Integration tests are crucial to this project and are located in `wasi_virt_layer-cli/tests/`. They rely on a set of helper utilities in `wasi_virt_layer-cli/tests/utils.rs`.

### 4.1. Test Dependencies

When adding or modifying tests, you may need to add development dependencies. These should be added under the `[dev-dependencies]` section in `wasi_virt_layer-cli/Cargo.toml` after user confirmation. For example, the `glob` crate is used for file path matching in tests.

### 4.2. Test Helper Abstraction

A key helper function `run_wasi_virt_layer` is provided in `wasi_virt_layer-cli/tests/utils.rs`. This function abstracts the process of building and running the `wasi_virt_layer` command with various arguments, simplifying test creation.

#### `run_wasi_virt_layer`

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
-   `keep_build_artifacts`: A `bool` indicating whether the CLI tool should keep intermediate build artifacts (e.g., `*.adjusted.wasm`, `*.opt.wasm`). If `false`, these files are deleted by the CLI during its execution.

#### `OutDir` Enum

The `out_dir` parameter uses the `OutDir` enum to specify the output directory strategy:

-   **`OutDir::Default`**: Uses the default output directory (`tests/dist`).
-   **`OutDir::Path(&str)`**: Specifies a custom output directory path.
-   **`OutDir::Random`**: Creates a new unique directory under `tests/` for the output. This is useful for tests that need to run in isolation to avoid conflicts.

#### `TestDir` Struct

The `run_wasi_virt_layer` function returns a `TestDir` instance. This is a wrapper around the test's output directory path. It implements the `Drop` trait to automatically delete the entire temporary test directory and its contents when the test function goes out of scope, ensuring a clean state between test runs.
