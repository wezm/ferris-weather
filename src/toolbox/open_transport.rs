//! OpenTransport stuff

// Can we implement embedded-nal for OT?

use core::ffi::{c_char, c_uint, c_void};
use core::mem::MaybeUninit;
use core::{mem, ptr};

use embedded_nal::{nb, IpAddr, SocketAddr, TcpClientStack};

use crate::toolbox::{Boolean, ByteCount, OSStatus, SInt32, UInt16, UInt32, UInt8, NO_ERR, TRUE};

const BUFFER_SIZE: usize = 4096;

// const kOTInvalidEndpointRef:

// struct ProviderRef(*mut c_void);
// struct EndpointRef(*mut c_void);
// struct MapperRef(*mut c_void);

type ProviderRef = *mut c_void;
type EndpointRef = *mut c_void;
// type MapperRef = *mut c_void;

pub struct OpenTransport {}

pub struct Socket {
    // buffer: [u8; BUFFER_SIZE],
    endpoint: EndpointRef,
    connected: bool,
    bound: bool,
}

#[repr(C)]
struct OTConfiguration {
    _private: [u8; 0],
}
type OTConfigurationRef = *mut OTConfiguration;

type OTOpenFlags = UInt32;

type OTDataSize = SInt32;

type OTServiceType = UInt32;

/* ***** Miscellaneous Type Definitions ******/

/* A millisecond timeout value*/
type OTTimeout = UInt32;
/* An ID number in connections/transactions     */
type OTSequence = SInt32;
/* An ID number for registered names            */
type OTNameID = SInt32;
/*
   A protocol-specific reason code for failure.
   Usually a Unix-style positive error code.
*/
type OTReason = SInt32;
/* Number of outstanding connection requests at a time.*/
type OTQLen = UInt32;
/* Will become internationalizeable shortly (yeah, right).*/
type OTClientName = *mut UInt8;
/* The command code in STREAMS messages.*/
type OTCommand = SInt32;
/* value describing a client*/
// type struct OpaqueOTClient*          OTClient;

type OTAddressType = UInt16;

type OTResult = SInt32;

type OTByteCount = c_uint;

type OTFlags = UInt32;

type OTEventCode = UInt32;

#[repr(C)]
struct TBind {
    addr: TNetbuf,
    qlen: OTQLen,
}

#[repr(C)]
struct TNetbuf {
    maxlen: ByteCount,
    len: ByteCount,
    buf: *mut UInt8,
}

#[repr(C)]
struct TDiscon {
    udata: TNetbuf,
    reason: OTReason,
    sequence: OTSequence,
}

#[allow(unused, non_upper_case_globals)]
mod consts {
    use core::ffi::CStr;

    use super::{OTDataSize, OTEventCode, OTFlags, OTServiceType, UInt32};

