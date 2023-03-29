#![no_std]
#![feature(lang_items)]

extern crate alloc;

mod byte_writer;

use core::ffi::{c_uchar, c_void};
use core::fmt::Write;
use core::panic::PanicInfo;
use core::slice;
use core::str;

use crate::byte_writer::ByteWriter;

use alloc::alloc::{GlobalAlloc, Layout};

struct Malloc;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe impl GlobalAlloc for Malloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void)
    }
}

#[global_allocator]
static GLOBAL: Malloc = Malloc;
static MSG: &[u8] = b"\x04Rust";

#[no_mangle]
pub unsafe extern "C" fn hello_rust() -> *const u8 {
    MSG.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn ConvertCtoF(celcius_str: *const c_uchar, faren_str: *mut c_uchar) {
    do_convert(celcius_str, faren_str, |c| c * 1.8 + 32.0)
}

#[no_mangle]
pub unsafe extern "C" fn ConvertFtoC(faren_str: *const c_uchar, celcius_str: *mut c_uchar) {
    do_convert(faren_str, celcius_str, |f| (f - 32.0) / 1.8)
}

fn do_convert(in_str: *const c_uchar, out_str: *mut c_uchar, convert: impl Fn(f32) -> f32) {
    let in_val = match ascii_pascal_to_str(in_str) {
        Some(s) => parse_temp(s),
        // If the text is invalid then don't bother updating celcius_str
        None => return,
    };

    // Perform the conversion
    let out_val = convert(in_val);

    // Stringify `out_val` and write the result back to out_str
    // NOTE(unsafe): It it assumed that out_str is a Str255, which has room for 255 characters
    let buf = unsafe { slice::from_raw_parts_mut(out_str, 255) };
    let mut writer = ByteWriter::new(&mut buf[1..]);
    if write!(writer, "{:.2}", out_val).is_err() {
        return;
    }

    // Set the length
    buf[0] = writer.len() as c_uchar; // FIXME: Enforce by construction of ByteWriter
}

/// Converts a Pascal string containing only 7-bit ASCII characters to a Rust str
///
/// Returns None if the Pascal string contains non-ASCII characters
fn ascii_pascal_to_str<'a>(bytes: *const c_uchar) -> Option<&'a str> {
    let len = unsafe { *bytes };
    if len == 0 {
        return Some("");
    }

    // NOTE(unsafe): offset _should_ be safe as len is non-zero as checked above
    let bytes = unsafe { slice::from_raw_parts(bytes.offset(1), usize::from(len)) };

    // Check that all chars are ASCII, if so then create a string slice from the bytes
    // NOTE(unsafe): from_utf8_unchecked is safe as we checked all bytes are ASCII, which
    // means they're valid UTF-8 too.
    bytes
        .iter()
        .all(|byte| byte.is_ascii())
        .then(|| unsafe { str::from_utf8_unchecked(bytes) })
}

/// Parse a temperature string, return 0 if it fails to parse
fn parse_temp(s: &str) -> f32 {
    s.parse().unwrap_or(0.)
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn eh_personality() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_is_pascal_string() {
        assert_eq!(MSG[0], MSG[1..].len().try_into().unwrap());
    }
}
