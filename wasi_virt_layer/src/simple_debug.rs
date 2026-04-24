use crate::{__private::wasip1, transporter::non_recursive_fd_write};

/// Prints a simple debug message to stderr.
/// This function is safe to call even in early initialization stages,
/// although it will only output once `pre_init` has been called.
pub fn simple_debug_print(buf: impl AsRef<[u8]>) {
    // We cannot call the import function until pre_init has been executed.
    // We do not use a queue because it requires memory allocation and is likely to corrupt the initialization.
    if is_pre_init() {
        return;
    }

    unsafe {
        let ciovec_arr = [wasip1::Ciovec {
            buf: buf.as_ref().as_ptr() as *const u8,
            buf_len: buf.as_ref().len(),
        }, wasip1::Ciovec {
            buf: b"\n".as_ptr(),
            buf_len: 1,
        }];

        let _ = non_recursive_fd_write(2, &ciovec_arr);
    }
}

#[cfg(feature = "threads")]
mod threads {
    use core::cell::UnsafeCell;

    thread_local! {
        static IS_PRE_INIT: UnsafeCell<bool> = UnsafeCell::new(true);
    }

    #[unsafe(no_mangle)]
    extern "C" fn simple_debug_wasip1_vfs_pre_init() {
        IS_PRE_INIT.with(|c| unsafe { *c.get() = false });
    }

    pub(crate) fn is_pre_init() -> bool {
        IS_PRE_INIT.with(|c| unsafe { *c.get() })
    }
}
#[cfg(feature = "threads")]
pub(crate) use threads::is_pre_init;

#[cfg(not(feature = "threads"))]
mod non_threads {
    use core::cell::UnsafeCell;

    struct NonThreadSafeCell<T> {
        value: UnsafeCell<T>,
    }

    unsafe impl<T> Sync for NonThreadSafeCell<T> {}

    static IS_PRE_INIT: NonThreadSafeCell<bool> = NonThreadSafeCell {
        value: UnsafeCell::new(true),
    };

    #[unsafe(no_mangle)]
    extern "C" fn simple_debug_wasip1_vfs_pre_init() {
        unsafe { *(IS_PRE_INIT.value.get()) = false };
    }

    pub(crate) fn is_pre_init() -> bool {
        unsafe { *IS_PRE_INIT.value.get() }
    }
}
#[cfg(not(feature = "threads"))]
pub(crate) use non_threads::is_pre_init;