    // OpenTransportProviders.h
    pub const TCPName: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"tcp\0") };
    pub const UDPName: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"udp\0") };

    pub const T_COTS: OTServiceType = 1; /* Connection-mode service                    */
    pub const T_COTS_ORD: OTServiceType = 2; /* Connection service with orderly release          */
    pub const T_CLTS: OTServiceType = 3; /* Connectionless-mode service                   */
    pub const T_TRANS: OTServiceType = 5; /* Connection-mode transaction service              */
    pub const T_TRANS_ORD: OTServiceType = 6; /* Connection transaction service with orderly release    */
    pub const T_TRANS_CLTS: OTServiceType = 7; /* Connectionless transaction service           */

    /* Masks for the flags field of TEndpointInfo.*/

    pub const T_SENDZERO: UInt32 = 0x0001; /* supports 0-length TSDU's          */
    pub const T_XPG4_1: UInt32 = 0x0002; /* supports the GetProtAddress call     */
    pub const T_CAN_SUPPORT_MDATA: UInt32 = 0x10000000; /* support M_DATAs on packet protocols    */
    pub const T_CAN_RESOLVE_ADDR: UInt32 = 0x40000000; /* Supports ResolveAddress call      */
    pub const T_CAN_SUPPLY_MIB: UInt32 = 0x20000000; /* Supports SNMP MIB data          */

    /*
       Special-case values for in the tsdu, etsdu, connect, and discon
       fields of TEndpointInfo.
    */

    pub const T_INFINITE: OTDataSize = -1; /* supports infinit amounts of data     */
    pub const T_INVALID: OTDataSize = -2; /* Does not support data transmission */

    /* ***** OTFlags ******/
    /*
       This type is used to describe bitwise flags in OT data structures
       and parameters.  Think of it as the OT analogue to the OptionBits
       type in "MacTypes.h".
    */

    /*
       These flags are used when sending and receiving data.  The
       constants defined are masks.
    */
    const T_MORE: OTFlags = 0x0001; /* More data to come in message     */
    const T_EXPEDITED: OTFlags = 0x0002; /* Data is expedited, if possible */
    const T_ACKNOWLEDGED: OTFlags = 0x0004; /* Acknowledge transaction         */
    const T_PARTIALDATA: OTFlags = 0x0008; /* Partial data - more coming     */
    const T_NORECEIPT: OTFlags = 0x0010; /* No event on transaction done     */
    const T_TIMEDOUT: OTFlags = 0x0020; /* Reply timed out              */

    /* These flags are used in the TOptMgmt structure to request services.*/

    const T_NEGOTIATE: OTFlags = 0x0004;
    const T_CHECK: OTFlags = 0x0008;
    const T_DEFAULT: OTFlags = 0x0010;
    const T_CURRENT: OTFlags = 0x0080;

    /*
       These flags are used in the TOptMgmt and TOption structures to
       return results.
    */

    const T_SUCCESS: OTFlags = 0x0020;
    const T_FAILURE: OTFlags = 0x0040;
    const T_PARTSUCCESS: OTFlags = 0x0100;
    const T_READONLY: OTFlags = 0x0200;
    const T_NOTSUPPORT: OTFlags = 0x0400;

    /* ***** Event Codes ******/
    /*
       OT event codes values for Open Transport.  These are the event codes that
       are sent to notification routine (notifiers).
    */

    /*
       Events are divided into numerous categories:

       1. (0x0000xxxx) The core XTI events have identifiers of the form
          T_XXXX.  These signal that an XTI event has occured on a stream.
       2. (0x1000xxxx) Private events are reserved for protocol specific
          events.  Each protocol stack defines them as appropriate for
          its own usage.
       3. (0x2000xxxxx) Completion events have identifiers of the form
          T_XXXXCOMPLETE.  These signal the completion of some asynchronous
          API routine, and are only delivered if the endpoint is in asynchronous
          mode.
       4. (0x2100xxxx) Stream events are generally encountered when programming
          the raw streams API and indicate some event on a raw stream, or
          some other event of interest in the STREAMS kernel.
       5. (0x2200xxxx) Signal events indicate that a signal has arrived on
          a raw stream.  See "Signal Values" for details.
       6. (0x2300xxxx) General provider events that might be generated by any
          provider.
       7. (0x2400xxxx) System events sent to all providers.
       8. (0x2500xxxx) System events sent to registered clients.
       9. (0x2600xxxx) System events used by configurators.
      10. (0x2700xxxx) Events sent to registered OT clients.
    */
    /*
       All event codes not described here are reserved by Apple.  If you receive
       an event code you do not understand, ignore it!
    */

    pub const T_LISTEN: OTEventCode = 0x0001; /* An connection request is available     */
    pub const T_CONNECT: OTEventCode = 0x0002; /* Confirmation of a connect request  */
    pub const T_DATA: OTEventCode = 0x0004; /* Standard data is available        */
    pub const T_EXDATA: OTEventCode = 0x0008; /* Expedited data is available         */
    pub const T_DISCONNECT: OTEventCode = 0x0010; /* A disconnect is available       */
    pub const T_ERROR: OTEventCode = 0x0020; /* obsolete/unused in library        */
    pub const T_UDERR: OTEventCode = 0x0040; /* A Unit Data Error has occurred     */
    pub const T_ORDREL: OTEventCode = 0x0080; /* An orderly release is available       */
    pub const T_GODATA: OTEventCode = 0x0100; /* Flow control lifted on standard data   */
    pub const T_GOEXDATA: OTEventCode = 0x0200; /* Flow control lifted on expedited data*/
    pub const T_REQUEST: OTEventCode = 0x0400; /* An Incoming request is available     */
    pub const T_REPLY: OTEventCode = 0x0800; /* An Incoming reply is available     */
    pub const T_PASSCON: OTEventCode = 0x1000; /* State is now T_DATAXFER          */
    pub const T_RESET: OTEventCode = 0x2000; /* Protocol has been reset          */
    pub const PRIVATEEVENT: OTEventCode = 0x10000000; /* Base of the private event range.*/
    pub const COMPLETEEVENT: OTEventCode = 0x20000000; /* Base of the completion event range.*/
    pub const T_BINDCOMPLETE: OTEventCode = 0x20000001; /* Bind call is complete          */
    pub const T_UNBINDCOMPLETE: OTEventCode = 0x20000002; /* Unbind call is complete          */
    pub const T_ACCEPTCOMPLETE: OTEventCode = 0x20000003; /* Accept call is complete          */
    pub const T_REPLYCOMPLETE: OTEventCode = 0x20000004; /* SendReply call is complete        */
    pub const T_DISCONNECTCOMPLETE: OTEventCode = 0x20000005; /* Disconnect call is complete         */
    pub const T_OPTMGMTCOMPLETE: OTEventCode = 0x20000006; /* OptMgmt call is complete          */
    pub const T_OPENCOMPLETE: OTEventCode = 0x20000007; /* An Open call is complete          */
    pub const T_GETPROTADDRCOMPLETE: OTEventCode = 0x20000008; /* GetProtAddress call is complete       */
    pub const T_RESOLVEADDRCOMPLETE: OTEventCode = 0x20000009; /* A ResolveAddress call is complet     */
    pub const T_GETINFOCOMPLETE: OTEventCode = 0x2000000A; /* A GetInfo call is complete        */
    pub const T_SYNCCOMPLETE: OTEventCode = 0x2000000B; /* A Sync call is complete          */
    pub const T_MEMORYRELEASED: OTEventCode = 0x2000000C; /* No-copy memory was released         */
    pub const T_REGNAMECOMPLETE: OTEventCode = 0x2000000D; /* A RegisterName call is complete       */
    pub const T_DELNAMECOMPLETE: OTEventCode = 0x2000000E; /* A DeleteName call is complete   */
    pub const T_LKUPNAMECOMPLETE: OTEventCode = 0x2000000F; /* A LookupName call is complete   */
    pub const T_LKUPNAMERESULT: OTEventCode = 0x20000010; /* A LookupName is returning a name     */
    pub const OTSyncIdleEvent: OTEventCode = 0x20000011; /* Synchronous call Idle event         */
    pub const STREAMEVENT: OTEventCode = 0x21000000; /* Base of the raw stream event range.*/
    pub const OTReservedEvent1: OTEventCode = 0x21000001; /* reserved for internal use by OT       */
    pub const GetmsgEvent: OTEventCode = 0x21000002; /* A GetMessage call is complete   */
    pub const StreamReadEvent: OTEventCode = 0x21000003; /* A Read call is complete          */
    pub const StreamWriteEvent: OTEventCode = 0x21000004; /* A Write call is complete          */
    pub const StreamIoctlEvent: OTEventCode = 0x21000005; /* An Ioctl call is complete       */
    pub const OTReservedEvent2: OTEventCode = 0x21000006; /* reserved for internal use by OT       */
    pub const StreamOpenEvent: OTEventCode = 0x21000007; /* An OpenStream call is complete     */
    pub const PollEvent: OTEventCode = 0x21000008; /* A Poll call is complete          */
    pub const OTReservedEvent3: OTEventCode = 0x21000009; /* reserved for internal use by OT       */
    pub const OTReservedEvent4: OTEventCode = 0x2100000A; /* reserved for internal use by OT       */
    pub const OTReservedEvent5: OTEventCode = 0x2100000B; /* reserved for internal use by OT       */
    pub const OTReservedEvent6: OTEventCode = 0x2100000C; /* reserved for internal use by OT       */
    pub const OTReservedEvent7: OTEventCode = 0x2100000D; /* reserved for internal use by OT       */
    pub const OTReservedEvent8: OTEventCode = 0x2100000E; /* reserved for internal use by OT       */
    pub const SIGNALEVENT: OTEventCode = 0x22000000; /* A signal has arrived on a raw stream, see "Signal Values" below.*/
    pub const PROTOCOLEVENT: OTEventCode = 0x23000000; /* Some event from the protocols   */
    pub const OTProviderIsDisconnected: OTEventCode = 0x23000001; /* Provider is temporarily off-line     */
    pub const OTProviderIsReconnected: OTEventCode = 0x23000002; /* Provider is now back on-line      */
    pub const OTProviderWillClose: OTEventCode = 0x24000001; /* Provider will close immediately       */
    pub const OTProviderIsClosed: OTEventCode = 0x24000002; /* Provider was closed              */
    pub const OTPortDisabled: OTEventCode = 0x25000001; /* Port is now disabled, result is 0, cookie is port ref */
    pub const OTPortEnabled: OTEventCode = 0x25000002; /* Port is now enabled, result is 0, cookie is port ref */
    pub const OTPortOffline: OTEventCode = 0x25000003; /* Port is now offline, result is 0, cookie is port ref */
    pub const OTPortOnline: OTEventCode = 0x25000004; /* Port is now online, result is 0, cookie is port ref */
    pub const OTClosePortRequest: OTEventCode = 0x25000005; /* Request to close/yield, result is reason, cookie is OTPortCloseStruct* */
    pub const OTYieldPortRequest: OTEventCode = 0x25000005; /* Request to close/yield, result is reason, cookie is OTPortCloseStruct* */
    pub const OTNewPortRegistered: OTEventCode = 0x25000006; /* New port has been registered, cookie is port ref */
    pub const OTPortNetworkChange: OTEventCode = 0x25000007; /* Port may have moved to a new network, result is 0, cookie is port ref */
    pub const OTConfigurationChanged: OTEventCode = 0x26000001; /* Protocol configuration changed     */
    pub const OTSystemSleep: OTEventCode = 0x26000002;
    pub const OTSystemShutdown: OTEventCode = 0x26000003;
    pub const OTSystemAwaken: OTEventCode = 0x26000004;
    pub const OTSystemIdle: OTEventCode = 0x26000005;
    pub const OTSystemSleepPrep: OTEventCode = 0x26000006;
    pub const OTSystemShutdownPrep: OTEventCode = 0x26000007;
    pub const OTSystemAwakenPrep: OTEventCode = 0x26000008;
    pub const OTStackIsLoading: OTEventCode = 0x27000001; /* Sent before Open Transport attempts to load the TCP/IP protocol stack.*/
    pub const OTStackWasLoaded: OTEventCode = 0x27000002; /* Sent after the TCP/IP stack has been successfully loaded.*/
    pub const OTStackIsUnloading: OTEventCode = 0x27000003; /* Sent before Open Transport unloads the TCP/IP stack.*/
}

