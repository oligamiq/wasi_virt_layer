// https://github.com/bytecodealliance/wasmtime/blob/cff811b55e8b715e037226f2f3c36c65676d319a/crates/wasi-preview1-component-adapter/src/lib.rs#L1655

pub mod env;
pub mod file;
pub mod process;
#[cfg(feature = "threads")]
pub mod thread;

/// @through Iterate through the identifiers, replacing `self` with `__self`, and call the callback with all identifiers.
/// @as_t Replace `self` with `__self` and call the callback with the identifier and type.
#[macro_export]
macro_rules! __as_t {
    (@as_t, self) => {
        type T = $crate::__private::__self;
    };

    (@as_t, __self) => {
        type T = $crate::__private::__self;
    };

    (@as_t, $wasm:ty) => {
        type T = $wasm;
    };

    (@as_ident, self) => {
        __self
    };

    (@as_ident, __self) => {
        __self
    };

    (@as_ident, $wasm:ident) => {
        $wasm
    };

    (@through, $($wasm:tt),* => $callback:path, $($ex:tt)*) => {
        $crate::__as_t!(@through_inner; $($wasm),*; => $callback, $($ex)*);
    };

    (@through_inner; $(,)? ; $(,)? $($tail:ident),* => $callback:path, $($ex:tt)*) => {
        $callback!($($ex)*, $($tail),*);
    };

    (@through_inner; self, $($left:ident),*; $(,)? $($tail:ident),* => $callback:path, $($ex:tt)*) => {
        $crate::__as_t!(@through_inner; $($left),*; $($tail),*, __self => $callback, $($ex)*);
    };

    (@through_inner; $pop:ident $(,)? $($left:ident),*; $(,)? $($tail:ident),* => $callback:path, $($ex:tt)*) => {
        $crate::__as_t!(@through_inner; $($left),*; $($tail),*, $pop => $callback, $($ex)*);
    };

    // (@through, self => $callback:path, $($ex:tt)*) => {
    //     $callback!($($ex)*, __self, $crate::__private::__self);
    // };

    // (@through, $wasm:ident => $callback:path, $($ex:tt)*) => {
    //     $callback!($($ex)*, $wasm, $wasm);
    // };

    // (@through, self => $callback:path) => {
    //     $callback!(__self, $crate::__private::__self);
    // };

    // (@through, $wasm:ident => $callback:path) => {
    //     $callback!($wasm, $wasm);
    // };
}
