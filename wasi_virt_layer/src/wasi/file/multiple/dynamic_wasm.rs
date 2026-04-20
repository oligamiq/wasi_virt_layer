#![cfg(feature = "multiple-fs")]

use crate::__private::wasip1;
use crate::memory::WasmAccessDynCompatibleRaw;

/// A structure holding function pointers for a pseudo-Wasm module.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PseudoWasmSimple {
    pub _main_ptr: fn() -> wasip1::Errno,
    pub _reset_ptr: fn(),
    pub _start_ptr: fn(),
    pub memcpy_raw_ptr: fn(*mut u8, *const u8, usize),
    pub memcpy_to_raw_ptr: fn(*mut u8, *const u8, usize),
    #[cfg(not(feature = "multi_memory"))]
    pub memory_director_raw_ptr: Option<fn(isize) -> isize>, // Optional for multi-memory support
}

/// A specific trait for static pseudo-Wasm structures.
/// It provides a function that receives the underlying pointer group and requires dynamic Wasm compatibility.
pub trait PseudoWasmTrait {
    type Generated;

    fn restore(&self, generated: Self::Generated) -> impl WasmAccessDynCompatibleRaw;

    /// Receives the struct holding the pointer group to create an instance.
    fn receive_pseudo_wasm(&self, ptrs: PseudoWasmSimple) -> Self::Generated;
}

/// A standard struct that holds the pointer group and implements `PseudoWasmTrait`
/// as well as `WasmAccessDynCompatibleRaw`.
/// Note: This struct does NOT (and cannot) implement `WasmAccessRaw` directly.
#[derive(Debug)]
pub struct StandardPseudoWasmHolder {
    #[cfg(feature = "threads")]
    pub once: core::sync::atomic::AtomicBool,
    #[cfg(not(feature = "threads"))]
    pub once: bool,
    pub simple: core::cell::UnsafeCell<PseudoWasmSimple>,
}

impl AsRef<StandardPseudoWasmHolder> for StandardPseudoWasmHolder {
    fn as_ref(&self) -> &StandardPseudoWasmHolder {
        self
    }
}

impl StandardPseudoWasmHolder {
    pub fn get(&self) -> &PseudoWasmSimple {
        unsafe { self.simple.get().as_ref().unwrap() }
    }
}

unsafe impl Send for StandardPseudoWasmHolder {}
unsafe impl Sync for StandardPseudoWasmHolder {}

impl PseudoWasmTrait for StandardPseudoWasmHolder {
    type Generated = ();

    fn restore(&self, _: Self::Generated) -> impl WasmAccessDynCompatibleRaw {
        self
    }

    fn receive_pseudo_wasm(&self, ptrs: PseudoWasmSimple) -> Self::Generated {
        #[cfg(feature = "threads")]
        {
            use core::sync::atomic::Ordering;
            if self
                .once
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_err()
            {
                return ();
            }
            unsafe { *self.simple.get() = ptrs };
        }
        #[cfg(not(feature = "threads"))]
        {
            if self.once {
                return ();
            }
            unsafe { *self.simple.get() = ptrs };
            self.once = true;
        }
    }
}

impl WasmAccessDynCompatibleRaw for StandardPseudoWasmHolder {
    #[inline(always)]
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        (self.get().memcpy_raw_ptr)(offset, src, len)
    }

    #[inline(always)]
    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        (self.get().memcpy_to_raw_ptr)(offset, src, len)
    }

    #[cfg(not(feature = "multi_memory"))]
    #[inline(always)]
    fn memory_director_raw(&self, ptr: isize) -> isize {
        (self.get().memory_director_raw_ptr.unwrap())(ptr)
    }

    #[inline(always)]
    fn _main_raw(&self) -> wasip1::Errno {
        (self.get()._main_ptr)()
    }

    #[inline(always)]
    fn _reset_raw(&self) {
        (self.get()._reset_ptr)()
    }

    #[inline(always)]
    fn _start_raw(&self) {
        (self.get()._start_ptr)()
    }
}