#[repr(C)]
struct TEndpointInfo {
    addr: OTDataSize,        /* Maximum size of an address        */
    options: OTDataSize,     /* Maximum size of options          */
    tsdu: OTDataSize,        /* Standard data transmit unit size     */
    etsdu: OTDataSize,       /* Expedited data transmit unit size  */
    connect: OTDataSize,     /* Maximum data size on connect      */
    discon: OTDataSize,      /* Maximum data size on disconnect       */
    servtype: OTServiceType, /* service type                */
    flags: UInt32,           /* Flags (see above for values)      */
}

/*
   TCall holds information about a connection and is a parameter to
   OTConnect, OTRcvConnect, OTListen, OTAccept, and OTSndDisconnect.
*/
#[repr(C)]
struct TCall {
    addr: TNetbuf,
    apt: TNetbuf,
    udata: TNetbuf,
    sequence: OTSequence,
}

type OTEndpointRef = *mut c_void;

#[derive(Debug)]
enum OTError {
    OSStatus(OSStatus),
    IPv6Unsupported,
}

/* ***** TCP/IP ******/

type InetPort = UInt16;
type InetHost = UInt32;

#[repr(C)]
struct InetAddress {
    fAddressType: OTAddressType, /* always AF_INET*/
    fPort: InetPort,             /* Port number */
    fHost: InetHost,             /* Host address in net byte order*/
    fUnused: [UInt8; 8],         /* Traditional unused bytes*/
}

