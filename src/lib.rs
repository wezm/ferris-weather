#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;

static MSG: &[u8] = b"\x04Rust";

#[no_mangle]
pub unsafe extern "C" fn hello_rust() -> *const u8 {
    MSG.as_ptr()
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
