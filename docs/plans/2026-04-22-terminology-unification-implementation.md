# Terminology Unification Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rename `const`/`static` to `Embedded`, `changeable`/`dynamic` to `Dynamic`, and unify trait implementations under `Standard`.

**Architecture:** Systematic search and replace across the workspace, starting from core library definitions, moving to CLI generators, then examples, and finally documentation.

**Tech Stack:** Rust, Cargo, Bash (sed/rg)

---

### Task 1: Rename Directories and Feature Flags
**Files:**
- Modify: `wasi_virt_layer/Cargo.toml`
- Modify: `wasi_virt_layer-cli/Cargo.toml`
- Modify: `examples/*/Cargo.toml`

**Step 1: Rename directories on disk**
Run:
```bash
mv wasi_virt_layer/src/wasi/file/constant wasi_virt_layer/src/wasi/file/embedded
mv wasi_virt_layer/src/wasi/file/changeable wasi_virt_layer/src/wasi/file/dynamic
```

**Step 2: Update feature names in `Cargo.toml`**
Change `const-fs` -> `embedded-fs`, `changeable-fs` -> `dynamic-fs`.

**Step 3: Commit**
```bash
git add .
git commit -m "refactor: rename fs directories and update feature flags"
```

### Task 2: Core Library Type Refactoring (`wasi_virt_layer`)
**Files:**
- Modify: `wasi_virt_layer/src/lib.rs`
- Modify: `wasi_virt_layer/src/wasi/file/mod.rs`
- Modify: `wasi_virt_layer/src/wasi/file/embedded/*.rs`
- Modify: `wasi_virt_layer/src/wasi/file/dynamic/*.rs`
- Modify: `wasi_virt_layer/src/utils.rs`

**Step 1: Rename structs and traits**
- `StandardEmbeddedFileSystem` -> `StandardEmbeddedFileSystem`
- `StandardDynamicFileSystem` -> `StandardDynamicFileSystem`
- `StandardMultipleFileSystem` -> `StandardMultipleFileSystem`
- `StandardEmbeddedNormalLFS` -> `StandardEmbeddedNormalLFS`
- `StandardDynamicLFS` -> `StandardDynamicLFS`
- `EmbeddedArrayBuilder` -> `EmbeddedArrayBuilder`

**Step 2: Update macro tags**
- `@embedded` -> `@embedded` in `plug_env!` and others.

**Step 3: Commit**
```bash
git add wasi_virt_layer/src
git commit -m "refactor: update core library types and macros to new terminology"
```

### Task 3: CLI Generator Refactoring (`wasi_virt_layer-cli`)
**Files:**
- Modify: `wasi_virt_layer-cli/src/generator/*.rs`
- Modify: `wasi_virt_layer-cli/src/args.rs`

**Step 1: Update code generation logic**
Ensure generated code uses `StandardEmbeddedFileSystem` etc. instead of old names.

**Step 2: Update CLI arguments**
Update help messages and potentially argument names if they use "const" or "static".

**Step 3: Commit**
```bash
git add wasi_virt_layer-cli/src
git commit -m "refactor: update cli generators and arguments"
```

### Task 4: Example and Test Update
**Files:**
- Modify: `examples/**/*.rs`
- Modify: `wasi_virt_layer-cli/tests/**/*.rs`

**Step 1: Update all usages in examples and tests**
Apply mass search and replace for renamed types and macros.

**Step 2: Run verification**
Run: `cargo check -r` and `cargo nextest run -r --fail-fast`.

**Step 3: Commit**
```bash
git add examples wasi_virt_layer-cli/tests
git commit -m "refactor: update examples and tests to use new terminology"
```


