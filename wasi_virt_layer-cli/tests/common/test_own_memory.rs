use crate::utils::{self, OutDir};

#[test]
fn test_own_memory_expansion() {
    // When the memory is not manually expanded, the test should fail to allocate memory.
    let err = utils::run_wasi_virt_layer(
        Some("own_memory_vfs"),
        None,
        None,  // t_single
        false, // threads
        OutDir::Random,
        true, // keep_build_artifacts
        &["--own-memory", "big_alloc"],
        None,
    )
    .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("RuntimeError: unreachable")
            || err_str.contains("memory allocation of 104857600 bytes failed"),
        "Expected memory allocation failure due to strict own_memory bounds, but got: {}",
        err_str
    );

    // When the memory is manually expanded (via feature), the test should succeed.
    let _dir = utils::run_wasi_virt_layer(
        Some("own_memory_vfs"),
        None,
        None,  // t_single
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

#[test]
fn test_own_memory_self_api() {
    let dir = utils::run_wasi_virt_layer(
        Some("own_memory_vfs"),
        None,
        None,  // t_single
        false, // threads
        OutDir::Random,
        true, // keep_build_artifacts
        &[
            "--own-memory",
            "--features",
            "manual_expand_memory,self_own_memory_api",
            "big_alloc",
        ],
        None,
    )
    .expect("Test should succeed with self own-memory API enabled");

    let stdout = std::fs::read_to_string(dir.0.join(".deno-test-stdout.log"))
        .expect("deno stdout should be captured");
    assert!(
        stdout.contains("self own-memory API results:"),
        "expected self own-memory API output, got: {stdout}"
    );
}

#[test]
fn test_own_memory_rejects_self_argument() {
    let err = utils::run_wasi_virt_layer(
        Some("own_memory_vfs"),
        None,
        None,  // t_single
        false, // threads
        OutDir::Random,
        true, // keep_build_artifacts
        &[
            "--own-memory",
            "--features",
            "invalid_self_own_memory_arg",
            "big_alloc",
        ],
        None,
    )
    .expect_err("own_memory!(self, ...) should fail at compile time");

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("own_memory! does not accept `self`"),
        "expected explicit self rejection, got: {err_str}"
    );
}
