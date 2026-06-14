use crate::utils::{self, OutDir};

#[test]
fn test_own_memory_expansion() {
    // When the memory is not manually expanded, the test should fail to allocate memory.
    let err = utils::run_wasi_virt_layer(
        Some("own_memory_vfs"),
        None,
        None, // t_single
        false, // threads
        OutDir::Random,
        true, // keep_build_artifacts
        &["--own-memory", "big_alloc"],
        None,
    )
    .unwrap_err();
    
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("RuntimeError: unreachable") || err_str.contains("memory allocation of 104857600 bytes failed"),
        "Expected memory allocation failure due to strict own_memory bounds, but got: {}",
        err_str
    );

    // When the memory is manually expanded (via feature), the test should succeed.
    // TODO But test failed
    let _dir = utils::run_wasi_virt_layer(
        Some("own_memory_vfs"),
        None,
        None, // t_single
        false, // threads
        OutDir::Random,
        true, // keep_build_artifacts
        &[
            "--own-memory",
            "--features",
            "manual_expand_memory",
            "big_alloc",
        ],
        None,
    )
    .expect("Test should succeed with manual memory expansion");
}
