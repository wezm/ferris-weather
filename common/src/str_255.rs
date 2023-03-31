use alloc::borrow::Cow;
use core::fmt;
use core::str;

use crate::macroman;

/// A Pascal string that can hold up to 255 characters
pub struct Str255 {
    buf: [u8; 256],
}

// TODO: implement FromStr

impl Str255 {
    pub fn new() -> Self {
        Str255 { buf: [0; 256] }
    }

    pub fn as_str(&self) -> Cow<'_, str> {
        macroman::to_str(self.buf())
    }

    // #[inline]
    // pub fn capacity(&self) -> usize {
    //     self.buf.len()
    // }

    pub fn remaining(&self) -> usize {
        255 - self.len()
    }

    // pub fn clear(&mut self) {
    //     self.cursor = 0;
    // }

    pub fn len(&self) -> usize {
        usize::from(self.buf[0])
    }

    // pub fn empty(&self) -> bool {
    //     self.cursor == 0
    // }
    //
    // pub fn full(&self) -> bool {
    //     self.capacity() == self.cursor
    // }

    fn buf(&self) -> &[u8] {
        &self.buf[1..][..self.len()]
    }

    // fn remaining_buf(&self) -> &[u8] {
    //     &self.buf[1..][..self.remaining()]
    // }

    // pub fn buf_mut(&mut self) -> &mut [u8] {
    //     &mut self.buf[0..self.cursor]
    // }

    /// Return a pointer to the data (including leading length byte)
    pub fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }
}

impl fmt::Write for Str255 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let offset = 1 + self.len();
        self.buf[offset..]
            .get_mut(..s.as_bytes().len())
            .ok_or(fmt::Error)
            .map(|buf| buf.copy_from_slice(s.as_bytes()))?;
        self.buf[0] += s.as_bytes().len() as u8;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write;

    #[test]
    fn str_255() {
        let mut s = Str255::new();
        write!(s, "0123456789").unwrap();
        assert_eq!(s.len(), 10);
        assert_eq!(s.remaining(), 245);

        // Fill it up to 250 chars
        for _ in 0..24 {
            write!(s, "0123456789").unwrap();
        }
        assert_eq!(s.len(), 250);
        assert_eq!(s.remaining(), 5);

        // Test overflow
        assert!(write!(s, "0123456789").is_err());

        // Test fill to capacity
        assert!(write!(s, "01234").is_ok());
        assert_eq!(s.len(), 255);
        assert_eq!(s.remaining(), 0);

        // Test overflow
        assert!(write!(s, "1").is_err());
    }

    #[test]
    fn test_formatting() {
        let mut s = Str255::new();
        write!(s, "Formatting: 0x{:08X}", 0xFEDCBA).unwrap();
        assert_eq!(s.as_str(), "Formatting: 0x00FEDCBA");
    }
}
