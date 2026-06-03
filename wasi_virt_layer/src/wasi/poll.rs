/// WASI poll wait_poll utilities.
pub mod wait_poll;
use crate::memory::WasmAccess;
use crate::{__private::wasip1::*, memory::WasmAccessName};
pub use wait_poll::{DefaultWaitPoll, WaitPoll};

/// Trait for handling the `poll_oneoff` WASI function.
pub trait PollOneoff {
    /// Concurrently polls for events.
    fn poll_oneoff<Wasm: WasmAccess + WasmAccessName + 'static>(
        subscriptions: *const Subscription,
        events: *mut Event,
        nsubscriptions: Size,
        nevents: *mut Size,
    ) -> Errno;
}

/// Default implementation of `PollOneoff`.
pub struct DefaultPoll;

impl PollOneoff for DefaultPoll {
    fn poll_oneoff<Wasm: WasmAccess + WasmAccessName + 'static>(
        subscriptions: *const Subscription,
        events: *mut Event,
        nsubscriptions: Size,
        nevents: *mut Size,
    ) -> Errno {
        let subscriptions = subscriptions as i32;
        let events = events as i32;
        let nsubscriptions = nsubscriptions as i32;
        let nevents = nevents as i32;

        let ret = crate::non_recursive_wasi_snapshot_preview1!(
            poll_oneoff(
                subscriptions: i32,
                events: i32,
                nsubscriptions: i32,
                nevents: i32
            ) -> i32
        );

        unsafe { core::mem::transmute::<u16, Errno>(ret as u16) }
    }
}

/// Plugs the poll ecosystem by defining necessary handlers.
///
/// ```rust,no_run
/// use wasi_virt_layer::prelude::*;
///
/// import_wasm!(test_wasm);
///
/// plug_poll!(DefaultPoll, test_wasm);
/// ```
#[macro_export]
macro_rules! plug_poll {
    ($ty:ty, $($wasm:ident),+) => {
        const _: () = {
            type __TYPE = $ty;
        };

        $crate::__private::paste::paste! {
            $(
                #[unsafe(no_mangle)]
                #[cfg(target_os = "wasi")]
                pub unsafe extern "C" fn [<__wasip1_vfs_ $wasm _poll_oneoff>](
                    subscriptions: *const $crate::__private::wasip1::Subscription,
                    events: *mut $crate::__private::wasip1::Event,
                    nsubscriptions: $crate::__private::wasip1::Size,
                    nevents: *mut $crate::__private::wasip1::Size,
                ) -> $crate::__private::wasip1::Errno {
                    $crate::__as_t!(@as_t, $wasm);
                    <$ty as $crate::poll::PollOneoff>::poll_oneoff::<T>(subscriptions, events, nsubscriptions, nevents)
                }
            )*
        }
    };
    ($($wasm:ident),*) => {
        $crate::__as_t!(@through, $($wasm),* => $crate::plug_poll, @inner);
    };
    (@inner, $($wasm:ident),*) => {
        $crate::plug_poll!($crate::poll::DefaultPoll, $($wasm),*);
    };
}
