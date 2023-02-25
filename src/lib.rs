static MSG: &[u8] = b"\x04Rust";

pub fn hello_rust() -> *const u8 {
    MSG.as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_is_pacal_string() {
        assert_eq!(MSG[0], MSG[1..].len().try_into().unwrap());
    }
}
