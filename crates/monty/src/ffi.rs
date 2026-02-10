use std::slice;
use std::ptr;
use crate::{MontyRun, ResourceLimits, LimitedTracker};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn monty_version() -> *const u8 {
    "Monty Native v1.1.0\0".as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn monty_execute(
    code_ptr: *const u8,
    code_len: usize,
    gas_limit: u64,
    memory_limit: usize,
    result_capacity: usize,
    result_buffer: *mut u8,
) -> i32 {
    if code_ptr.is_null() || result_buffer.is_null() {
        return -1;
    }

    let code_span = unsafe { slice::from_raw_parts(code_ptr, code_len) };
    let code_str = match std::str::from_utf8(code_span) {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let limits = ResourceLimits {
        max_instructions: if gas_limit > 0 { Some(gas_limit) } else { None },
        max_memory: if memory_limit > 0 { Some(memory_limit) } else { None },
        ..ResourceLimits::new()
    };

    let runner = match MontyRun::new(code_str.to_string(), "ffi.py", vec![], vec![]) {
        Ok(r) => r,
        Err(e) => {
            copy_to_buffer(&format!("Parse Error:\n{}", e), result_buffer, result_capacity);
            return 2;
        }
    };

    let tracker = LimitedTracker::new(limits);
    let mut print = crate::io::CollectStringPrint::new();
    
    match runner.run(vec![], tracker, &mut print) {
        Ok(obj) => {
            let mut out = print.output().to_string();
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("{:?}", obj));
            copy_to_buffer(&out, result_buffer, result_capacity);
            0
        }
        Err(e) => {
            // Include a prefix so we know exactly which code produced this
            let err_msg = format!("[Native v1.1.0] Runtime Error:\n{}", e);
            copy_to_buffer(&err_msg, result_buffer, result_capacity);
            1
        }
    }
}

fn copy_to_buffer(s: &str, buffer: *mut u8, capacity: usize) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(capacity - 1);
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, len);
        *buffer.add(len) = 0;
    }
}
