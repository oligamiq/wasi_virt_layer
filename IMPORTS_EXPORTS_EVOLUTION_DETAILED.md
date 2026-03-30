# Wasm Import/Export Evolution (Detailed)

This document exhaustively tracks the changes in Wasm import and export names through each generator stage for different feature combinations.

## Feature Combination: `no_features`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

## Feature Combination: `alloc`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

## Feature Combination: `std`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

## Feature Combination: `multi_memory`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

## Feature Combination: `unstable_print_debug`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

## Feature Combination: `multi_memory_std`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

## Feature Combination: `multi_memory_unstable_print_debug`

### Stage 0: Initial Modules

### Stage: `no_std_vfs.wasm (initial)`

Failed to process `no_std_vfs.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/no_std_vfs.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Stage: `test_wasm.wasm (initial)`

Failed to process `test_wasm.wasm (initial)`. Error:
```
error: failed to read from `target/wasm32-wasip1/release/test_wasm.wasm`

Caused by:
    0: 指定されたパスが見つかりません。 (os error 3)

```

### Post-Merge Stages

### Stage: `merged.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_to"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm_memory_copy_from"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm__start"` |
| `"wasip1-vfs"` | `"__wasip1_vfs_test_wasm___main_void"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"__wasip1_vfs_flag_vfs_memory"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_vfs_global"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm_memory"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"__wasip1_vfs_test_wasm___main_void"` |
| `"__wasip1_vfs_flag_test_wasm_memory"` |
| `"__wasip1_vfs_flag_test_wasm_global"` |

### Stage: `merged.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `merged.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.component.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `(instance` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `(;0;)` |
| `(;0;)` |
| `(;1;)` |
| `(;2;)` |
| `(;3;)` |
| `(;4;)` |
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |
| `"[static]wasip1.fd-write-import"` |
| `"[static]wasip1.fd-read-import"` |
| `"[static]wasip1.environ-get-import"` |
| `"[static]wasip1.environ-sizes-get-import"` |
| `"[static]wasip1.proc-exit-import"` |
| `(;6;)` |

### Stage: `no_std_vfs.core.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"__wasip1_vfs_flag_info_vfs_name_no_std_vfs"` |
| `"__wasip1_vfs_flag_info_target_names_0000000019test_wasm"` |
| `"__wasip1_vfs_flag_info_target_memory_type_Multi"` |
| `"__wasip1_vfs_flag_info_unstable_print_debug_true"` |
| `"__wasip1_vfs_flag_info_dwarf_false"` |
| `"__wasip1_vfs_flag_info_threads_false"` |
| `"__wasip1_vfs_flag_info_adjust_abi_false"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.opt.adjusted.opt.adjusted.adjusted.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

### Stage: `no_std_vfs.core.wasm`

#### Imports

| Module | Name |
|---|---|
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-write-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.fd-read-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.environ-sizes-get-import"` |
| `"wasip1-vfs:host/virtual-file-system-wasip1-core"` | `"[static]wasip1.proc-exit-import"` |

#### Exports

| Name |
|---|
| `"memory"` |
| `"__wasip1_vfs_test_wasm__start_anchor"` |
| `"__wasip1_vfs_test_wasm_proc_exit"` |
| `"__wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"__wasip1_vfs_test_wasm_environ_get"` |
| `"__wasip1_vfs_test_wasm_fd_write"` |
| `"__wasip1_vfs_test_wasm_fd_readdir"` |
| `"__wasip1_vfs_test_wasm_path_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"__wasip1_vfs_test_wasm_fd_close"` |
| `"__wasip1_vfs_test_wasm_path_open"` |
| `"__wasip1_vfs_test_wasm_fd_read"` |
| `"__wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"__wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"main"` |
| `"cabi_realloc_wit_bindgen_0_43_0"` |
| `"cabi_realloc"` |
| `"debug_blind_print_etc_flag"` |
| `"debug_wasip1_vfs_pre_init"` |
| `"cabi_realloc_wit_bindgen_0_44_0"` |
| `"debug___wasip1_vfs_test_wasm_proc_exit"` |
| `"debug___wasip1_vfs_test_wasm_environ_sizes_get"` |
| `"debug___wasip1_vfs_test_wasm_environ_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_write"` |
| `"debug___wasip1_vfs_test_wasm_fd_readdir"` |
| `"debug___wasip1_vfs_test_wasm_path_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_prestat_dir_name"` |
| `"debug___wasip1_vfs_test_wasm_fd_close"` |
| `"debug___wasip1_vfs_test_wasm_path_open"` |
| `"debug___wasip1_vfs_test_wasm_fd_read"` |
| `"debug___wasip1_vfs_test_wasm_fd_filestat_get"` |
| `"debug___wasip1_vfs_test_wasm_fd_fdstat_get"` |
| `"__wasip1_vfs_test_wasm__start"` |
| `"_____debug_left___wasip1_vfs_test_wasm___main_void"` |

