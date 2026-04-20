#![cfg(feature = "multiple-fs")]

use crate::__private::wasip1;
use crate::file::multiple::wasm::WasmAccessDynCompatibleTuple;
use crate::memory::{WasmAccessDynCompatibleRaw, WasmAccessNameDynCompatible};

/// A structure holding function pointers for a pseudo-Wasm module.
/// This has separated name values so you must build in PseudoWasmTrait's receiver fn.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PseudoWasmSimpleBuilder {
    pub name_ptr: *const u8,
    pub name_len: usize,
    pub _main_ptr: fn() -> wasip1::Errno,
    pub _reset_ptr: fn(),
    pub _start_ptr: fn(),
    pub memcpy_raw_ptr: fn(*mut u8, *const u8, usize),
    pub memcpy_to_raw_ptr: fn(*mut u8, *const u8, usize),
    #[cfg(not(feature = "multi_memory"))]
    pub memory_director_raw_ptr: Option<fn(isize) -> isize>, // Optional for multi-memory support
}

impl PseudoWasmSimpleBuilder {
    pub fn build(self) -> PseudoWasmSimple {
        let name = if self.name_ptr.is_null() || self.name_len == 0 {
            smallstr::SmallString::new()
        } else {
            unsafe {
                let slice = core::slice::from_raw_parts(self.name_ptr, self.name_len);
                smallstr::SmallString::from_str(core::str::from_utf8_unchecked(slice))
            }
        };

        PseudoWasmSimple {
            name,
            _main_ptr: self._main_ptr,
            _reset_ptr: self._reset_ptr,
            _start_ptr: self._start_ptr,
            memcpy_raw_ptr: self.memcpy_raw_ptr,
            memcpy_to_raw_ptr: self.memcpy_to_raw_ptr,
            #[cfg(not(feature = "multi_memory"))]
            memory_director_raw_ptr: self.memory_director_raw_ptr,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PseudoWasmSimple {
    pub name: smallstr::SmallString<[u8; 32]>,
    pub _main_ptr: fn() -> wasip1::Errno,
    pub _reset_ptr: fn(),
    pub _start_ptr: fn(),
    pub memcpy_raw_ptr: fn(*mut u8, *const u8, usize),
    pub memcpy_to_raw_ptr: fn(*mut u8, *const u8, usize),
    #[cfg(not(feature = "multi_memory"))]
    pub memory_director_raw_ptr: Option<fn(isize) -> isize>, // Optional for multi-memory support
}

impl PseudoWasmSimple {
    pub fn with_name(&self, f: &mut dyn FnMut(&str)) {
        if self.name.is_empty() {
            f("");
        } else {
            f(self.name.as_str());
        }
    }
}

/// A specific trait for static pseudo-Wasm structures.
/// It provides a function that receives the underlying pointer group and requires dynamic Wasm compatibility.
pub trait PseudoWasmTrait {
    type Generated;

    fn restore(&self, generated: Self::Generated) -> impl WasmAccessDynCompatibleTuple;

    /// Receives the struct holding the pointer group to create an instance.
    fn receive_pseudo_wasm(&self, ptrs: PseudoWasmSimpleBuilder) -> Self::Generated;
}

/// A standard struct that holds the pointer group and implements `PseudoWasmTrait`
/// as well as `WasmAccessDynCompatibleRaw`.
/// Note: This struct does NOT (and cannot) implement `WasmAccessRaw` directly.
#[derive(Debug)]
pub struct StandardPseudoWasmHolder {
    #[cfg(feature = "threads")]
    pub once: core::sync::atomic::AtomicBool,
    pub simple: core::cell::UnsafeCell<Option<PseudoWasmSimple>>,
}

impl AsRef<StandardPseudoWasmHolder> for StandardPseudoWasmHolder {
    fn as_ref(&self) -> &StandardPseudoWasmHolder {
        self
    }
}

impl StandardPseudoWasmHolder {
    pub const fn new_const() -> Self {
        Self {
            #[cfg(feature = "threads")]
            once: core::sync::atomic::AtomicBool::new(false),
            simple: core::cell::UnsafeCell::new(None),
        }
    }

    pub fn get<'s>(&'s self) -> &'s PseudoWasmSimple {
        #[cfg(feature = "trace")]
        unsafe { (*self.simple.get()).as_ref().expect("PseudoWasmSimple is not initialized yet") }

        #[cfg(not(feature = "trace"))]
        unsafe { (*self.simple.get()).as_ref().unwrap() }
    }
}

unsafe impl Send for StandardPseudoWasmHolder {}
unsafe impl Sync for StandardPseudoWasmHolder {}

impl PseudoWasmTrait for StandardPseudoWasmHolder {
    type Generated = ();

    fn restore(&self, _: Self::Generated) -> impl WasmAccessDynCompatibleTuple {
        self
    }

