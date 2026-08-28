#![allow(dead_code)]

/// A constant-time binary search map for use in `const` contexts.
#[derive(Debug, Clone, Copy)]
pub struct ConstBinaryMap<'a, K, V: Copy, const LEN: usize> {
    keys: [usize; LEN],
    values: [V; LEN],
    __marker: core::marker::PhantomData<&'a K>,
}

impl<'a, K, V: Copy, const LEN: usize> ConstBinaryMap<'a, K, V, LEN> {
    /// Creates a new `ConstBinaryMap` from separate keys and values arrays.
    pub const fn from_key_and_values(keys: [usize; LEN], values: [V; LEN]) -> Self {
        let mut tuples = merge_arrays_to_tuples(keys, values);

        quicksort_internal(tuples.as_mut_ptr(), 0, (LEN - 1) as isize);

        let (keys, values) = split_tuples_to_arrays(&tuples);

        Self {
            keys,
            values,
            __marker: core::marker::PhantomData,
        }
    }

    /// Creates a new `ConstBinaryMap` from an array of key-value tuples.
    pub const fn from_key_values(mut key_values: [(usize, V); LEN]) -> Self {
        quicksort_internal(key_values.as_mut_ptr(), 0, (LEN - 1) as isize);

        let (keys, values) = split_tuples_to_arrays(&key_values);

        Self {
            keys,
            values,
            __marker: core::marker::PhantomData,
        }
    }

