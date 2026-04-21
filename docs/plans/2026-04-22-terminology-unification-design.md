# Terminology Unification Design

## 1. Overview
This design aims to unify terminology within the `wasi_virt_layer` project to avoid confusion with Rust keywords and to provide a clearer structure for traits and their implementations.

## 2. Terminology Mapping

### 2.1 Core Concepts
| Old Term | New Term | Description |
| :--- | :--- | :--- |
| `const`, `static` | **`Embedded`** | Data pre-defined and fixed in the binary, not dynamically allocated. |
| `changeable`, `dynamic` | **`Dynamic`** | Data or structures that can be modified or grow at runtime. |
| (Trait implementations) | **`Standard`** | Standardized implementation of a trait provided by this library. |

### 2.2 Structs and Traits
| Old Name | New Name | Nature |
| :--- | :--- | :--- |
| `Wasip1ConstVFS` | `StandardEmbeddedFileSystem` | Implementation of `Wasip1FileSystem` |
| `ChangeableVFS` | `StandardDynamicFileSystem` | Implementation of `Wasip1FileSystem` |
| `Wasip1MultipleVFS` | `StandardMultipleFileSystem` | Implementation of `Wasip1FileSystem` |
| `VFSConstNormalLFS` | `StandardEmbeddedNormalLFS` | Implementation of LFS trait |
| `ChangeableLFS` | `StandardDynamicLFS` | Implementation of LFS trait |
| `StaticArrayBuilder` | `EmbeddedArrayBuilder` | Fixed-capacity utility |
| `DefaultStdIO` | `StandardStdIO` | Standard I/O implementation |
| `VirtualEnvConstState` | `EmbeddedVirtualEnvState` | Environment state |
| `VirtualEnvState` | `DynamicVirtualEnvState` | Environment state |

### 2.3 File System / Feature Flags
| Old Category | New Category | Feature Flag |
| :--- | :--- | :--- |
| `constant` | `embedded` | `embedded-fs` |
| `changeable` | `dynamic` | `dynamic-fs` |
| `multiple` | `multiple` | `multiple-fs` |

## 3. Implementation Plan

### 3.1 Directory & File Renaming
- `wasi/file/constant/` -> `wasi/file/embedded/`
- `wasi/file/changeable/` -> `wasi/file/dynamic/`
- Associated files within these directories will be renamed to match their new struct names where appropriate.

### 3.2 Macro Updates
- `plug_env!(@const, ...)` -> `plug_env!(@embedded, ...)`
- `plug_env!(@dynamic, ...)` -> `plug_env!(@dynamic, ...)` (mode name stays same, internal logic might change)

### 3.3 CI & Config
- Update all `Cargo.toml` files to reflect new feature flags.
- Update test scripts and CLI argument parsing in `wasi_virt_layer-cli`.

## 4. Migration Strategy
1.  Apply changes to `wasi_virt_layer` core library.
2.  Update `wasi_virt_layer-cli` generators.
3.  Update workspace-wide `Cargo.toml` and feature usage.
4.  Refactor examples and integration tests.
5.  Verify with full test suite.