extern "C" {
    // OSStatus InitOpenTransport_(void);
    fn InitOpenTransport_() -> OSStatus;

    // EndpointRef OTOpenEndpoint_(
    // OTConfigurationRef cfig,
    // OTOpenFlags oflag,
    // TEndpointInfo *info,        /* can be NULL */
    // OSStatus *err);
    fn OTOpenEndpoint_(
        cfig: OTConfigurationRef,
        oflag: OTOpenFlags,
        info: *mut TEndpointInfo,
        err: *mut OSStatus,
    ) -> OTEndpointRef;

    // OTConfigurationRef OTCreateConfiguration_(const char *path);
    fn OTCreateConfiguration_(path: *const c_char) -> OTConfigurationRef;

    // OSStatus OTSetSynchronous_(ProviderRef ref);
    fn OTSetSynchronous_(provider: ProviderRef) -> OSStatus;

    // OSStatus OTSetBlocking_(ProviderRef ref);
    fn OTSetBlocking_(provider: ProviderRef) -> OSStatus;

    // typedef void (*OTNotifyProcPtrC )(void *contextPtr, OTEventCode code, OTResult result, void *cookie);
    //
    // typedef struct
    // {
    //     OTNotifyProcPtrC proc;
    //     void *contextPtr;
    // } NotifyProcContext;
    //
    // OSStatus OTInstallNotifier_(ProviderRef ref, NotifyProcContext *ctxt);
    fn OTInstallNotifier_(provider: ProviderRef) -> OSStatus;

    // OSStatus OTUseSyncIdleEvents_(ProviderRef ref, Boolean useEvents);
    fn OTUseSyncIdleEvents_(provider: ProviderRef, use_events: Boolean) -> OSStatus;

    // OSStatus OTBind_(
    // EndpointRef   ref,
    // TBind *       reqAddr,       /* can be NULL */
    // TBind *       retAddr);
    fn OTBind_(endpoint: EndpointRef, req_addr: *mut TBind, ret_addr: *mut TBind) -> OSStatus;

    fn OTUnbind_(endpoint: EndpointRef) -> OSStatus;

    fn OTInitInetAddress_(addr: *mut InetAddress, port: InetPort, host: InetHost);

    // fn OTInitDNSAddress(addr: DNSAddress, name: *mut c_char);

    fn OTConnect_(endpoint: EndpointRef, snd_call: *mut TCall, rcv_call: *mut TCall) -> OSStatus;

    fn OTSnd_(
        endpoint: EndpointRef,
        buf: *mut c_void,
        nbytes: OTByteCount,
        flags: OTFlags,
    ) -> OTResult;

    fn OTRcv_(
        endpoint: EndpointRef,
        buf: *mut c_void,
        nbytes: OTByteCount,
        flags: *mut OTFlags,
    ) -> OTResult;

    fn OTLook_(endpoint: EndpointRef) -> OTResult;

    fn OTRcvDisconnect_(endpoint: EndpointRef, discon: *mut TDiscon) -> OSStatus;

    fn OTRcvOrderlyDisconnect_(endpoint: EndpointRef) -> OSStatus;

    fn OTSndOrderlyDisconnect_(endpoint: EndpointRef) -> OSStatus;

    fn OTCloseProvider_(provider: ProviderRef) -> OSStatus;

    fn CloseOpenTransport_();
}

