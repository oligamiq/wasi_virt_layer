// cargo +nightly b -r --target wasm32-wasip1-threads -p test_threads
// https://github.com/rust-lang/rust/issues/146721
// wasm-opt target/wasm32-wasip1-threads/release/test_threads.wasm -o examples/test_wasm/test_threads/test_threads.wasm -Oz
// cargo r -r -- -p threads_vfs examples/test_wasm/test_threads/test_threads.wasm -t single --no-tracing --threads true
// wasmtime run -Sthreads=y --env RUST_MIN_STACK=16777216 --env RUST_BACKTRACE=full target/wasm32-wasip1-threads/release/test_threads.wasm

fn main() {
    println!("Hello, world!");

    std::thread::spawn(|| {
        println!("Hello from a thread!");
    })
    .join()
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn __wasi_init_tp();
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn __________wasip1_vfs_thread_initializer() {
    unsafe { __wasi_init_tp() };
}

mod pthread {
    use core::ffi;
    pub type pthread_t = *mut ffi::c_void;
    pub type size_t = usize;
    #[repr(C)]
    union pthread_attr_union {
        __i: [ffi::c_int; if size_of::<ffi::c_long>() == 8 { 14 } else { 9 }],
        __vi: [ffi::c_int; if size_of::<ffi::c_long>() == 8 { 14 } else { 9 }],
        __s: [ffi::c_ulong; if size_of::<ffi::c_long>() == 8 { 7 } else { 9 }],
    }
    #[repr(C)]
    pub struct pthread_attr_t {
        __u: pthread_attr_union,
    }

    #[cfg(target_arch = "wasm32")]
    unsafe extern "C" {
        pub fn pthread_create(
            native: *mut pthread_t,
            attr: *const pthread_attr_t,
            f: extern "C" fn(*mut ffi::c_void) -> *mut ffi::c_void,
            value: *mut ffi::c_void,
        ) -> ffi::c_int;
        pub fn pthread_join(native: pthread_t, value: *mut *mut ffi::c_void) -> ffi::c_int;
        pub fn pthread_attr_init(attrp: *mut pthread_attr_t) -> ffi::c_int;
        pub fn pthread_attr_setstacksize(
            attr: *mut pthread_attr_t,
            stack_size: size_t,
        ) -> ffi::c_int;
        pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> ffi::c_int;
        pub fn pthread_detach(thread: pthread_t) -> ffi::c_int;
    }

    #[cfg(target_arch = "wasm32")]
    #[unsafe(no_mangle)]
    pub extern "C" fn __wasip1_vfs_debug_pthread_create() {
        unsafe {
            extern "C" fn fake(_: *mut ffi::c_void) -> *mut ffi::c_void {
                core::ptr::null_mut()
            }

            pthread_create(
                core::ptr::null_mut(),
                core::ptr::null(),
                fake,
                core::ptr::null_mut(),
            );
        }
    }

    #[cfg(target_arch = "wasm32")]
    #[unsafe(no_mangle)]
    pub extern "C" fn __wasip1_vfs_debug_pthread_attr_destroy() {
        unsafe {
            pthread_attr_destroy(core::ptr::null_mut());
        }
    }
}
