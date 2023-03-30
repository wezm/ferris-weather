// Functions to bridge the gap between Pascal calling-convention and C
// calling-convention.

#include "trampoline.h"

DialogItemIndex StopAlert_(SInt16 alertID)
{
    return StopAlert(alertID, NULL);
}

void ParamText_(ConstStr255Param param0, ConstStr255Param param1, ConstStr255Param param2, ConstStr255Param param3)
{
    ParamText(param0, param1, param2, param3);
}

OSStatus InitOpenTransport_(void)
{
    return InitOpenTransport();
}

EndpointRef OTOpenEndpoint_(OTConfigurationRef cfig, OTOpenFlags oflag, TEndpointInfo *info, OSStatus *err)
{
    return OTOpenEndpoint(cfig, oflag, info, err);
}

OTConfigurationRef OTCreateConfiguration_(const char *path)
{
    return OTCreateConfiguration(path);
}

OSStatus OTSetSynchronous_(ProviderRef ref)
{
    return OTSetAsynchronous(ref);
}

OSStatus OTSetBlocking_(ProviderRef ref)
{
    return OTSetBlocking(ref);
}

static pascal void OTNotifyProcTrampoline(void *contextPtr, OTEventCode code, OTResult result, void *cookie)
{
    // Call the real notify proc with its context
    NotifyProcContext *ctxt = (NotifyProcContext *) contextPtr;
    return ctxt->proc(ctxt->contextPtr, code, result, cookie);
}

OSStatus OTInstallNotifier_(ProviderRef ref, NotifyProcContext *ctxt)
{
    return OTInstallNotifier(ref, OTNotifyProcTrampoline, ctxt);
}

OSStatus OTUseSyncIdleEvents_(ProviderRef ref, Boolean useEvents)
{
    return OTUseSyncIdleEvents(ref, useEvents);
}

OSStatus OTBind_(EndpointRef ref, TBind *reqAddr, TBind *retAddr)
{
    return OTBind(ref, reqAddr, retAddr);
}

OSStatus OTUnbind_(EndpointRef ref)
{
    return OTUnbind(ref);
}

void OTInitInetAddress_(InetAddress *addr, InetPort port, InetHost host)
{
    OTInitInetAddress(addr, port, host);
}

OTByteCount OTInitDNSAddress_(DNSAddress *addr, char *str)
{
    return OTInitDNSAddress(addr, str);
}

OSStatus OTConnect_(EndpointRef ref, TCall *sndCall, TCall *rcvCall)
{
    return OTConnect(ref, sndCall, rcvCall);
}

OTResult OTSnd_(EndpointRef ref, void *buf, OTByteCount nbytes, OTFlags flags)
{
    return OTSnd(ref, buf, nbytes, flags);
}

OTResult OTRcv_(EndpointRef ref, void *buf, OTByteCount nbytes, OTFlags *flags)
{
    return OTRcv(ref, buf, nbytes, flags);
}

OTResult OTLook_(EndpointRef ref)
{
    return OTLook(ref);
}

OSStatus OTRcvDisconnect_(EndpointRef ref, TDiscon *discon)
{
    return OTRcvDisconnect(ref, discon);
}

OSStatus OTRcvOrderlyDisconnect_(EndpointRef ref)
{
    return OTRcvOrderlyDisconnect(ref);
}

OSStatus OTSndOrderlyDisconnect_(EndpointRef ref)
{
    return OTSndOrderlyDisconnect(ref);
}

OSStatus OTCloseProvider_(ProviderRef ref)
{
    return OTCloseProvider(ref);
}

void CloseOpenTransport_(void)
{
    CloseOpenTransport();
}