// macro_rules! check {
//     ($e:expr) => {
//         let err = $e;
//         if err != NO_ERR {
//             return Err(err)
//         }
//     };
// }
macro_rules! check {
    ($expr:expr $(,)?) => {
        match $expr {
            NO_ERR => (),
            err => return Err(err),
        }
    };
}

impl OpenTransport {
    // TODO: Make this a singleton or something so this is only called once?
    pub fn init() -> Result<Self, OSStatus> {
        match unsafe { InitOpenTransport_() } {
            NO_ERR => Ok(OpenTransport {}),
            err => Err(err),
        }
    }
}

impl Drop for OpenTransport {
    fn drop(&mut self) {
        unsafe { CloseOpenTransport_() };
    }
}

impl TcpClientStack for OpenTransport {
    type TcpSocket = Socket;
    type Error = OSStatus;

    fn socket(&mut self) -> Result<Self::TcpSocket, Self::Error> {
        let endpoint = unsafe {
            let mut err = NO_ERR;
            let ep = OTOpenEndpoint_(
                OTCreateConfiguration_(consts::TCPName.as_ptr()),
                0,
                ptr::null_mut(),
                &mut err as *mut OSStatus,
            );
            if err != NO_ERR {
                return Err(err);
            }

            // By default endpoints created with OTOpenEndpoint are syncronous, non-blocking
            // TODO: Verify/assert that?

            // OTSetSynchronous(ep);
            // check!(OTSetSynchronous_(ep));
            // OTAssert("MyDownloadHTTPSimple: OTSetSynchronous failed",
            //          junk == noErr);
            // junk = OTSetBlocking(ep);
            // check!(OTSetBlocking_(ep));
            // OTAssert("MyDownloadHTTPSimple: OTSetBlocking failed",
            //          junk == noErr);
            // junk = OTInstallNotifier(ep, YieldingNotifier, nil);
            // check!(OTInstallNotifier_(ep));
            // OTAssert("MyDownloadHTTPSimple: OTInstallNotifier failed",
            //          junk == noErr);
            // junk = OTUseSyncIdleEvents(ep, true);
            // check!(OTUseSyncIdleEvents_(ep, TRUE));
            // OTAssert("MyDownloadHTTPSimple: OTUseSyncIdleEvents failed",
            //          junk == noErr);
            //
            // /* Bind the endpoint. */
            // err = OTBind(ep, nil, nil);
            check!(OTBind_(ep, ptr::null_mut(), ptr::null_mut()));
            // bound = (err == noErr);
            ep
        };
        Ok(Socket {
            endpoint,
            bound: true,
            connected: false,
        })
    }