    fn receive_pseudo_wasm(&self, ptrs: PseudoWasmSimpleBuilder) -> Self::Generated {
        let ptrs = ptrs.build();
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
            unsafe { *self.simple.get() = Some(ptrs) };
        }
        #[cfg(not(feature = "threads"))]
        {
            #[cfg_attr(not(feature = "trace"), allow(unused_variables))]
            if let Some(ptr) = unsafe { &mut *self.simple.get() }.as_mut() {
                #[cfg(feature = "trace")]
                {
                    panic!("PseudoWasmSimple is already initialized with: {:?}", ptr);
                }
            } else {
                unsafe { *self.simple.get() = Some(ptrs) };
            }
        }
    }
}

impl WasmAccessNameDynCompatible for StandardPseudoWasmHolder {
    fn with_name(&self, f: &mut dyn FnMut(&str)) {
        self.get().with_name(f);
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
            pub extern "C" fn [<__wasi_export_pseudo_wasm_ $name>](ptrs: $crate::file::PseudoWasmSimpleBuilder) {
                let holder = $holder;
                holder.receive_pseudo_wasm(ptrs);
            }
        }
    };
}

#[derive(Debug)]
pub struct StandardPseudoWasmMultipleHolder {
    #[cfg(feature = "threads")]
    pub holders: parking_lot::RwLock<smallvec::SmallVec<[PseudoWasmSimple; 4]>>,

    #[cfg(not(feature = "threads"))]
    pub holders: core::cell::UnsafeCell<smallvec::SmallVec<[PseudoWasmSimple; 4]>>,
}

unsafe impl Send for StandardPseudoWasmMultipleHolder {}
unsafe impl Sync for StandardPseudoWasmMultipleHolder {}

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
            holders: core::cell::UnsafeCell::new(smallvec::SmallVec::new_const()),
        }
    }

    pub fn add_holder(&self, holder: PseudoWasmSimple) {
        #[cfg(feature = "threads")]
        {
            self.holders.write().push(holder);
        }
        #[cfg(not(feature = "threads"))]
        {
            unsafe { (*self.holders.get()).push(holder) };
        }
    }

    pub fn get_holder_with<R>(&self, id: usize, f: impl FnOnce(&PseudoWasmSimple) -> R) -> R {
        #[cfg(feature = "threads")]
        {
            let holders = self.holders.read();
            let holder = &holders[id];
            f(holder)
        }
        #[cfg(not(feature = "threads"))]
        {
            let holders = unsafe { &*self.holders.get() };
            let holder = &holders[id];
            f(holder)
        }
    }
}

impl PseudoWasmTrait for StandardPseudoWasmMultipleHolder {
    type Generated = usize;

    fn restore(&self, id: Self::Generated) -> impl WasmAccessDynCompatibleTuple {
        StandardPseudoWasmMultipleHolderInstant {
            refer: self,
            refers_to: id,
        }
    }

    fn receive_pseudo_wasm(&self, ptrs: PseudoWasmSimpleBuilder) -> Self::Generated {
        self.add_holder(ptrs.build());

        #[cfg(feature = "threads")]
        let len = self.holders.read().len();
        #[cfg(not(feature = "threads"))]
        let len = unsafe { (*self.holders.get()).len() };

        len - 1
    }
}

impl WasmAccessNameDynCompatible for StandardPseudoWasmMultipleHolderInstant<'_> {
    fn with_name(&self, f: &mut dyn FnMut(&str)) {
        self.refer.get_holder_with(self.refers_to, |holder| {
            #[cfg(feature = "trace")]
            {
                if holder.name.is_empty() {
                    panic!("WasmAccessName is not initialized yet for holder: {:?}", holder);
                }
            }

            if holder.name.is_empty() {
                f("");
            } else {
                f(holder.name.as_str());
            }
        })

    }
}

impl WasmAccessDynCompatibleRaw for StandardPseudoWasmMultipleHolderInstant<'_> {
    #[inline(always)]
    fn memcpy_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        self.refer.get_holder_with(self.refers_to, |holder| (holder.memcpy_raw_ptr)(offset, src, len))
    }

    #[inline(always)]
    fn memcpy_to_raw(&self, offset: *mut u8, src: *const u8, len: usize) {
        self.refer.get_holder_with(self.refers_to, |holder| (holder.memcpy_to_raw_ptr)(offset, src, len))
    }

    #[cfg(not(feature = "multi_memory"))]
    #[inline(always)]
    fn memory_director_raw(&self, ptr: isize) -> isize {
        self.refer.get_holder_with(self.refers_to, |holder| (holder.memory_director_raw_ptr.unwrap())(ptr))
    }

    #[inline(always)]
    fn _main_raw(&self) -> wasip1::Errno {
        self.refer.get_holder_with(self.refers_to, |holder| (holder._main_ptr)())
    }

    #[inline(always)]
    fn _reset_raw(&self) {
        self.refer.get_holder_with(self.refers_to, |holder| (holder._reset_ptr)())
    }

    #[inline(always)]
    fn _start_raw(&self) {
        self.refer.get_holder_with(self.refers_to, |holder| (holder._start_ptr)())
    }
}
