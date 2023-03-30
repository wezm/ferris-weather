use core::ffi::{c_long, c_short, c_uchar, c_ulong, c_ushort};

mod open_transport;

pub use open_transport::{OpenTransport, Socket};

// Corresponds to noErr constant
pub const NO_ERR: OSStatus = 0;

// MacTypes.h
pub type SInt16 = c_short;
pub type SInt32 = c_long;

pub type UInt8 = c_uchar;
pub type UInt16 = c_ushort;
pub type UInt32 = c_ulong;

pub type ByteCount = UInt32;

pub type OSStatus = SInt32;

pub type Boolean = c_uchar;

pub const TRUE: Boolean = 1;
pub const FALSE: Boolean = 0;

pub type Str255 = [c_uchar; 256];

pub type ConstStr255Param = *const c_uchar;

// Dialogs.h
type DialogItemIndex = SInt16;

#[allow(unused, non_upper_case_globals)]
pub mod consts {
    /* OpenTransport errors*/
    use crate::toolbox::OSStatus;

    pub const OTNoError: OSStatus = 0; /* No Error occurred                    */
    pub const OTOutOfMemoryErr: OSStatus = -3211; /* OT ran out of memory, may be a temporary      */
    pub const OTNotFoundErr: OSStatus = -3201; /* OT generic not found error               */
    pub const OTDuplicateFoundErr: OSStatus = -3216; /* OT generic duplicate found error             */
    pub const OTBadAddressErr: OSStatus = -3150; /* XTI2OSStatus(TBADADDR) A Bad address was specified          */
    pub const OTBadOptionErr: OSStatus = -3151; /* XTI2OSStatus(TBADOPT) A Bad option was specified             */
    pub const OTAccessErr: OSStatus = -3152; /* XTI2OSStatus(TACCES) Missing access permission           */
    pub const OTBadReferenceErr: OSStatus = -3153; /* XTI2OSStatus(TBADF) Bad provider reference               */
    pub const OTNoAddressErr: OSStatus = -3154; /* XTI2OSStatus(TNOADDR) No address was specified           */
    pub const OTOutStateErr: OSStatus = -3155; /* XTI2OSStatus(TOUTSTATE) Call issued in wrong state           */
    pub const OTBadSequenceErr: OSStatus = -3156; /* XTI2OSStatus(TBADSEQ) Sequence specified does not exist         */
    pub const OTSysErrorErr: OSStatus = -3157; /* XTI2OSStatus(TSYSERR) A system error occurred            */
    pub const OTLookErr: OSStatus = -3158; /* XTI2OSStatus(TLOOK) An event occurred - call Look()         */
    pub const OTBadDataErr: OSStatus = -3159; /* XTI2OSStatus(TBADDATA) An illegal amount of data was specified */
    pub const OTBufferOverflowErr: OSStatus = -3160; /* XTI2OSStatus(TBUFOVFLW) Passed buffer not big enough          */
    pub const OTFlowErr: OSStatus = -3161; /* XTI2OSStatus(TFLOW) Provider is flow-controlled          */
    pub const OTNoDataErr: OSStatus = -3162; /* XTI2OSStatus(TNODATA) No data available for reading          */
    pub const OTNoDisconnectErr: OSStatus = -3163; /* XTI2OSStatus(TNODIS) No disconnect indication available         */
    pub const OTNoUDErrErr: OSStatus = -3164; /* XTI2OSStatus(TNOUDERR) No Unit Data Error indication available */
    pub const OTBadFlagErr: OSStatus = -3165; /* XTI2OSStatus(TBADFLAG) A Bad flag value was supplied          */
    pub const OTNoReleaseErr: OSStatus = -3166; /* XTI2OSStatus(TNOREL) No orderly release indication available   */
    pub const OTNotSupportedErr: OSStatus = -3167; /* XTI2OSStatus(TNOTSUPPORT) Command is not supported           */
    pub const OTStateChangeErr: OSStatus = -3168; /* XTI2OSStatus(TSTATECHNG) State is changing - try again later     */
    pub const OTNoStructureTypeErr: OSStatus = -3169; /* XTI2OSStatus(TNOSTRUCTYPE) Bad structure type requested for OTAlloc    */
    pub const OTBadNameErr: OSStatus = -3170; /* XTI2OSStatus(TBADNAME) A bad endpoint name was supplied         */
    pub const OTBadQLenErr: OSStatus = -3171; /* XTI2OSStatus(TBADQLEN) A Bind to an in-use addr with qlen > 0   */
    pub const OTAddressBusyErr: OSStatus = -3172; /* XTI2OSStatus(TADDRBUSY) Address requested is already in use       */
    pub const OTIndOutErr: OSStatus = -3173; /* XTI2OSStatus(TINDOUT) Accept failed because of pending listen  */
    pub const OTProviderMismatchErr: OSStatus = -3174; /* XTI2OSStatus(TPROVMISMATCH) Tried to accept on incompatible endpoint   */
    pub const OTResQLenErr: OSStatus = -3175; /* XTI2OSStatus(TRESQLEN)                            */
    pub const OTResAddressErr: OSStatus = -3176; /* XTI2OSStatus(TRESADDR)                            */
    pub const OTQFullErr: OSStatus = -3177; /* XTI2OSStatus(TQFULL)                          */
    pub const OTProtocolErr: OSStatus = -3178; /* XTI2OSStatus(TPROTO) An unspecified provider error occurred       */
    pub const OTBadSyncErr: OSStatus = -3179; /* XTI2OSStatus(TBADSYNC) A synchronous call at interrupt time       */
    pub const OTCanceledErr: OSStatus = -3180; /* XTI2OSStatus(TCANCELED) The command was cancelled            */
    pub const EPERMErr: OSStatus = -3200; /* Permission denied            */
    pub const ENOENTErr: OSStatus = -3201; /* No such file or directory       */
    pub const ENORSRCErr: OSStatus = -3202; /* No such resource               */
    pub const EINTRErr: OSStatus = -3203; /* Interrupted system service        */
    pub const EIOErr: OSStatus = -3204; /* I/O error                 */
    pub const ENXIOErr: OSStatus = -3205; /* No such device or address       */
    pub const EBADFErr: OSStatus = -3208; /* Bad file number                 */
    pub const EAGAINErr: OSStatus = -3210; /* Try operation again later       */
    pub const ENOMEMErr: OSStatus = -3211; /* Not enough space               */
    pub const EACCESErr: OSStatus = -3212; /* Permission denied            */
    pub const EFAULTErr: OSStatus = -3213; /* Bad address                   */
    pub const EBUSYErr: OSStatus = -3215; /* Device or resource busy          */
    pub const EEXISTErr: OSStatus = -3216; /* File exists                   */
    pub const ENODEVErr: OSStatus = -3218; /* No such device               */
    pub const EINVALErr: OSStatus = -3221; /* Invalid argument               */
    pub const ENOTTYErr: OSStatus = -3224; /* Not a character device          */
    pub const EPIPEErr: OSStatus = -3231; /* Broken pipe                   */
    pub const ERANGEErr: OSStatus = -3233; /* Message size too large for STREAM  */
    pub const EWOULDBLOCKErr: OSStatus = -3234; /* Call would block, so was aborted     */
    pub const EDEADLKErr: OSStatus = -3234; /* or a deadlock would occur       */
    pub const EALREADYErr: OSStatus = -3236; /*                          */
    pub const ENOTSOCKErr: OSStatus = -3237; /* Socket operation on non-socket     */
    pub const EDESTADDRREQErr: OSStatus = -3238; /* Destination address required      */
    pub const EMSGSIZEErr: OSStatus = -3239; /* Message too long               */
    pub const EPROTOTYPEErr: OSStatus = -3240; /* Protocol wrong type for socket     */
    pub const ENOPROTOOPTErr: OSStatus = -3241; /* Protocol not available          */
    pub const EPROTONOSUPPORTErr: OSStatus = -3242; /* Protocol not supported          */
    pub const ESOCKTNOSUPPORTErr: OSStatus = -3243; /* Socket type not supported       */
    pub const EOPNOTSUPPErr: OSStatus = -3244; /* Operation not supported on socket  */
    pub const EADDRINUSEErr: OSStatus = -3247; /* Address already in use          */
    pub const EADDRNOTAVAILErr: OSStatus = -3248; /* Can't assign requested address     */
    pub const ENETDOWNErr: OSStatus = -3249; /* Network is down                 */
    pub const ENETUNREACHErr: OSStatus = -3250; /* Network is unreachable          */
    pub const ENETRESETErr: OSStatus = -3251; /* Network dropped connection on reset    */
    pub const ECONNABORTEDErr: OSStatus = -3252; /* Software caused connection abort     */
    pub const ECONNRESETErr: OSStatus = -3253; /* Connection reset by peer          */
    pub const ENOBUFSErr: OSStatus = -3254; /* No buffer space available       */
    pub const EISCONNErr: OSStatus = -3255; /* Socket is already connected         */
    pub const ENOTCONNErr: OSStatus = -3256; /* Socket is not connected          */
    pub const ESHUTDOWNErr: OSStatus = -3257; /* Can't send after socket shutdown     */
    pub const ETOOMANYREFSErr: OSStatus = -3258; /* Too many references: can't splice  */
    pub const ETIMEDOUTErr: OSStatus = -3259; /* Connection timed out             */
    pub const ECONNREFUSEDErr: OSStatus = -3260; /* Connection refused           */
    pub const EHOSTDOWNErr: OSStatus = -3263; /* Host is down                */
    pub const EHOSTUNREACHErr: OSStatus = -3264; /* No route to host               */
    pub const EPROTOErr: OSStatus = -3269; /* ��� fill out missing codes ���     */
    pub const ETIMEErr: OSStatus = -3270; /*                          */
    pub const ENOSRErr: OSStatus = -3271; /*                          */
    pub const EBADMSGErr: OSStatus = -3272; /*                          */
    pub const ECANCELErr: OSStatus = -3273; /*                          */
    pub const ENOSTRErr: OSStatus = -3274; /*                          */
    pub const ENODATAErr: OSStatus = -3275; /*                          */
    pub const EINPROGRESSErr: OSStatus = -3276; /*                          */
    pub const ESRCHErr: OSStatus = -3277; /*                          */
    pub const ENOMSGErr: OSStatus = -3278; /*                          */
    pub const OTClientNotInittedErr: OSStatus = -3279; /*                          */
    pub const OTPortHasDiedErr: OSStatus = -3280; /*                          */
    pub const OTPortWasEjectedErr: OSStatus = -3281; /*                          */
    pub const OTBadConfigurationErr: OSStatus = -3282; /*                          */
    pub const OTConfigurationChangedErr: OSStatus = -3283; /*                          */
    pub const OTUserRequestedErr: OSStatus = -3284; /*                          */
    pub const OTPortLostConnection: OSStatus = -3285; /*                          */
}

extern "C" {
    pub fn StopAlert_(id: SInt16) -> DialogItemIndex;

    pub fn ParamText_(
        param1: ConstStr255Param,
        param2: ConstStr255Param,
        param3: ConstStr255Param,
        param4: ConstStr255Param,
    );
}
