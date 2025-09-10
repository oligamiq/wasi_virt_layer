use crate::transporter;

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_call_indirect(tid: i32, idx: i32) {
    call_function::depth_write_out();
    transporter::Wasip1Transporter::write_to_stderr(b"debug_call_indirect: tid=").unwrap();
    num_to_str(tid, transporter::Wasip1Transporter::write_to_stderr).unwrap();
    transporter::Wasip1Transporter::write_to_stderr(b", idx=").unwrap();
    num_to_str(idx, transporter::Wasip1Transporter::write_to_stderr).unwrap();
    transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
}

fn num_to_str(
    n: i32,
    writer: impl Fn(&[u8]) -> Result<wasip1::Size, wasip1::Errno>,
) -> Result<(), wasip1::Errno> {
    if n == 0 {
        writer(&[b'0'])?;
        return Ok(());
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

    writer(&buf[i..])?;
    Ok(())
}

mod call_function {
    use super::*;

    thread_local! {
        static DEPTH: core::cell::Cell<u32> = core::cell::Cell::new(0);
    }

    pub(super) fn depth_write_out() {
        DEPTH.with(|d| depth_write_out_inner(d.get()));
    }

    fn depth_write_out_inner(depth: u32) {
        for _ in 0..depth {
            transporter::Wasip1Transporter::write_to_stderr(b">").unwrap();
        }
    }

    const DECREASE_ERROR: &[u8] = b"Attempted to decrease depth below 0";

    fn decrease_with_write_out() {
        DEPTH.with(|d| {
            let current = d.get();
            if current > 0 {
                let current = current - 1;
                d.set(current);
                depth_write_out_inner(current);
            } else {
                transporter::Wasip1Transporter::write_to_stderr(DECREASE_ERROR).unwrap();
            }
        });
    }

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
        transporter::Wasip1Transporter::write_to_stderr(b"debug_call_function: idx=").unwrap();
        num_to_str(idx, transporter::Wasip1Transporter::write_to_stderr).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn debug_call_function_end(idx: i32) {
        decrease_with_write_out();
        transporter::Wasip1Transporter::write_to_stderr(b"debug_call_function_end: idx=").unwrap();
        num_to_str(idx, transporter::Wasip1Transporter::write_to_stderr).unwrap();
        transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_blind_print_etc_flag() {
    println!("debug_blind_print_etc_flag called");
    eprintln!("This is an error message from debug_blind_print_etc_flag");

    let str = format!("This is a formatted message: {}, {}", 42, "hello");
    transporter::Wasip1Transporter::write_to_stderr(str.as_bytes()).unwrap();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug_atomic_wait(ptr: *const i32, expression: *const i32, timeout_ns: i64) {
    transporter::Wasip1Transporter::write_to_stderr(b"debug_atomic_wait: ptr=").unwrap();
    transporter::Wasip1Transporter::write_to_stderr(format!("{ptr:?}").as_bytes()).unwrap();
    transporter::Wasip1Transporter::write_to_stderr(b", expression=").unwrap();
    transporter::Wasip1Transporter::write_to_stderr(format!("{expression:?}").as_bytes()).unwrap();
    transporter::Wasip1Transporter::write_to_stderr(b", timeout_ns=").unwrap();
    transporter::Wasip1Transporter::write_to_stderr(timeout_ns.to_string().as_bytes()).unwrap();
    transporter::Wasip1Transporter::write_to_stderr(b"\n").unwrap();
}
