use crate::__private::wasip1::*;
use crate::clock::Clock;
use crate::memory::WasmAccess;
use crate::transporter::non_recursive_sched_yield;

/// A simple implementation of `poll_oneoff` that performs a blocking sleep/yield.
/// This version uses the provided Clock trait for time operations.
pub struct WaitPoll<C: Clock = crate::clock::StandardClock>(std::marker::PhantomData<C>);

impl<C: Clock> crate::poll::PollOneoff for WaitPoll<C> {
    #[inline(never)]
    fn poll_oneoff<Wasm: WasmAccess>(
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

        let (userdata, event_type, timeout, _precision, flags) = unsafe {
            let base_ptr = subscriptions_ptr as *const u8;

            let userdata = Wasm::load_le::<u64>(base_ptr as *const u64);
            let event_type = Wasm::load_le::<u8>(base_ptr.add(8) as *const u8);
            let timeout = Wasm::load_le::<Timestamp>(base_ptr.add(24) as *const Timestamp);
            let precision = Wasm::load_le::<Timestamp>(base_ptr.add(32) as *const Timestamp);
            let flags = Wasm::load_le::<Subclockflags>(base_ptr.add(40) as *const Subclockflags);

            (userdata, event_type, timeout, precision, flags)
        };

        // TODO: For now, we only support clock subscriptions.
        if event_type != EVENTTYPE_CLOCK.raw() {
            return ERRNO_NOTSUP;
        }

        // Use the Clock trait to get the current time
        fn get_now<Wasm: WasmAccess, Clock: crate::clock::Clock>() -> Timestamp {
            let mut time_buf = 0u64;
            let _ = Clock::clock_time_get::<Wasm>(CLOCKID_REALTIME, 0, &mut time_buf);
            time_buf
        }

        // Perform the wait
        // Note: precision is NOT subtracted from end_time. Precision is advisory only
        // and represents the maximum tolerable imprecision, not a timeout adjustment.
        let end_time = if (flags & SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME) != 0 {
            timeout
        } else {
            get_now::<Wasm, C>().saturating_add(timeout)
        };

        loop {
            unsafe { non_recursive_sched_yield() };

            let now = get_now::<Wasm, C>();

            if now >= end_time {
                break;
            }
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