/// Expose a function in the generated Wasm to register structures that implement PseudoWasmTraits.
#[macro_export]
macro_rules! export_pseudo_wasm {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            $crate::export_pseudo_wasm!($name; &[<$name:upper>]);
        }
    };

    ($name:ident; $holder:expr) => {
        const _: () = {
            const fn __asserter(_: &impl $crate::file::PseudoWasmTrait) {}

            __asserter($holder);
        };

        $crate::__private::paste::paste! {
            #[unsafe(no_mangle)]
            pub extern "C" fn [<__wasi_export_pseudo_wasm_ $name>](ptrs: PseudoWasmSimple) {
                let holder = $holder;
                holder.receive_pseudo_wasm(ptrs);
            }
        }
    };
}

/// Like `import_wasm!`, but for pseudo-Wasm modules.
#[macro_export]
macro_rules! import_pseudo_wasm {
    ($name:ident) => {
        $crate::__private::paste::paste! {
            $crate::import_pseudo_wasm!($name; &[<$name:upper>]);
        }
    };

    ($name:ident; $holder:expr) => {
        const _: () = {
            const fn __asserter(_: &impl $crate::file::PseudoWasmTrait) {}

            __asserter($holder);
        };

        compile_error!("The `import_pseudo_wasm!` macro with a holder argument is not implemented yet. Please implement it to import pseudo-Wasm modules with a specific holder.")
    };
}

#[derive(Deb)]
pub struct StandardPseudoWasmMultipleHolder {
    #[cfg(feature = "threads")]
    pub holders: parking_lot::RwLock<smallvec::SmallVec<[PseudoWasmSimple; 4]>>,

    #[cfg(not(feature = "threads"))]
    pub holders: UnsafeCell<smallvec::SmallVec<[PseudoWasmSimple; 4]>>,
}

#[derive(Debug, Clone)]
pub struct StandardPseudoWasmMultipleHolderInstant<'a> {
    refer: &'a StandardPseudoWasmMultipleHolder,
    refers_to: usize,
}

impl StandardPseudoWasmMultipleHolder {
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "threads")]
            holders: parking_lot::RwLock::new(smallvec::SmallVec::new_const()),
            #[cfg(not(feature = "threads"))]
            holders: UnsafeCell::new(smallvec::SmallVec::new_const()),
        }
    }

    pub fn add_holder(&mut self, holder: PseudoWasmSimple) {
        #[cfg(feature = "threads")]
        {
            self.holders.write().push(holder);
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { (*self.holders.get()).push(holder) };
        }
    }

    pub fn get_holder(&self, index: usize) -> &PseudoWasmSimple {
        #[cfg(feature = "threads")]
        {
            &self.holders.read()[index]
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { &(*self.holders.get())[index] }
        }
    }
}

impl PseudoWasmTrait for StandardPseudoWasmMultipleHolder {
    type Generated = usize;

    fn restore(&self, id: Self::Generated) -> impl WasmAccessDynCompatibleRaw {
        StandardPseudoWasmMultipleHolderInstant {
            refer: self,
            refers_to: id,
        }
    }

    fn receive_pseudo_wasm(&self, ptrs: PseudoWasmSimple) -> Self::Generated {
        // This is a placeholder implementation. You would need to implement logic to manage multiple holders.
        // For example, you could store the received `ptrs` in a vector or handle them based on some identifier.
        // Here we simply add the new holder to the list.
        let mut new_holder = self.clone();
        new_holder.add_holder(ptrs);
        *self = new_holder;
        self.holders.len() - 1
    }
}

impl WasmAccessDynCompatibleRaw for StandardPseudoWasmMultipleHolderInstant<'_> {
    #[inline(always)]
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        (self.get_holder(self.refers_to).memcpy_raw_ptr)(offset, src, len)
    }

    #[inline(always)]
    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        (self.get_holder(self.refers_to).memcpy_to_raw_ptr)(offset, src, len)
    }

    #[cfg(not(feature = "multi_memory"))]
    #[inline(always)]
    fn memory_director_raw(&self, ptr: isize) -> isize {
        (self
            .get_holder(self.refers_to)
            .memory_director_raw_ptr
            .unwrap())(ptr)
    }

    #[inline(always)]
    fn _main_raw(&self) -> wasip1::Errno {
        (self.get_holder(self.refers_to)._main_ptr)()
    }

    #[inline(always)]
    fn _reset_raw(&self) {
        (self.get_holder(self.refers_to)._reset_ptr)()
    }

    #[inline(always)]
    fn _start_raw(&self) {
        (self.get_holder(self.refers_to)._start_ptr)()
    }
}