    /// Retrieves a value by its key using binary search.
    pub const fn get(&'a self, key: usize) -> Option<&'a V> {
        let mut low = 0;
        let mut high = LEN as isize - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if self.keys[mid as usize] == key {
                return Some(&self.values[mid as usize]);
            } else if self.keys[mid as usize] < key {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_const_binary_map_sorting() {
        const KEYS: [usize; 5] = [1, 3, 9, 20, 35];
        const VALUES: [&str; 5] = ["a", "b", "c", "d", "e"];
        const MAP: ConstBinaryMap<usize, &str, 5> =
            ConstBinaryMap::from_key_and_values(KEYS, VALUES);

        assert_eq!(MAP.get(1), Some(&"a"));
        assert_eq!(MAP.get(3), Some(&"b"));
        assert_eq!(MAP.get(9), Some(&"c"));
        assert_eq!(MAP.get(20), Some(&"d"));
        assert_eq!(MAP.get(35), Some(&"e"));
        assert_eq!(MAP.get(2), None);
    }

    #[test]
    fn test_const_binary_map_non_sorted() {
        const KEYS: [usize; 5] = [35, 20, 9, 3, 1];
        const VALUES: [&str; 5] = ["e", "d", "c", "b", "a"];
        const MAP: ConstBinaryMap<usize, &str, 5> =
            ConstBinaryMap::from_key_and_values(KEYS, VALUES);

        assert_eq!(MAP.get(1), Some(&"a"));
        assert_eq!(MAP.get(3), Some(&"b"));
        assert_eq!(MAP.get(9), Some(&"c"));
        assert_eq!(MAP.get(20), Some(&"d"));
        assert_eq!(MAP.get(35), Some(&"e"));
        assert_eq!(MAP.get(2), None);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [(3, 'c'), (1, 'a'), (2, 'b')];
        quicksort_internal(arr.as_mut_ptr(), 0, (arr.len() - 1) as isize);
        assert_eq!(arr, [(1, 'a'), (2, 'b'), (3, 'c')]);
    }

    #[test]
    fn test_quick_sort_const() {
        const ARR: [(usize, char); 3] = {
            let mut arr = [(3, 'c'), (1, 'a'), (2, 'b')];
            quicksort_internal(arr.as_mut_ptr(), 0, (arr.len() - 1) as isize);
            arr
        };
        assert_eq!(ARR, [(1, 'a'), (2, 'b'), (3, 'c')]);
    }
}

/// https://github.com/slightlyoutofphase/staticvec/blob/a3557755b9ee29238e98302cfab550a75675f339/src/utils.rs#L178
/// A simple quicksort function for internal use, called in
/// ['quicksorted_unstable`](crate::StaticVec::quicksorted_unstable).
#[inline]
pub(crate) const fn quicksort_internal<T: Copy>(
    values: *mut (usize, T),
    mut low: isize,
    mut high: isize,
) {
    // We call this function from exactly one place where `low` and `high` are known to be within an
    // appropriate range before getting passed into it, so there's no need to check them again here.
    // We also know that `values` will never be null, so we can safely give an optimizer hint here.
    loop {
        let mut i = low;
        let mut j = high;
        unsafe {
            let (p, _) = *values.offset(low + ((high - low) >> 1));
            loop {
                while (*values.offset(i)).0 < p {
                    i += 1;
                }
                while (*values.offset(j)).0 > p {
                    j -= 1;
                }
                if i <= j {
                    if i != j {
                        let q = *values.offset(i);
                        *values.offset(i) = *values.offset(j);
                        *values.offset(j) = q;
                    }
                    i += 1;
                    j -= 1;
                }
                if i > j {
                    break;
                }
            }
        }
        if j - low < high - i {
            if low < j {
                quicksort_internal(values, low, j);
            }
            low = i;
        } else {
            if i < high {
                quicksort_internal(values, i, high)
            }
            high = j;
        }
        if low >= high {
            break;
        }
    }
}

pub(crate) const fn merge_arrays_to_tuples<T: Copy, U: Copy, const N: usize>(
    a: [T; N],
    b: [U; N],
) -> [(T, U); N] {
    use const_for::const_for;
    let mut key_with_values = EmbeddedArrayBuilder::new();
    const_for!(i in 0..N => {
        key_with_values.push((a[i], b[i]));
    });
    key_with_values.build()
}

pub(crate) const fn split_tuples_to_arrays<T: Copy, U: Copy, const N: usize>(
    tuples: &[(T, U); N],
) -> ([T; N], [U; N]) {
    use const_for::const_for;
    let mut keys = EmbeddedArrayBuilder::new();
    let mut values = EmbeddedArrayBuilder::new();
    const_for!(i in 0..N => {
        keys.push(tuples[i].0);
        values.push(tuples[i].1);
    });
    (keys.build(), values.build())
}

/// A fixed-capacity array builder that can be used in `const` context.
#[derive(Debug, Clone, Copy)]
pub struct EmbeddedArrayBuilder<T: Copy, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

impl<T: Copy, const N: usize> EmbeddedArrayBuilder<T, N> {
    /// Creates a new `EmbeddedArrayBuilder`.
    pub const fn new_const() -> Self {
        Self {
            data: [None; N],
            len: 0,
        }
    }

    /// Creates a new `EmbeddedArrayBuilder`.
    pub const fn new() -> Self {
        Self::new_const()
    }

    /// Pushes a value into the builder. Returns `Some(value)` if the builder is full.
    pub const fn push(&mut self, value: T) -> Option<T> {
        if self.len < N {
            self.data[self.len] = Some(value);
            self.len += 1;
            None
        } else {
            Some(value)
        }
    }

    /// Removes the value at the given index.
    pub const fn remove(&mut self, index: usize) -> Option<T> {
        if index < N {
            let old_value = self.data[index];

            use const_for::const_for;

            const_for!(i in index..(self.len - 1) => {
                self.data[i] = self.data[i + 1];
            });
            self.data[self.len - 1] = None;

            self.len -= 1;
            old_value
        } else {
            None
        }
    }

    /// Pops the last value from the builder.
    pub const fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            self.data[self.len].take()
        } else {
            None
        }
    }

    /// Returns the number of elements in the builder.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns a reference to the element at the given index.
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// Sets the value at the given index.
    pub const fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < N {
            let old_value = self.data[index];
            self.data[index] = Some(value);
            old_value
        } else {
            None
        }
    }

    /// Returns whether the builder is full.
    pub const fn check_len(&self) -> bool {
        self.len == N
    }

    /// Builds the final array. Panics if the builder is not full.
    pub const fn build(self) -> [T; N] {
        use const_for::const_for;

        let first = self.data.first().unwrap().unwrap();
        let mut array = [first; N];
        const_for!(i in 0..N => {
            if let Some(value) = self.data[i] {
                array[i] = value;
            } else {
                panic!("EmbeddedArrayBuilder is not full, cannot build array");
            }
        });

        array
    }

    /// Builds the final array. Panics if the builder is not full and `is_full` is true.
    pub const fn build_with_is_check(self, is_full: bool) -> [T; N] {
        use const_for::const_for;

        let first = self.data.first().unwrap().unwrap();
        let mut array = [first; N];
        const_for!(i in 0..N => {
            if let Some(value) = self.data[i] {
                array[i] = value;
            } else {
                if is_full {
                    panic!("EmbeddedArrayBuilder is not full, cannot build array");
                }
            }
        });

        array
    }
}

