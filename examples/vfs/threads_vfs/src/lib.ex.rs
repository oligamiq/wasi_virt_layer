mod a {
    use super::*;
    #[allow(non_camel_case_types)]
    pub(crate) enum ThreadAccessor {
        __self,
        test_threads,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for ThreadAccessor {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ThreadAccessor::__self => "__self",
                    ThreadAccessor::test_threads => "test_threads",
                },
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for ThreadAccessor {
        #[inline]
        fn clone(&self) -> ThreadAccessor {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for ThreadAccessor {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for ThreadAccessor {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for ThreadAccessor {
        #[inline]
        fn eq(&self, other: &ThreadAccessor) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for ThreadAccessor {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::hash::Hash for ThreadAccessor {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl ::wasip1_virtual_layer::thread::ThreadAccess for ThreadAccessor {
        fn call_wasi_thread_start(
            &self,
            ptr: ::wasip1_virtual_layer::thread::ThreadRunner,
            thread_id: Option<core::num::NonZero<u32>>,
        ) {
            {
                match *self {
                    Self::__self => {
                        unsafe {
                            __wasip1_vfs___self_wasi_thread_start(
                                match thread_id {
                                    Some(id) => u32::from(id) as i32,
                                    None => -1,
                                },
                                ptr.inner() as i32,
                            )
                        }
                    }
                    Self::test_threads => {
                        unsafe {
                            __wasip1_vfs_test_threads_wasi_thread_start(
                                match thread_id {
                                    Some(id) => u32::from(id) as i32,
                                    None => -1,
                                },
                                ptr.inner() as i32,
                            )
                        }
                    }
                }
            }
        }
        fn as_name(&self) -> &'static str {
            match *self {
                Self::__self => {
                    type T = ::wasip1_virtual_layer::__private::__self;
                    <T as ::wasip1_virtual_layer::memory::WasmAccess>::NAME
                }
                Self::test_threads => {
                    type T = test_threads;
                    <T as ::wasip1_virtual_layer::memory::WasmAccess>::NAME
                }
            }
        }
    }
    #[doc(hidden)]
    #[link(wasm_import_module = "wasip1-vfs")]
    unsafe extern "C" {
        pub fn __wasip1_vfs___self_wasi_thread_start(thread_id: i32, ptr: i32);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wasip1_vfs___self_wasi_thread_start_anchor(
        thread_id: i32,
        ptr: i32,
    ) {
        unsafe {
            __wasip1_vfs___self_wasi_thread_start(thread_id, ptr);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wasip1_vfs_wasi_thread_spawn___self(
        data_ptr: ::wasip1_virtual_layer::__private::inner::thread::ThreadRunner,
    ) -> i32 {
        use ::wasip1_virtual_layer::thread::{VirtualThread, ThreadAccess};
        const ACCESSOR: ThreadAccessor = ThreadAccessor::__self;
        #[allow(unused_mut)]
        let mut pool = DirectThreadPool;
        match pool.new_thread(ACCESSOR, data_ptr) {
            Some(thread_id) => {
                return u32::from(thread_id) as i32;
            }
            None => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Failed to create a new thread"),
                    );
                };
            }
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs___self_sched_yield() -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        use ::wasip1_virtual_layer::thread::VirtualThread;
        #[allow(unused_mut)]
        let mut pool = DirectThreadPool;
        type T = ::wasip1_virtual_layer::__private::__self;
        pool.sched_yield::<T>()
    }
    #[doc(hidden)]
    #[link(wasm_import_module = "wasip1-vfs")]
    unsafe extern "C" {
        pub fn __wasip1_vfs_test_threads_wasi_thread_start(thread_id: i32, ptr: i32);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wasip1_vfs_test_threads_wasi_thread_start_anchor(
        thread_id: i32,
        ptr: i32,
    ) {
        unsafe {
            __wasip1_vfs_test_threads_wasi_thread_start(thread_id, ptr);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wasip1_vfs_wasi_thread_spawn_test_threads(
        data_ptr: ::wasip1_virtual_layer::__private::inner::thread::ThreadRunner,
    ) -> i32 {
        use ::wasip1_virtual_layer::thread::{VirtualThread, ThreadAccess};
        const ACCESSOR: ThreadAccessor = ThreadAccessor::test_threads;
        #[allow(unused_mut)]
        let mut pool = DirectThreadPool;
        match pool.new_thread(ACCESSOR, data_ptr) {
            Some(thread_id) => {
                return u32::from(thread_id) as i32;
            }
            None => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Failed to create a new thread"),
                    );
                };
            }
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wasip1_vfs_test_threads_sched_yield() -> ::wasip1_virtual_layer::__private::wasip1::Errno {
        use ::wasip1_virtual_layer::thread::VirtualThread;
        #[allow(unused_mut)]
        let mut pool = DirectThreadPool;
        type T = test_threads;
        pool.sched_yield::<T>()
    }
}
