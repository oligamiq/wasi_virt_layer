use crate::__private::wasip1::*;
use crate::memory::WasmAccess;
use crate::transporter::non_recursive_sched_yield;

/// A simple implementation of `poll_oneoff` that performs a blocking sleep/yield.
pub struct WaitPoll;

impl crate::poll::PollOneoff for WaitPoll {
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

        let (userdata, event_type, timeout, precision, flags) = unsafe {
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

        fn get_now() -> Timestamp {
            let sys_time = std::time::SystemTime::now();

            sys_time
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as Timestamp
        }

        // Perform the wait
        let end_time = if (flags & SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME) != 0 {
            timeout
        } else {
            get_now().saturating_add(timeout)
        }
        .saturating_sub(precision);

        loop {
            unsafe { non_recursive_sched_yield() };

            let now = get_now();

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