/// Allocates a buffer and initializes it using the provided function.
#[cfg(feature = "alloc")]
pub unsafe fn alloc_buff<T, R>(
    size: usize,
    init: impl FnOnce(&mut [T]) -> R,
) -> (alloc::boxed::Box<[T]>, R) {
    let mut buf = alloc::boxed::Box::<[T]>::new_uninit_slice(size);
    let mut buff =
        &mut *(unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut T, size) });
    let result = init(&mut buff);
    (unsafe { buf.assume_init() }, result)
}

/// Utility for ensuring a function is called only once.
pub struct InitOnce {
    is_init: core::sync::atomic::AtomicBool,
}

impl InitOnce {
    /// Creates a new `InitOnce` instance.
    pub const fn new_const() -> Self {
        Self {
            is_init: core::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Calls the provided function once.
    pub fn call_once(&self, f: impl FnOnce()) {
        if !self
            .is_init
            .swap(true, core::sync::atomic::Ordering::SeqCst)
        {
            f();
        }
    }
}

#[cfg(feature = "detect-wasi-reentrancy")]
#[macro_export]
#[doc(hidden)]
macro_rules! __enter_non_recursive_wasi_call {
    ($name:ident) => {
        let _non_recursive_wasi_call_guard =
            $crate::__private::utils::NonRecursiveWasiCallGuard::enter(stringify!($name));
    };
}

#[cfg(not(feature = "detect-wasi-reentrancy"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __enter_non_recursive_wasi_call {
    ($name:ident) => {};
}

/// Typically, WASI ABI calls are made using plug!.
/// However, there may be cases where you want to call the ABI directly from within plug!.
/// Doing so would result in recursion.
/// To address this, call the function through this macro.
/// When calling a function through this macro,
/// it becomes a proper WASI ABI call after plug is connected.
/// This allows you to call the ABI without causing recursion.
///
/// # Re-entrancy warning
///
/// "Non-recursive" describes how the generated import is connected; it cannot prevent the host
/// implementation from calling back into the virtualized WASI function. The host sink must
/// terminate outside the VFS. In particular, forwarding virtual stderr to a `std::io::stderr()`
/// that is backed by the same VFS recursively enters `fd_write`.
///
/// Enable the `detect-wasi-reentrancy` feature to guard this boundary. A synchronous re-entry on
/// the same thread traps immediately. The Wasm trap intentionally performs no diagnostic output,
/// because formatting the error through stdout or stderr could itself recurse.
///
/// Note: This ABI call is low-level. Verify the ABI and final host routing thoroughly.
///
/// ```rust
/// use wasi_virt_layer::wasip1;
///
/// unsafe fn fd_read(
///     fd: wasip1::Fd,
///     iovs: wasip1::IovecArray<'_>,
/// ) -> Result<wasip1::Size, wasip1::Errno> {
///     use crate::*;
///
///     let mut rp0 = core::mem::MaybeUninit::<wasip1::Size>::uninit();
///
///     let fd = fd as i32;
///     let iovs_ptr = iovs.as_ptr() as i32;
///     let iovs_len = iovs.len() as i32;
///     let rp0_ptr = rp0.as_mut_ptr() as i32;
///
///     let ret = wasi_virt_layer::non_recursive_wasi_snapshot_preview1!(
///         fd_read(
///             fd: i32,
///             iovs_ptr: i32,
///             iovs_len: i32,
///             rp0_ptr: i32
///         ) -> i32
///     );
///
///     match ret {
///         0 => Ok(unsafe { core::ptr::read(rp0.as_mut_ptr() as i32 as *const wasip1::Size) }),
///         _ => Err(unsafe { core::mem::transmute::<u16, wasip1::Errno>(ret as u16) }),
///     }
/// }
/// ```
#[macro_export]
macro_rules! non_recursive_wasi_snapshot_preview1 {
    (
        $name:ident ($($arg:ident : $arg_ty:ty),* $(,)?) -> $ret:ty
    ) => {
        {
        $crate::__enter_non_recursive_wasi_call!($name);

        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "non_recursive_wasi_snapshot_preview1")]
            unsafe extern "C" {
                pub fn $name($($arg: $arg_ty),*) -> $ret;
            }

            unsafe { $name($($arg),*) }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            #[allow(unused_variables)]
            fn $name($(_: $arg_ty),*) -> $ret {
                unimplemented!("non_recursive_wasi_snapshot_preview1 is only supported on wasm32 target")
            }

            $name($($arg),*)
        }
    }
    };
}

