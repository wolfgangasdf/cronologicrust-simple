/* automatically generated by rust-bindgen 0.58.1 */

pub type USHORT = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TDC {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Config {
    _unused: [u8; 0],
}
pub type HIT = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TDCConfigException {
    pub errorString: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TDCHit {
    pub time: ::std::os::raw::c_longlong,
    pub channel: ::std::os::raw::c_uchar,
    pub type_: ::std::os::raw::c_uchar,
    pub bin: ::std::os::raw::c_ushort,
}
pub const TDCHit_RISING: ::std::os::raw::c_int = 1;
pub const TDCHit_FALLING: ::std::os::raw::c_int = 0;
pub const TDCHit_TDC_ERROR: ::std::os::raw::c_int = 2;
#[doc = " This class contains the information about one TDC"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TDCInfo {
    pub index: ::std::os::raw::c_int,
    pub channelStart: ::std::os::raw::c_int,
    pub channelCount: ::std::os::raw::c_int,
    pub highResChannelCount: ::std::os::raw::c_int,
    pub highResChannelStart: ::std::os::raw::c_int,
    pub lowResChannelCount: ::std::os::raw::c_int,
    pub lowResChannelStart: ::std::os::raw::c_int,
    pub resolution: f64,
    pub serialNumber: DWORD,
    pub version: ::std::os::raw::c_int,
    pub fifoSize: ::std::os::raw::c_int,
    pub INLCorrection: *mut ::std::os::raw::c_int,
    pub DNLData: *mut ::std::os::raw::c_ushort,
    pub flashValid: bool,
    pub boardConfiguration: ::std::os::raw::c_uchar,
    pub reserved2: ::std::os::raw::c_ushort,
    pub bufferSize: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Frame {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TDCManager {
    pub tdcs: [*mut TDC; 5usize],
    pub tdcCount: ::std::os::raw::c_int,
    pub VendorID: USHORT,
    pub DeviceID: USHORT,
    pub readThrottle: bool,
    pub config: *mut Config,
    pub groupingEnabled: bool,
    pub externalClock: bool,
    pub state: ::std::os::raw::c_int,
    pub configChanged: bool,
    pub tempIndex: ::std::os::raw::c_int,
    pub tempCount: ::std::os::raw::c_int,
    pub frameStates: [TDCManager_TDCFrameState; 5usize],
    pub lastRolloverTime: ::std::os::raw::c_int,
    pub metaRolloverTime: ::std::os::raw::c_int,
    pub mergeBuffers: [*mut DWORD; 5usize],
    pub readCounts: [::std::os::raw::c_int; 5usize],
    pub mergeBufferCurrent: [bool; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TDCManager_TDCFrameState {
    pub frame: *mut Frame,
    pub position: ::std::os::raw::c_int,
    pub rollover: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "\u{1}?Init@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_Init(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?Init@TDCManager@@QEAAXHH@Z"]
    pub fn TDCManager_Init1(
        this: *mut TDCManager,
        startDevice: ::std::os::raw::c_int,
        deviceCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}?Start@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_Start(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?Stop@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_Stop(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?Pause@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_Pause(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?getDriverVersion@TDCManager@@QEAAHXZ"]
    pub fn TDCManager_getDriverVersion(this: *mut TDCManager) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}?Continue@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_Continue(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?Reconfigure@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_Reconfigure(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?SetParameter@TDCManager@@QEAA_NPEBD0@Z"]
    pub fn TDCManager_SetParameter(
        this: *mut TDCManager,
        property: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?SetParameter@TDCManager@@QEAA_NPEBD@Z"]
    pub fn TDCManager_SetParameter1(
        this: *mut TDCManager,
        config: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?ReadConfigFile@TDCManager@@QEAA_NPEBD@Z"]
    pub fn TDCManager_ReadConfigFile(
        this: *mut TDCManager,
        filename: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?ReadConfigString@TDCManager@@QEAA_NPEBD@Z"]
    pub fn TDCManager_ReadConfigString(
        this: *mut TDCManager,
        parameter: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?GetParameter@TDCManager@@QEAAPEBDPEBD@Z"]
    pub fn TDCManager_GetParameter(
        this: *mut TDCManager,
        parameter: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Returns all parameter names count contains the number of elements in array"]
    #[doc = "the result must be freed with \"delete[] params\""]
    #[link_name = "\u{1}?GetParameterNames@TDCManager@@QEAAPEAPEBDAEAH@Z"]
    pub fn TDCManager_GetParameterNames(
        this: *mut TDCManager,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}?CleanUp@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_CleanUp(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?Read@TDCManager@@QEAAHPEAKH@Z"]
    pub fn TDCManager_Read(
        this: *mut TDCManager,
        out: *mut HIT,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}?ReadTDCHit@TDCManager@@QEAAHPEAUTDCHit@@H@Z"]
    pub fn TDCManager_ReadTDCHit(
        this: *mut TDCManager,
        buffer: *mut TDCHit,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}?ReadTDCHitSince@TDCManager@@QEAAHPEAUTDCHit@@H_J@Z"]
    pub fn TDCManager_ReadTDCHitSince(
        this: *mut TDCManager,
        buffer: *mut TDCHit,
        length: ::std::os::raw::c_int,
        since: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}?ReadNextFrame@TDCManager@@QEAAHPEAKH@Z"]
    pub fn TDCManager_ReadNextFrame(
        this: *mut TDCManager,
        out: *mut HIT,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}?PrettyPrint@TDCManager@@SAXKPEAD@Z"]
    pub fn TDCManager_PrettyPrint1(hit: HIT, buffer: *mut ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}?getTDCInfo@TDCManager@@QEAA?AVTDCInfo@@H@Z"]
    pub fn TDCManager_getTDCInfo(this: *mut TDCManager, index: ::std::os::raw::c_int) -> TDCInfo;
}
extern "C" {
    #[doc = " Clears all remaining data which is not yet read"]
    #[link_name = "\u{1}?ClearBuffer@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_ClearBuffer(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?GetTDCStatusRegister@TDCManager@@QEAA_JH@Z"]
    pub fn TDCManager_GetTDCStatusRegister(
        this: *mut TDCManager,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " Internal methods"]
    #[link_name = "\u{1}?EmergencyCleanUp@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_EmergencyCleanUp(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?updateThrottle@TDCManager@@QEAA_N_NH@Z"]
    pub fn TDCManager_updateThrottle(
        this: *mut TDCManager,
        value: bool,
        freeBuffer: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?PokeD@TDCManager@@QEAAXHHK@Z"]
    pub fn TDCManager_PokeD(
        this: *mut TDCManager,
        index: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
        value: DWORD,
    );
}
extern "C" {
    #[link_name = "\u{1}?PeekD@TDCManager@@QEAAKHH@Z"]
    pub fn TDCManager_PeekD(
        this: *mut TDCManager,
        index: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    ) -> DWORD;
}
extern "C" {
    #[link_name = "\u{1}?WriteConfigFlash@TDCManager@@QEAAXXZ"]
    pub fn TDCManager_WriteConfigFlash(this: *mut TDCManager);
}
extern "C" {
    #[link_name = "\u{1}?SetTDCSerialNumber@TDCManager@@QEAAXHK@Z"]
    pub fn TDCManager_SetTDCSerialNumber(
        this: *mut TDCManager,
        index: ::std::os::raw::c_int,
        serialNumber: DWORD,
    );
}
extern "C" {
    #[link_name = "\u{1}?SetMaxDevices@TDCManager@@QEAAXH@Z"]
    pub fn TDCManager_SetMaxDevices(this: *mut TDCManager, deviceCount: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}??0TDCManager@@QEAA@GG@Z"]
    pub fn TDCManager_TDCManager(this: *mut TDCManager, _VendorID: USHORT, _DeviceID: USHORT);
}
impl TDCManager {
    #[inline]
    pub unsafe fn Init(&mut self) {
        TDCManager_Init(self)
    }
    #[inline]
    pub unsafe fn Init1(
        &mut self,
        startDevice: ::std::os::raw::c_int,
        deviceCount: ::std::os::raw::c_int,
    ) {
        TDCManager_Init1(self, startDevice, deviceCount)
    }
    #[inline]
    pub unsafe fn Start(&mut self) {
        TDCManager_Start(self)
    }
    #[inline]
    pub unsafe fn Stop(&mut self) {
        TDCManager_Stop(self)
    }
    #[inline]
    pub unsafe fn Pause(&mut self) {
        TDCManager_Pause(self)
    }
    #[inline]
    pub unsafe fn getDriverVersion(&mut self) -> ::std::os::raw::c_int {
        TDCManager_getDriverVersion(self)
    }
    #[inline]
    pub unsafe fn Continue(&mut self) {
        TDCManager_Continue(self)
    }
    #[inline]
    pub unsafe fn Reconfigure(&mut self) {
        TDCManager_Reconfigure(self)
    }
    #[inline]
    pub unsafe fn SetParameter(
        &mut self,
        property: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool {
        TDCManager_SetParameter(self, property, value)
    }
    #[inline]
    pub unsafe fn SetParameter1(&mut self, config: *const ::std::os::raw::c_char) -> bool {
        TDCManager_SetParameter1(self, config)
    }
    #[inline]
    pub unsafe fn ReadConfigFile(&mut self, filename: *const ::std::os::raw::c_char) -> bool {
        TDCManager_ReadConfigFile(self, filename)
    }
    #[inline]
    pub unsafe fn ReadConfigString(&mut self, parameter: *const ::std::os::raw::c_char) -> bool {
        TDCManager_ReadConfigString(self, parameter)
    }
    #[inline]
    pub unsafe fn GetParameter(
        &mut self,
        parameter: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        TDCManager_GetParameter(self, parameter)
    }
    #[inline]
    pub unsafe fn GetParameterNames(
        &mut self,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *const ::std::os::raw::c_char {
        TDCManager_GetParameterNames(self, count)
    }
    #[inline]
    pub unsafe fn CleanUp(&mut self) {
        TDCManager_CleanUp(self)
    }
    #[inline]
    pub unsafe fn Read(
        &mut self,
        out: *mut HIT,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        TDCManager_Read(self, out, size)
    }
    #[inline]
    pub unsafe fn ReadTDCHit(
        &mut self,
        buffer: *mut TDCHit,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        TDCManager_ReadTDCHit(self, buffer, length)
    }
    #[inline]
    pub unsafe fn ReadTDCHitSince(
        &mut self,
        buffer: *mut TDCHit,
        length: ::std::os::raw::c_int,
        since: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int {
        TDCManager_ReadTDCHitSince(self, buffer, length, since)
    }
    #[inline]
    pub unsafe fn ReadNextFrame(
        &mut self,
        out: *mut HIT,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        TDCManager_ReadNextFrame(self, out, size)
    }
    #[inline]
    pub unsafe fn PrettyPrint(hit: HIT, buffer: *mut ::std::os::raw::c_char) {
        TDCManager_PrettyPrint1(hit, buffer)
    }
    #[inline]
    pub unsafe fn getTDCInfo(&mut self, index: ::std::os::raw::c_int) -> TDCInfo {
        TDCManager_getTDCInfo(self, index)
    }
    #[inline]
    pub unsafe fn ClearBuffer(&mut self) {
        TDCManager_ClearBuffer(self)
    }
    #[inline]
    pub unsafe fn GetTDCStatusRegister(
        &mut self,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong {
        TDCManager_GetTDCStatusRegister(self, index)
    }
    #[inline]
    pub unsafe fn EmergencyCleanUp(&mut self) {
        TDCManager_EmergencyCleanUp(self)
    }
    #[inline]
    pub unsafe fn updateThrottle(
        &mut self,
        value: bool,
        freeBuffer: ::std::os::raw::c_int,
    ) -> bool {
        TDCManager_updateThrottle(self, value, freeBuffer)
    }
    #[inline]
    pub unsafe fn PokeD(
        &mut self,
        index: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
        value: DWORD,
    ) {
        TDCManager_PokeD(self, index, offset, value)
    }
    #[inline]
    pub unsafe fn PeekD(
        &mut self,
        index: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    ) -> DWORD {
        TDCManager_PeekD(self, index, offset)
    }
    #[inline]
    pub unsafe fn WriteConfigFlash(&mut self) {
        TDCManager_WriteConfigFlash(self)
    }
    #[inline]
    pub unsafe fn SetTDCSerialNumber(&mut self, index: ::std::os::raw::c_int, serialNumber: DWORD) {
        TDCManager_SetTDCSerialNumber(self, index, serialNumber)
    }
    #[inline]
    pub unsafe fn SetMaxDevices(&mut self, deviceCount: ::std::os::raw::c_int) {
        TDCManager_SetMaxDevices(self, deviceCount)
    }
    #[inline]
    pub unsafe fn new(_VendorID: USHORT, _DeviceID: USHORT) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        TDCManager_TDCManager(__bindgen_tmp.as_mut_ptr(), _VendorID, _DeviceID);
        __bindgen_tmp.assume_init()
    }
}
