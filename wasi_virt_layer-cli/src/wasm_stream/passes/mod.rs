#![allow(unused_variables, unused_assignments, unreachable_patterns, dead_code)]
pub mod abi_connect;
pub mod anonymous;
pub mod check;
pub mod check_unused_threads;
pub mod dummy_injector;
pub mod multi_memory_lowering;
pub mod patch_component;
pub mod poll;
pub mod post_combine;
pub mod pre_vfs_memory_refuge;
pub mod producer;
pub mod shared_global;
pub mod special_func;
pub mod starts_pre;
pub mod threads_spawn;
pub mod wrap_unreachable;

pub mod atomic_patch;
pub mod component_ctx;
pub mod extract_mem_sizes;
pub mod fn_in_starts;
pub mod memory_post_components;
