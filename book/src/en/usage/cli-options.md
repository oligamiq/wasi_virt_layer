# CLI Options

The `wasi_virt_layer` CLI provides flags to configure stack management behavior during the module building and stitching process.

## Stack Configuration

You can configure the stack size and slots for both the VFS module and target modules.

*   **`--stack-size MODULE=SIZE`**
    Configures the stack size for a specific module.
    *   `MODULE`: The name of the module (e.g., `vfs`, `target_name`).
    *   `SIZE`: The size of the stack. It supports suffixes for easier readability, such as `MiB`, `KiB`, `M`, and `K` (e.g., `--stack-size vfs=2MiB`).

*   **`--stack-slots MODULE=COUNT`**
    Configures the number of stack slots. This is primarily used for multi-memory target execution where multiple slots are managed within a stack arena.
    *   `MODULE`: The name of the target module.
    *   `COUNT`: The number of slots to allocate.

### Constraints and Validation

The CLI enforces certain validations on stack configurations:
*   `--stack-slots` is rejected if applied to the VFS module (the VFS manages its own distinct stack).
*   `--stack-slots` is rejected when operating in single-memory modes.
*   In single-memory modes, if target stack isolation is required, configuring the VFS stack size (`--stack-size vfs=...`) is mandatory.
