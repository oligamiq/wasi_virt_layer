use crate::__private::wasip1::*;
use crate::clock::Clock;
use crate::memory::{WasmAccess, WasmAccessName};

#[cfg(target_os = "wasi")]
#[link(wasm_import_module = "wvl_poll")]
unsafe extern "C" {
    #[doc(hidden)]
    pub fn __wvl_poll_atomic_wait(addr: *mut u32, expected: u32, timeout: i64) -> i32;
}

#[cfg(not(target_os = "wasi"))]
#[doc(hidden)]
pub unsafe extern "C" fn __wvl_poll_atomic_wait(
    _addr: *mut u32,
    _expected: u32,
    _timeout: i64,
) -> i32 {
    2 // timed out dummy return
}

/// A simple implementation of `poll_oneoff` that performs a blocking sleep/yield.
/// This version uses the provided Clock trait for time operations.
pub struct WaitPoll<C: Clock = crate::clock::StandardClock>(core::marker::PhantomData<C>);

impl<C: Clock> crate::poll::PollOneoff for WaitPoll<C> {
    #[inline(never)]
    fn poll_oneoff<Wasm: WasmAccess + WasmAccessName + 'static>(
        subscriptions_ptr: *const Subscription,
        ret_event_ptr: *mut Event,
        nsubscriptions: Size,
        ret_stored_events_ptr: *mut Size,
    ) -> Errno {
        if nsubscriptions == 0 {
            return ERRNO_INVAL;
        }

        // TODO: For now, we only support a single subscription.
        if nsubscriptions > 1 {
            return ERRNO_NOTSUP;
        }

        let (userdata, event_type, clock_id, timeout, _precision, flags) = unsafe {
            let base_ptr = subscriptions_ptr as *const u8;

            let userdata = Wasm::load_le::<u64>(base_ptr as *const u64);
            let event_type = Wasm::load_le::<u8>(base_ptr.add(8) as *const u8);
            let clock_id = Wasm::load_le::<Clockid>(base_ptr.add(16) as *const Clockid);
            let timeout = Wasm::load_le::<Timestamp>(base_ptr.add(24) as *const Timestamp);
            let precision = Wasm::load_le::<Timestamp>(base_ptr.add(32) as *const Timestamp);
            let flags = Wasm::load_le::<Subclockflags>(base_ptr.add(40) as *const Subclockflags);

            (userdata, event_type, clock_id, timeout, precision, flags)
        };

        // TODO: For now, we only support clock subscriptions.
        if event_type != EVENTTYPE_CLOCK.raw() {
            return ERRNO_NOTSUP;
        }

        // Use the Clock trait to get the current time
        // We use crate::__self::__self to ensure we don't use the memory director
        // for our own local buffer.
        fn get_now<Clock: crate::clock::Clock>(clock_id: Clockid) -> Timestamp {
            let mut time_buf = 0u64;
            let _ = Clock::clock_time_get::<crate::__self::__self>(clock_id, 0, &mut time_buf);
            time_buf
        }

        // Perform the wait
        // Note: precision is NOT subtracted from end_time. Precision is advisory only
        // and represents the maximum tolerable imprecision, not a timeout adjustment.
        let end_time = if (flags & SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME) != 0 {
            timeout
        } else {
            get_now::<C>(clock_id).saturating_add(timeout)
        };

        let mut dummy = 0;
        loop {
            let now = get_now::<C>(clock_id);

            if now >= end_time {
                break;
            }

            unsafe {
                let timeout_ns = end_time.saturating_sub(now);
                __wvl_poll_atomic_wait(&mut dummy, 0, timeout_ns as i64);
                crate::transporter::non_recursive_sched_yield();
            };
        }

        // Write an event to the out buffer
        let event = Event {
            userdata,
            error: ERRNO_SUCCESS,
            type_: EVENTTYPE_CLOCK,
            fd_readwrite: EventFdReadwrite {
                nbytes: 0,
                flags: 0,
            },
        };

        Wasm::store_le(ret_event_ptr, event);
        Wasm::store_le(ret_stored_events_ptr, 1);

        ERRNO_SUCCESS
    }
}

/// Alias for WaitPoll using the default StandardClock implementation.
pub type DefaultWaitPoll = WaitPoll<crate::clock::StandardClock>;
