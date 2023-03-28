use core::fmt;

/// https://stackoverflow.com/a/64726826
pub struct ByteWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> ByteWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        ByteWriter { buf, cursor: 0 }
    }

    // pub fn as_str(&self) -> &str {
    //     str::from_utf8(&self.buf[0..self.cursor]).unwrap()
    // }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
    }

    pub fn len(&self) -> usize {
        self.cursor
    }

    pub fn empty(&self) -> bool {
        self.cursor == 0
    }

    pub fn full(&self) -> bool {
        self.capacity() == self.cursor
    }
}

impl fmt::Write for ByteWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let capacity = self.capacity();
        if capacity < self.cursor + s.as_bytes().len() {
            // Not enough space in the buffer
            return Err(fmt::Error);
        }

        self.buf[self.cursor..][..s.as_bytes().len()].copy_from_slice(s.as_bytes());

        self.cursor += s.as_bytes().len();
        Ok(())
    }
}
