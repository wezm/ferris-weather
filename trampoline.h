//
// Created by wmoore on 29/03/23.
//

#ifndef TRAMPOLINE_H
#define TRAMPOLINE_H

#include <MacTypes.h>
#include <Dialogs.h>

#include <OpenTransport.h>
#include <OpenTransportProviders.h>

// Dialogs.h
DialogItemIndex StopAlert_(SInt16 alertID);
DialogItemIndex NoteAlert_(SInt16 alertID);

void ParamText_(
    ConstStr255Param param0,
    ConstStr255Param param1,
    ConstStr255Param param2,
    ConstStr255Param param3);

// OpenTransport.h
OSStatus InitOpenTransport_(void);

EndpointRef OTOpenEndpoint_(
    OTConfigurationRef cfig,
    OTOpenFlags oflag,
    TEndpointInfo *info,        /* can be NULL */
    OSStatus *err);

OTConfigurationRef OTCreateConfiguration_(const char *path);

OSStatus OTSetSynchronous_(ProviderRef ref);

OSStatus OTSetBlocking_(ProviderRef ref);

typedef void (*OTNotifyProcPtrC )(void *contextPtr, OTEventCode code, OTResult result, void *cookie);

typedef struct
{
    OTNotifyProcPtrC proc;
    void *contextPtr;
} NotifyProcContext;

OSStatus OTInstallNotifier_(ProviderRef ref, NotifyProcContext *ctxt);

OSStatus OTUseSyncIdleEvents_(ProviderRef ref, Boolean useEvents);

OSStatus OTBind_(
    EndpointRef ref,
    TBind *reqAddr,       /* can be NULL */
    TBind *retAddr);

OSStatus OTUnbind_(EndpointRef ref);

// void temp() {
//     OTOpenEndpoint()
//     OTSetSynchronous()
// }

void OTInitInetAddress_(
    InetAddress *addr,
    InetPort port,
    InetHost host);

OTByteCount OTInitDNSAddress_(DNSAddress *addr, char *str);

OSStatus
OTConnect_(
    EndpointRef ref,
    TCall *sndCall,
    TCall *rcvCall);      /* can be NULL */

OTResult
OTSnd_(
    EndpointRef ref,
    void *buf,
    OTByteCount nbytes,
    OTFlags flags);

OTResult
OTRcv_(
    EndpointRef ref,
    void *buf,
    OTByteCount nbytes,
    OTFlags *flags);

OTResult OTLook_(EndpointRef ref);

OSStatus OTRcvDisconnect_(
    EndpointRef ref,
    TDiscon *discon);      /* can be NULL */

OSStatus OTRcvOrderlyDisconnect_(EndpointRef ref);

OSStatus OTSndOrderlyDisconnect_(EndpointRef ref);

OSStatus OTCloseProvider_(ProviderRef ref);

void CloseOpenTransport_(void);

#endif //TRAMPOLINE_H
