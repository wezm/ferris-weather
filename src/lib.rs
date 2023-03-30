#![no_std]

extern crate alloc;

mod byte_writer;
mod mastodon;
mod toolbox;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::vec::Vec;
use core::ffi::{c_uchar, c_void};
use core::fmt::Write;
use core::slice;
use core::str;

use embedded_nal::{nb, IpAddr, Ipv4Addr, SocketAddr, TcpClientStack};
use http_io::client::HttpRequestBuilder;
use http_io::url::Url;
use http_io::Read;

use panic_abort as _;

use crate::byte_writer::ByteWriter;
use crate::toolbox::consts::{EIOErr, EMSGSIZEErr, OTBadAddressErr};
use crate::toolbox::{OSStatus, OpenTransport, ParamText_, SInt16, Socket, StopAlert_};

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

const ALRT_ID: SInt16 = 128;

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

#[no_mangle]
pub extern "C" fn present_error(err: OSStatus) {
    let mut buf = [0; 256];
    let mut writer = ByteWriter::new(&mut buf[1..]);
    if write!(writer, "{}", err).is_err() {
        panic!("unable to format error code");
    }

    // Set the length
    buf[0] = writer.len() as c_uchar; // FIXME: Enforce by construction of ByteWriter

    let empty = [0; 256];

    unsafe {
        ParamText_(buf.as_ptr(), empty.as_ptr(), empty.as_ptr(), empty.as_ptr());
        StopAlert_(ALRT_ID);
    }
}

#[no_mangle]
pub extern "C" fn do_request() {
    match try_do_request() {
        Ok(()) => (),
        Err(err) => present_error(err),
    }
}

fn try_do_request() -> Result<(), OSStatus> {
    let url: Url = "http://jackkelly.name/"
        .parse()
        .map_err(|_| OTBadAddressErr)?;

    let mut ot = OpenTransport::init()?;

    let mut socket = ot.socket()?;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(139, 180, 175, 151)), 80);

    loop {
        match ot.connect(&mut socket, addr) {
            Ok(()) => break,
            Err(nb::Error::WouldBlock) => continue,
            Err(nb::Error::Other(status)) => return Err(status),
        }
    }

    let socket = OTSocket {
        ot: &mut ot,
        socket,
    };

    let mut response = HttpRequestBuilder::get(url)
        .map_err(|_| OTBadAddressErr)?
        .send(socket)
        .map_err(|_| OTBadAddressErr)?
        .finish()
        .map_err(|_| OTBadAddressErr)?;

    let mut sink = match response.body.content_length() {
        Some(len) if len < 500 * 1024 => Vec::with_capacity(len as usize),
        Some(_len) => return Err(EMSGSIZEErr), // Response too big
        None => Vec::new(),
    };

    let mut buf = [0u8; 4095];
    loop {
        match response.body.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => sink.extend_from_slice(&buf[..n]),
            Err(http_io::error::Error::OSStatus(status)) => return Err(status),
            Err(_other) => return Err(EIOErr), // TODO: try to map http_io errors to statuses? or create a better error type fro this function
        }
    }

    let mut buf = [0; 256];
    let mut writer = ByteWriter::new(&mut buf[1..]);
    if write!(writer, "Read {} bytes", sink.len()).is_err() {
        panic!("unable to format message");
    }

    // Set the length
    buf[0] = writer.len() as c_uchar; // FIXME: Enforce by construction of ByteWriter

    let empty = [0; 256];

    unsafe {
        ParamText_(buf.as_ptr(), empty.as_ptr(), empty.as_ptr(), empty.as_ptr());
        StopAlert_(ALRT_ID);
    }

    Ok(())
}

struct OTSocket<'a> {
    ot: &'a mut OpenTransport,
    socket: Socket,
}

impl http_io::Read for OTSocket<'_> {
    fn read(&mut self, buf: &mut [u8]) -> http_io::error::Result<usize> {
        loop {
            match self.ot.receive(&mut self.socket, buf) {
                Ok(nbytes) => return Ok(nbytes),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(status)) => {
                    return Err(http_io::error::Error::OSStatus(status))
                }
            }

            // TODO: YieldToOtherThreads
        }
    }
}

impl http_io::Write for OTSocket<'_> {
    fn write(&mut self, buf: &[u8]) -> http_io::error::Result<usize> {
        loop {
            match self.ot.send(&mut self.socket, buf) {
                Ok(nbytes) => return Ok(nbytes),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(status)) => {
                    return Err(http_io::error::Error::OSStatus(status))
                }
            }

            // TODO: YieldToOtherThreads
        }
    }

    fn flush(&mut self) -> http_io::error::Result<()> {
        // It's possible this can be done with an ioctl
        // OTIoctl(ep, I_FLUSH, (void*) FLUSHRW);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_is_pascal_string() {
        assert_eq!(MSG[0], MSG[1..].len().try_into().unwrap());
    }
}