/// Detects synchronous re-entry through the non-recursive WASI import boundary.
///
/// This type is public only so exported macros can reference it across crate boundaries.
#[cfg(feature = "detect-wasi-reentrancy")]
#[doc(hidden)]
pub struct NonRecursiveWasiCallGuard;

#[cfg(feature = "detect-wasi-reentrancy")]
std::thread_local! {
    static NON_RECURSIVE_WASI_CALL_ACTIVE: core::cell::Cell<Option<&'static str>> =
        const { core::cell::Cell::new(None) };
}

#[cfg(feature = "detect-wasi-reentrancy")]
impl NonRecursiveWasiCallGuard {
    /// Enters the guarded non-recursive WASI boundary.
    #[inline]
    pub fn enter(function: &'static str) -> Self {
        NON_RECURSIVE_WASI_CALL_ACTIVE.with(|active| {
            if let Some(previous) = active.replace(Some(function)) {
                active.set(Some(previous));
                reentrant_non_recursive_wasi_call(previous, function);
            }
        });
        Self
    }
}

#[cfg(feature = "detect-wasi-reentrancy")]
impl Drop for NonRecursiveWasiCallGuard {
    fn drop(&mut self) {
        NON_RECURSIVE_WASI_CALL_ACTIVE.with(|active| active.set(None));
    }
}

#[cfg(all(feature = "detect-wasi-reentrancy", target_arch = "wasm32"))]
#[cold]
#[inline(never)]
fn reentrant_non_recursive_wasi_call(_previous: &'static str, _current: &'static str) -> ! {
    core::arch::wasm32::unreachable()
}

#[cfg(all(feature = "detect-wasi-reentrancy", not(target_arch = "wasm32")))]
#[cold]
#[inline(never)]
fn reentrant_non_recursive_wasi_call(previous: &'static str, current: &'static str) -> ! {
    panic!(
        "re-entrant non-recursive WASI call detected: `{current}` entered while `{previous}` was active"
    )
}

#[cfg(all(test, feature = "detect-wasi-reentrancy"))]
mod non_recursive_wasi_call_guard_tests {
    use super::NonRecursiveWasiCallGuard;

    #[test]
    fn rejects_reentrant_call_and_resets_after_drop() {
        let first = NonRecursiveWasiCallGuard::enter("fd_write");
        let reentry = std::panic::catch_unwind(|| {
            let _second = NonRecursiveWasiCallGuard::enter("fd_write");
        });
        assert!(reentry.is_err());

        drop(first);
        let _next_call = NonRecursiveWasiCallGuard::enter("fd_write");
    }
}

pub struct UnsafeOnceCell<T> {
    value: core::cell::UnsafeCell<Option<T>>,
}

unsafe impl<T: Sync> Sync for UnsafeOnceCell<T> {}
unsafe impl<T: Send> Send for UnsafeOnceCell<T> {}

impl<T> UnsafeOnceCell<T> {
    /// Creates a new `UnsafeOnceCell` that is not initialized.
    pub const fn new() -> Self {
        Self {
            value: core::cell::UnsafeCell::new(None),
        }
    }

    /// Initializes the cell with the provided value. This is unsafe because it must only be called once, and the caller must ensure that no threads are accessing the cell until initialization is complete.
    pub unsafe fn init(&self, value: T) -> Result<(), ()> {
        unsafe {
            if (*self.value.get()).is_some() {
                return Err(());
            }
            *self.value.get() = Some(value);
        }
        Ok(())
    }

    /// Gets a reference to the value. This is unsafe because it assumes that the value has been initialized and will not be modified after initialization.
    pub unsafe fn get(&self) -> &T {
        unsafe { (*self.value.get()).as_ref().unwrap() }
    }
}

impl<T> core::ops::Deref for UnsafeOnceCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.get() }
    }
}

impl<T: Default> UnsafeOnceCell<T> {
    /// Initializes the cell with the default value of `T`. This is a convenience method that can be used when `T` implements `Default`.
    pub unsafe fn init_default(&self) -> Result<(), ()> {
        unsafe { self.init(T::default()) }
    }
}
