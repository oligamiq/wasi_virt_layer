// https://github.com/bytecodealliance/wasmtime/blob/cff811b55e8b715e037226f2f3c36c65676d319a/crates/wasi-preview1-component-adapter/src/lib.rs#L1655

/// WASI args module
pub mod args;
/// WASI clock module
pub mod clock;
/// WASI env module
pub mod env;
/// WASI file module
pub mod file;
/// WASI poll module
pub mod poll;
/// WASI process module
pub mod process;
/// WASI random module
pub mod random;
/// WASI sched module
pub mod sched;
#[cfg(feature = "threads")]
/// WASI thread module
pub mod thread;
/// Internal module for wrapping unreachable code
pub mod wrap_unreachable;

/// Internal helper macro for handle `self` and other identifiers.
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

    (@through, $($wasm:ident),* $(,)? => $callback:path, $($ex:tt)*) => {
        $crate::__as_t!(@through_inner; $($wasm),*, ; => $callback, $($ex)*);
    };

    (@through_inner; , ; $(,)? $($tail:ident),* => $callback:path, $($ex:tt)*) => {
        $callback!($($ex)*, $($tail),*);
    };

    (@through_inner; self, $($left:ident),* $(,)?; $(,)? $($tail:ident),* => $callback:path, $($ex:tt)*) => {
        $crate::__as_t!(@through_inner; $($left),*, ; $($tail),*, __self => $callback, $($ex)*);
    };

    (@through_inner; $pop:ident, $($left:ident),* $(,)?; $(,)? $($tail:ident),* => $callback:path, $($ex:tt)*) => {
        $crate::__as_t!(@through_inner; $($left),*, ; $($tail),*, $pop => $callback, $($ex)*);
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