    fn connect(
        &mut self,
        socket: &mut Self::TcpSocket,
        remote: SocketAddr,
    ) -> embedded_nal::nb::Result<(), Self::Error> {
        // OTMemzero(&snd_call, sizeof(TCall));
        // let mut addr = unsafe { mem::zeroed::<InetAddress>() };
        let mut addr = MaybeUninit::<InetAddress>::uninit();

        let ip = match remote.ip() {
            // FIXME: This might work since Macs are big-endian but I'm not sure it's right
            IpAddr::V4(ip) => InetHost::from_be_bytes(ip.octets()),
            IpAddr::V6(_) => todo!("return IPv6 error"),
        };

        let mut addr = unsafe {
            OTInitInetAddress_(addr.as_mut_ptr(), remote.port(), ip);
            addr.assume_init()
        };

        let mut snd_call = unsafe { mem::zeroed::<TCall>() };
        snd_call.addr.buf = &mut addr as *mut InetAddress as *mut u8; // (UInt8 *) &hostDNSAddress;
        snd_call.addr.len = mem::size_of::<InetAddress>() as ByteCount;
        // snd_call.addr.len = OTInitDNSAddress(&hostDNSAddress, (char *)
        //                                     hostName);
        // err = OTConnect(ep, &snd_call, nil);
        let err = unsafe {
            OTConnect_(
                socket.endpoint,
                &mut snd_call as *mut TCall,
                ptr::null_mut(),
            )
        };
        if err != NO_ERR {
            return Err(nb::Error::Other(err));
        }
        socket.connected = true;
        Ok(())
    }

