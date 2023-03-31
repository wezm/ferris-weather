#![no_std]

extern crate alloc;

mod toolbox;
mod weather;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::ffi::CString;
use alloc::vec::Vec;
use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::fmt::Write;
use core::panic::PanicInfo;
use core::str;

use classic_common::Str255;
use embedded_nal::{nb, IpAddr, Ipv4Addr, SocketAddr, TcpClientStack};
use http_io::client::HttpRequestBuilder;
use http_io::url::Url;
use http_io::Read;

use crate::toolbox::consts::{EIOErr, EMSGSIZEErr, OTBadAddressErr};
use crate::toolbox::{NoteAlert_, OSStatus, OpenTransport, ParamText_, SInt16, Socket, StopAlert_};

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

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    let mut msg = Str255::new();
    match panic_info.payload().downcast_ref::<&str>() {
        Some(s) if s.len() < 256 => {
            write!(msg, "Rust code panicked: {s:?}");
        }
        Some(_) => {
            write!(msg, "Rust code panicked: (message too big)");
        }
        None => {
            write!(msg, "Rust code panicked");
        }
    }

    present_error_message(&msg);
    unsafe { exit(1) };
}

const ALRT_ID: SInt16 = 128;
const NOTE_ALRT_ID: SInt16 = 129;

extern "C" {
    pub fn printf(format: *const c_char, ...) -> c_int;

    fn exit(status: c_int) -> !;
}

fn present_error(err: OSStatus) {
    let mut s = Str255::new();
    if write!(s, "{}", err).is_err() {
        panic!("unable to format error code"); // FIXME: Don't panic
    }

    let empty = Str255::new();

    unsafe {
        ParamText_(s.as_ptr(), empty.as_ptr(), empty.as_ptr(), empty.as_ptr());
        StopAlert_(ALRT_ID);
    }
}

fn present_error_message(msg: &Str255) {
    let empty = Str255::new();

    unsafe {
        ParamText_(msg.as_ptr(), empty.as_ptr(), empty.as_ptr(), empty.as_ptr());
        StopAlert_(ALRT_ID);
    }
}

fn present_weather(location: &str, temp: f32) {
    let mut s = Str255::new();
    if write!(s, "The temperature in {} is {}°C", location, temp).is_err() {
        panic!("unable to format error code");
    }

    let empty = Str255::new();

    unsafe {
        ParamText_(s.as_ptr(), empty.as_ptr(), empty.as_ptr(), empty.as_ptr());
        NoteAlert_(NOTE_ALRT_ID);
    }
}

fn present_note(msg: &Str255) {
    let empty = Str255::new();

    unsafe {
        ParamText_(msg.as_ptr(), empty.as_ptr(), empty.as_ptr(), empty.as_ptr());
        StopAlert_(NOTE_ALRT_ID);
    }
}

#[no_mangle]
pub extern "C" fn do_request() {
    match try_do_request() {
        Ok(()) => (),
        Err(err) => {
            let mut s = Str255::new();
            if write!(s, "try_do_request failed: {}", err).is_err() {
                panic!("unable to format error code");
            }

            present_error_message(&s)
        }
    }
}

fn try_do_request() -> Result<(), OSStatus> {
    let url: Url = "http://www.7bit.org/weather.json"
        .parse()
        .map_err(|_| OTBadAddressErr)?;

    let mut ot = OpenTransport::init()?;

    let mut socket = ot.socket()?;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(45, 76, 112, 252)), 80);

    loop {
        match ot.connect(&mut socket, addr) {
            Ok(()) => break,
            Err(nb::Error::WouldBlock) => continue,
            Err(nb::Error::Other(status)) => return Err(status),
        }
    }
    // unsafe {
    //     printf(b"connected\n\0".as_ptr() as *const c_char);
    // }

    let socket = OTSocket {
        ot: &mut ot,
        socket: Some(socket),
    };

    let mut response = HttpRequestBuilder::get(url)
        .map_err(|_| OTBadAddressErr)?
        .send(socket)
        .map_err(|_| OTBadAddressErr)?
        .finish()
        .map_err(|_| OTBadAddressErr)?;
    let content_len = match response.body.content_length() {
        Some(len) => len as i32,
        None => -1,
    };
    // unsafe {
    //     printf(
    //         b"got response of len %d\n\0".as_ptr() as *const c_char,
    //         content_len,
    //     );
    // }

    let mut sink = match response.body.content_length() {
        Some(len) if len < 500 * 1024 => Vec::with_capacity(len as usize),
        Some(_len) => return Err(EMSGSIZEErr), // Response too big
        None => Vec::new(),
    };

    let mut buf = [0u8; 4095];
    loop {
        match response.body.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                sink.extend_from_slice(&buf[..n]);
                // unsafe {
                //     printf(b"read %u bytes\n\0".as_ptr() as *const c_char, n);
                // }
            }
            Err(http_io::error::Error::OSStatus(status)) => return Err(status),
            Err(_other) => return Err(EIOErr), // TODO: try to map http_io errors to statuses? or create a better error type fro this function
        }
    }
    // let mut note = Str255::new();
    // let _ = write!(note, "read response body, got {} bytes", sink.len());
    // present_note(&note);

    // let OTSocket { socket, .. } = socket;

    match weather::parse(&sink) {
        Ok(Some(observation)) => present_weather(&observation.name, observation.air_temp),
        Ok(None) => {
            let mut s = Str255::new();
            if write!(s, "no observations available").is_err() {
                panic!("unable to format error code");
            }

            present_error_message(&s)
        }
        Err(err) => {
            let mut s = Str255::new();
            if write!(s, "unable to parse JSON: {}", err).is_err() {
                panic!("unable to format error code");
            }

            present_error_message(&s);
            let limit = 1024.min(sink.len());
            let res_text = CString::new(&sink[..limit]).unwrap();
            // unsafe {
            //     printf(b"%s\n\0".as_ptr() as *const c_char, res_text.as_ptr());
            // }
        }
    }
    // unsafe {
    //     printf(b"done\n\0".as_ptr() as *const c_char);
    // }

    Ok(())
}

struct OTSocket<'a> {
    ot: &'a mut OpenTransport,
    socket: Option<Socket>,
}

impl Drop for OTSocket<'_> {
    fn drop(&mut self) {
        let _ = self.ot.close(self.socket.take().unwrap());
    }
}

impl Read for OTSocket<'_> {
    fn read(&mut self, buf: &mut [u8]) -> http_io::error::Result<usize> {
        loop {
            match self.ot.receive(self.socket.as_mut().unwrap(), buf) {
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
            match self.ot.send(&mut self.socket.as_mut().unwrap(), buf) {
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
