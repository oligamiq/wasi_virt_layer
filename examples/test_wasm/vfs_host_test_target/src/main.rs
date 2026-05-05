#[unsafe(no_mangle)]
pub extern "C" fn test_user_func() {
    println!("test_user_func called (target side)");
}

#[link(wasm_import_module = "__wasip1_vfs-host")]
unsafe extern "C" {
    fn importing_test_vfs_func();
}

fn main() {
    println!("Hello from the target!");

    // We call the host function that should be redirected to the host
    unsafe { importing_test_vfs_func() };
}