    fn is_connected(&mut self, socket: &Self::TcpSocket) -> Result<bool, Self::Error> {
        // TODO: do we need to do more here to track the state?
        // Perhaps call OTEndPointInfo?
        Ok(socket.connected)
    }

    fn send(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &[u8],
    ) -> embedded_nal::nb::Result<usize, Self::Error> {
        // TODO: Handle this
        // The maximum size of the data you can send is specified by the tsdu field of the
        // TEndpointInfo structure

        // If you want to break up the data into logical units, you can set the T_MORE bit of
        // the flags parameter to indicate that you are using additional calls to the OTSnd
        // function to send more data that belongs to the same logical unit.

        // If the endpoint is in non-blocking or asynchronous mode, it is also possible that
        // only part of the data is actually accepted by the transport provider. In this case,
        // the OTSnd function returns a value that is less than the value of the nbytes
        // parameter. In this case, you should call the function again to send the
        // remaining data.

        // NOTE(safety): We're casting a const pointer to a mut pointer here because the argument
        // to OTSnd is not const... hopefully OTSnd does not attempt to modify it.
        let buf = buffer.as_ptr() as *mut c_void;
        let res = unsafe { OTSnd_(socket.endpoint, buf, buffer.len() as OTByteCount, 0) };

        if res >= 0 {
            Ok(res as usize)
        } else {
            Err(nb::Error::Other(res))
        }
    }

    fn receive(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &mut [u8],
    ) -> embedded_nal::nb::Result<usize, Self::Error> {
        // TODO: check that socket is connected

        let mut flags: OTFlags = 0;
        let res = unsafe {
            OTRcv_(
                socket.endpoint,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as OTByteCount,
                &mut flags as *mut OTFlags,
            )
        };

        match res {
            0 => todo!("how to handle this? Treat as WouldBlock or as success: done reading?"),
            nread if res > 0 => Ok(nread as usize),
            super::consts::EAGAINErr
            | super::consts::EWOULDBLOCKErr
            | super::consts::OTNoDataErr => Err(nb::Error::WouldBlock),
            super::consts::OTLookErr => {
                let look = unsafe { OTLook_(socket.endpoint) };
                if look < 0 {
                    return Err(nb::Error::Other(look));
                }

                // NOTE(cast): Should be safe as negative values checked above
                match look as OTEventCode {
                    consts::T_DISCONNECT => {
                        // TODO: For TCP it seems like OTRcvDisconnect can't really fail

                        unsafe { OTRcvDisconnect_(socket.endpoint, ptr::null_mut()) };
                        socket.connected = false;
                        Ok(0)
                    }
                    consts::T_ORDREL => {
                        unsafe { OTRcvOrderlyDisconnect_(socket.endpoint) };
                        unsafe { OTSndOrderlyDisconnect_(socket.endpoint) };
                        socket.connected = false;
                        Ok(0)
                    }
                    _ => Ok(0),
                }
            }
            _ => Err(nb::Error::Other(res)),
        }
    }

    fn close(&mut self, mut socket: Self::TcpSocket) -> Result<(), Self::Error> {
        if socket.bound {
            unsafe { check!(OTUnbind_(socket.endpoint)) }; // FIXME: verify error handling here
        }
        socket.bound = false;
        Ok(())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        // TODO: Does below need to be handled?
        // Warning: You need to be sure that there are no outstanding
        // T_MEMORYRELEASED events for a provider before you close
        // the provider. Otherwise, Open Transport attempts to
        // deliver the event to a provider that no longer exists, with
        // unpredictable results, such as crashing the system.
        unsafe { OTCloseProvider_(self.endpoint) }; // Error ignored since we can't do much here
    }
}
