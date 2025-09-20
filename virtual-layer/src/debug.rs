pub(crate) fn out(buf: &[u8]) {
    unsafe {
        let ciovec_arr = [wasip1::Ciovec {
            buf: buf.as_ptr() as *const u8,
            buf_len: buf.len(),
        }];

        let mut rp0 = core::mem::MaybeUninit::<wasip1::Size>::uninit();
        wasip1::wasi_snapshot_preview1::fd_write(
            wasip1::FD_STDERR as i32,
            ciovec_arr.as_ptr() as i32,
            1,
            rp0.as_mut_ptr() as i32,
        );
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_call_indirect(tid: i32, idx: i32) {
    call_function::depth_write_out();
    out(b"debug_call_indirect: tid=");
    num_to_str(tid, out);
    out(b", idx=");
    num_to_str(idx, out);
    out(b"\n");
}

#[inline(never)]
fn num_to_str(n: i32, writer: impl Fn(&[u8])) {
    if n == 0 {
        writer(&[b'0']);
        return;
    }

    let mut buf = [0u8; 11];
    let mut i = buf.len();

    let negative = n < 0;
    let mut num = if negative { -(n as i64) } else { n as i64 };

    while num > 0 {
        i -= 1;
        buf[i] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    if negative {
        i -= 1;
        buf[i] = b'-';
    }

    writer(&buf[i..]);
}

#[inline(never)]
fn ptr_to_str(ptr: *const (), writer: impl Fn(&[u8])) {
    let mut buf = [0u8; 2 + core::mem::size_of::<usize>() * 2];
    buf[0] = b'0';
    buf[1] = b'x';

    let mut i = buf.len();
    let mut num = ptr as usize;

    if num == 0 {
        i -= 1;
        buf[i] = b'0';
    } else {
        while num > 0 {
            i -= 1;
            let digit = (num & 0xf) as u8;
            buf[i] = if digit < 10 {
                b'0' + digit
            } else {
                b'a' + (digit - 10)
            };
            num >>= 4;
        }
    }

    writer(&buf[i..]);
}

mod call_function {
    use super::*;

    thread_local! {
        static DEPTH: core::cell::Cell<u32> = core::cell::Cell::new(0);
    }

    #[inline(never)]
    pub(super) fn depth_write_out() {
        DEPTH.with(|d| depth_write_out_inner(d.get()));
    }

    #[inline(never)]
    fn depth_write_out_inner(depth: u32) {
        for _ in 0..depth {
            out(b">");
        }
    }

    const DECREASE_ERROR: &[u8] = b"Attempted to decrease depth below 0";

    #[inline(never)]
    fn decrease_with_write_out() {
        DEPTH.with(|d| {
            let current = d.get();
            if current > 0 {
                let current = current - 1;
                d.set(current);
                depth_write_out_inner(current);
            } else {
                out(DECREASE_ERROR);
            }
        });
    }

    #[inline(never)]
    fn increase_with_write_out() {
        DEPTH.with(|d| {
            let current = d.get();
            depth_write_out_inner(current);
            d.set(current + 1);
        });
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn debug_call_function_start(idx: i32) {
        increase_with_write_out();
        out(b"debug_call_function: idx=");
        num_to_str(idx, out);
        out(b"\n");
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn debug_call_function_end(idx: i32) {
        decrease_with_write_out();
        out(b"debug_call_function_end: idx=");
        num_to_str(idx, out);
        out(b"\n");
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_blind_print_etc_flag() {
    println!("debug_blind_print_etc_flag called");
    eprintln!("This is an error message from debug_blind_print_etc_flag");

    let str = format!("This is a formatted message: {}, {}", 42, "hello");
    out(str.as_bytes());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_atomic_wait(ptr: *const i32, expression: *const i32, timeout_ns: i64) {
    out(b"debug_atomic_wait: ptr=");
    ptr_to_str(ptr as *const (), out);
    out(b", expression=");
    ptr_to_str(expression as *const (), out);
    out(b", timeout_ns=");
    num_to_str(timeout_ns as i32, out);
    out(b"\n");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_something() {
    out(b"debug_something called\n");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_loop(idx: i32) {
    call_function::depth_write_out();
    out(b"debug_loop called: idx=");
    num_to_str(idx, out);
    out(b"\n");
}
