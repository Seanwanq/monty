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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn monty_type_check(
    code_ptr: *const u8,
    code_len: usize,
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

    let source_file = monty_type_checking::SourceFile::new(code_str, "ffi.py");

    match monty_type_checking::type_check(&source_file, None) {
        Ok(Some(diagnostics)) => {
            // Found errors
            // Use Debug impl which formats via DisplayDiagnostics
            let output = format!("{:?}", diagnostics);
            copy_to_buffer(&output, result_buffer, result_capacity);
            1 // Ty errors found
        }
        Ok(None) => {
            // No errors
            copy_to_buffer("No issues found.", result_buffer, result_capacity);
            0 // Success
        }
        Err(e) => {
            // Internal error
            let err_msg = format!("Type Check Internal Error: {}", e);
            copy_to_buffer(&err_msg, result_buffer, result_capacity);
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn monty_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn monty_free(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, 0, len);
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
