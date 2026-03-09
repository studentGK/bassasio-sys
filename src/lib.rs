//! # bassasio-sys
//!
//! Raw FFI bindings for **BASSASIO 1.4** — the ASIO audio wrapper that sits on
//! top of the BASS audio library (Un4seen Developments Ltd.).
//!
//! This crate depends on [`bass-sys`] for all shared primitive types and error
//! codes.  Only BASSASIO-specific symbols are defined here.
//!
//! ## Shared types from bass-sys
//! The following are re-exported from `bass-sys` so callers only need one crate:
//!
//! * Primitive aliases: `BOOL`, `DWORD`, `QWORD`, `TRUE`, `FALSE`
//! * Handle types: `HSTREAM` (used by `BASS_ASIO_ChannelEnableBASS`)
//! * BASS error codes: `BASS_OK`, `BASS_ERROR_*`
//! * `BassError` error type
//!
//! ## Windows-only
//! ASIO is a Windows-exclusive technology.  This entire crate is
//! `#[cfg(target_os = "windows")]`-gated.
//!
//! ## Constant verification
//! All numeric values verified against `bassasio.h`
//! (BASSASIO 1.4, copyright 2005-2023 Un4seen Developments).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
#![cfg(target_os = "windows")]

use std::os::raw::{c_char, c_float, c_int, c_void};

// ─── Re-export shared types and constants from bass-sys ──────────────────────
pub use bass_sys::{
    // Primitive types
    BOOL, DWORD, QWORD,
    // Handle type used by BASSASIO
    HSTREAM,
    // BASS error codes reused by BASSASIO
    BASS_OK,
    BASS_ERROR_FILEOPEN,
    BASS_ERROR_DRIVER,
    BASS_ERROR_HANDLE,
    BASS_ERROR_FORMAT,
    BASS_ERROR_INIT,
    BASS_ERROR_START,
    BASS_ERROR_ALREADY,
    BASS_ERROR_NOCHAN,
    BASS_ERROR_ILLPARAM,
    BASS_ERROR_DEVICE,
    BASS_ERROR_NOTAVAIL,
    BASS_ERROR_UNKNOWN,
};

// TRUE / FALSE — bass-sys defines BOOL = i32 but does not export these constants.
pub const TRUE:  BOOL = 1;
pub const FALSE: BOOL = 0;

/// A BASS/BASSASIO error code (wraps the raw integer returned by ErrorGetCode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BassError(pub std::os::raw::c_int);
impl std::fmt::Display for BassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BASS error {}", self.0)
    }
}
impl std::error::Error for BassError {}

// ─── BASSASIO version ─────────────────────────────────────────────────────────
/// API version for BASSASIO 1.4.
/// Verified: bassasio.h `#define BASSASIOVERSION 0x104`
pub const BASSASIOVERSION: DWORD = 0x0104;

// ─── BASS_ASIO_Init flags ─────────────────────────────────────────────────────
pub const BASS_ASIO_THREAD:    DWORD = 1;
pub const BASS_ASIO_JOINORDER: DWORD = 2;

// ─── ASIONOTIFYPROC notification codes ───────────────────────────────────────
pub const BASS_ASIO_NOTIFY_RATE:  DWORD = 1;
pub const BASS_ASIO_NOTIFY_RESET: DWORD = 2;

// ─── BASS_ASIO_ChannelIsActive return values ─────────────────────────────────
pub const BASS_ASIO_ACTIVE_DISABLED: DWORD = 0;
pub const BASS_ASIO_ACTIVE_ENABLED:  DWORD = 1;
pub const BASS_ASIO_ACTIVE_PAUSED:   DWORD = 2;

// ─── Channel sample formats (all verified against bassasio.h) ────────────────
pub const BASS_ASIO_FORMAT_16BIT:   DWORD = 16;
pub const BASS_ASIO_FORMAT_24BIT:   DWORD = 17;
pub const BASS_ASIO_FORMAT_32BIT:   DWORD = 18;
pub const BASS_ASIO_FORMAT_FLOAT:   DWORD = 19;
pub const BASS_ASIO_FORMAT_32BIT16: DWORD = 24;
pub const BASS_ASIO_FORMAT_32BIT18: DWORD = 25;
pub const BASS_ASIO_FORMAT_32BIT20: DWORD = 26;
pub const BASS_ASIO_FORMAT_32BIT24: DWORD = 27;
pub const BASS_ASIO_FORMAT_DSD_LSB: DWORD = 32;
pub const BASS_ASIO_FORMAT_DSD_MSB: DWORD = 33;
/// Flag: apply TPDF dither when converting float → integer. OR with a PCM format.
pub const BASS_ASIO_FORMAT_DITHER:  DWORD = 0x100;

// ─── BASS_ASIO_ChannelReset flags (all verified against bassasio.h) ──────────
pub const BASS_ASIO_RESET_ENABLE: DWORD = 1;
pub const BASS_ASIO_RESET_JOIN:   DWORD = 2;
pub const BASS_ASIO_RESET_PAUSE:  DWORD = 4;
pub const BASS_ASIO_RESET_FORMAT: DWORD = 8;
pub const BASS_ASIO_RESET_RATE:   DWORD = 16;
pub const BASS_ASIO_RESET_VOLUME: DWORD = 32;
/// Also apply the reset to channels joined to the specified channel.
pub const BASS_ASIO_RESET_JOINED: DWORD = 0x10000;

// ─── BASS_ASIO_ChannelGetLevel flag ──────────────────────────────────────────
/// OR into `channel` to request RMS level instead of peak level.
pub const BASS_ASIO_LEVEL_RMS: DWORD = 0x1000000;

// ─── Structures ───────────────────────────────────────────────────────────────

/// Device info. Used with [`BASS_ASIO_GetDeviceInfo`] / [`BASS_ASIO_AddDevice`].
/// Pointers are owned by BASSASIO — do not free them.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BASS_ASIO_DEVICEINFO {
    pub name:   *const c_char,
    pub driver: *const c_char,
}
unsafe impl Send for BASS_ASIO_DEVICEINFO {}
unsafe impl Sync for BASS_ASIO_DEVICEINFO {}

/// Per-channel info. Used with [`BASS_ASIO_ChannelGetInfo`].
/// `format` reflects the native format, unaffected by [`BASS_ASIO_ChannelSetFormat`].
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BASS_ASIO_CHANNELINFO {
    pub group:  DWORD,
    pub format: DWORD,
    pub name:   [c_char; 32],
}

/// Current-device info. Used with [`BASS_ASIO_GetInfo`].
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BASS_ASIO_INFO {
    pub name:      [c_char; 32],
    pub version:   DWORD,
    pub inputs:    DWORD,
    pub outputs:   DWORD,
    pub bufmin:    DWORD,
    pub bufmax:    DWORD,
    pub bufpref:   DWORD,
    pub bufgran:   c_int,
    pub initflags: DWORD,
}

/// Windows `CLSID` / `GUID` (for [`BASS_ASIO_AddDevice`]).
/// Layout is identical to `windows_sys::core::GUID`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

// ─── Callback types ───────────────────────────────────────────────────────────

/// ASIO channel callback (`ASIOPROC`).
/// For output channels: return bytes written (negative → 0).
/// For input channels: return value is ignored.
///
/// # Safety
/// Called from the ASIO thread. Do NOT call [`BASS_ASIO_Stop`] or
/// [`BASS_ASIO_Free`] from within this callback.
pub type ASIOPROC = unsafe extern "system" fn(
    input: BOOL, channel: DWORD, buffer: *mut c_void, length: DWORD, user: *mut c_void,
) -> DWORD;

/// Driver notification callback (`ASIONOTIFYPROC`).
/// Do not reinitialize the device from within this callback.
pub type ASIONOTIFYPROC = unsafe extern "system" fn(notify: DWORD, user: *mut c_void);

// ─── Extern function declarations ────────────────────────────────────────────
extern "system" {
    // Device management
    pub fn BASS_ASIO_AddDevice(clsid: *const GUID, driver: *const c_char, name: *const c_char) -> DWORD;
    pub fn BASS_ASIO_Init(device: c_int, flags: DWORD) -> BOOL;
    pub fn BASS_ASIO_Free() -> BOOL;
    pub fn BASS_ASIO_GetDeviceInfo(device: DWORD, info: *mut BASS_ASIO_DEVICEINFO) -> BOOL;
    pub fn BASS_ASIO_GetInfo(info: *mut BASS_ASIO_INFO) -> BOOL;
    pub fn BASS_ASIO_SetDevice(device: DWORD) -> BOOL;
    pub fn BASS_ASIO_GetDevice() -> DWORD;
    pub fn BASS_ASIO_SetUnicode(unicode: BOOL) -> BOOL;

    // Version & diagnostics
    pub fn BASS_ASIO_GetVersion() -> DWORD;
    pub fn BASS_ASIO_ErrorGetCode() -> DWORD;
    pub fn BASS_ASIO_GetCPU() -> c_float;

    // Device start/stop
    pub fn BASS_ASIO_Start(buflen: DWORD, threads: DWORD) -> BOOL;
    pub fn BASS_ASIO_Stop() -> BOOL;
    pub fn BASS_ASIO_IsStarted() -> BOOL;

    // Rate & latency
    pub fn BASS_ASIO_SetRate(rate: f64) -> BOOL;
    pub fn BASS_ASIO_GetRate() -> f64;
    pub fn BASS_ASIO_CheckRate(rate: f64) -> BOOL;
    /// ⚠ Call after `BASS_ASIO_Start`; value depends on buffer size.
    pub fn BASS_ASIO_GetLatency(input: BOOL) -> DWORD;

    // Notifications
    pub fn BASS_ASIO_SetNotify(proc_: Option<ASIONOTIFYPROC>, user: *mut c_void) -> BOOL;

    // DSD / Lock / UI / Future
    pub fn BASS_ASIO_SetDSD(dsd: BOOL) -> BOOL;
    pub fn BASS_ASIO_Lock(lock: BOOL) -> BOOL;
    /// ⚠ Call from the same thread as `BASS_ASIO_Init` if `BASS_ASIO_THREAD` was not used.
    pub fn BASS_ASIO_ControlPanel() -> BOOL;
    pub fn BASS_ASIO_Future(selector: DWORD, param: *mut c_void) -> BOOL;

    // Direct monitoring
    pub fn BASS_ASIO_Monitor(input: c_int, output: DWORD, gain: DWORD, state: DWORD, pan: DWORD) -> BOOL;

    // Channel enable/disable
    pub fn BASS_ASIO_ChannelEnable(input: BOOL, channel: DWORD, proc_: Option<ASIOPROC>, user: *mut c_void) -> BOOL;
    /// `handle` must be `BASS_STREAM_DECODE` (output) or `STREAMPROC_PUSH/DUMMY` (input).
    /// ⚠ 8-bit BASS channels unsupported; use `BASS_SAMPLE_FLOAT`.
    pub fn BASS_ASIO_ChannelEnableBASS(input: BOOL, channel: DWORD, handle: HSTREAM, join: BOOL) -> BOOL;
    /// ⚠ Mirror channels cannot be joined together.
    pub fn BASS_ASIO_ChannelEnableMirror(channel: DWORD, input2: BOOL, channel2: DWORD) -> BOOL;
    /// Pass `channel2 = -1` to remove the current join.
    pub fn BASS_ASIO_ChannelJoin(input: BOOL, channel: DWORD, channel2: c_int) -> BOOL;
    /// Pass `channel = 0xFFFF_FFFF` for "all channels".
    pub fn BASS_ASIO_ChannelPause(input: BOOL, channel: DWORD) -> BOOL;
    /// `flags` is a combination of `BASS_ASIO_RESET_*`. Pass `channel = -1` for all.
    pub fn BASS_ASIO_ChannelReset(input: BOOL, channel: c_int, flags: DWORD) -> BOOL;

    // Channel queries
    pub fn BASS_ASIO_ChannelIsActive(input: BOOL, channel: DWORD) -> DWORD;
    pub fn BASS_ASIO_ChannelGetInfo(input: BOOL, channel: DWORD, info: *mut BASS_ASIO_CHANNELINFO) -> BOOL;

    // Format & rate
    pub fn BASS_ASIO_ChannelGetFormat(input: BOOL, channel: DWORD) -> DWORD;
    pub fn BASS_ASIO_ChannelSetFormat(input: BOOL, channel: DWORD, format: DWORD) -> BOOL;
    pub fn BASS_ASIO_ChannelGetRate(input: BOOL, channel: DWORD) -> f64;
    pub fn BASS_ASIO_ChannelSetRate(input: BOOL, channel: DWORD, rate: f64) -> BOOL;

    // Volume (`channel = -1` = master)
    pub fn BASS_ASIO_ChannelGetVolume(input: BOOL, channel: c_int) -> c_float;
    pub fn BASS_ASIO_ChannelSetVolume(input: BOOL, channel: c_int, volume: c_float) -> BOOL;

    // Level (OR `BASS_ASIO_LEVEL_RMS` into `channel` for RMS)
    pub fn BASS_ASIO_ChannelGetLevel(input: BOOL, channel: DWORD) -> c_float;
}

// ─── Safe wrappers ────────────────────────────────────────────────────────────

/// Retrieve the last BASSASIO error code for the current thread.
#[inline]
pub fn asio_error_get_code() -> DWORD {
    unsafe { BASS_ASIO_ErrorGetCode() }
}

fn asio_bool_result(ok: BOOL) -> Result<(), BassError> {
    if ok != FALSE { Ok(()) } else { Err(BassError(asio_error_get_code() as c_int)) }
}

pub fn init(device: i32, flags: DWORD) -> Result<(), BassError> {
    asio_bool_result(unsafe { BASS_ASIO_Init(device, flags) })
}
pub fn free() -> Result<(), BassError> {
    asio_bool_result(unsafe { BASS_ASIO_Free() })
}
pub fn start(buflen: DWORD, threads: DWORD) -> Result<(), BassError> {
    asio_bool_result(unsafe { BASS_ASIO_Start(buflen, threads) })
}
pub fn stop() -> Result<(), BassError> {
    asio_bool_result(unsafe { BASS_ASIO_Stop() })
}
pub fn set_rate(rate: f64) -> Result<(), BassError> {
    asio_bool_result(unsafe { BASS_ASIO_SetRate(rate) })
}
pub fn get_rate() -> Result<f64, BassError> {
    let r = unsafe { BASS_ASIO_GetRate() };
    if r < 0.0 { Err(BassError(asio_error_get_code() as c_int)) } else { Ok(r) }
}

/// # Safety
/// `proc_fn` is called from the ASIO thread. `user_data` must outlive the
/// device and be safe to share across threads.
pub unsafe fn channel_enable(
    input: bool, channel: DWORD, proc_fn: Option<ASIOPROC>, user_data: *mut c_void,
) -> Result<(), BassError> {
    asio_bool_result(BASS_ASIO_ChannelEnable(
        if input { TRUE } else { FALSE }, channel, proc_fn, user_data,
    ))
}
pub fn channel_enable_bass(input: bool, channel: DWORD, handle: HSTREAM, join: bool) -> Result<(), BassError> {
    asio_bool_result(unsafe {
        BASS_ASIO_ChannelEnableBASS(
            if input { TRUE } else { FALSE }, channel, handle, if join { TRUE } else { FALSE },
        )
    })
}
pub fn channel_reset(input: bool, channel: i32, flags: DWORD) -> Result<(), BassError> {
    asio_bool_result(unsafe {
        BASS_ASIO_ChannelReset(if input { TRUE } else { FALSE }, channel, flags)
    })
}
pub fn channel_set_volume(input: bool, channel: i32, volume: f32) -> Result<(), BassError> {
    asio_bool_result(unsafe {
        BASS_ASIO_ChannelSetVolume(if input { TRUE } else { FALSE }, channel, volume)
    })
}
pub fn channel_get_volume(input: bool, channel: i32) -> Result<f32, BassError> {
    let v = unsafe { BASS_ASIO_ChannelGetVolume(if input { TRUE } else { FALSE }, channel) };
    if v < 0.0 { Err(BassError(asio_error_get_code() as c_int)) } else { Ok(v) }
}
pub fn channel_get_level(input: bool, channel: DWORD) -> Result<f32, BassError> {
    let v = unsafe { BASS_ASIO_ChannelGetLevel(if input { TRUE } else { FALSE }, channel) };
    if v < 0.0 { Err(BassError(asio_error_get_code() as c_int)) } else { Ok(v) }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_constants_are_distinct() {
        let formats = [
            BASS_ASIO_FORMAT_16BIT, BASS_ASIO_FORMAT_24BIT, BASS_ASIO_FORMAT_32BIT,
            BASS_ASIO_FORMAT_FLOAT, BASS_ASIO_FORMAT_32BIT16, BASS_ASIO_FORMAT_32BIT18,
            BASS_ASIO_FORMAT_32BIT20, BASS_ASIO_FORMAT_32BIT24,
            BASS_ASIO_FORMAT_DSD_LSB, BASS_ASIO_FORMAT_DSD_MSB,
        ];
        for (i, a) in formats.iter().enumerate() {
            for (j, b) in formats.iter().enumerate() {
                if i != j { assert_ne!(a, b, "[{i}] and [{j}] collide: {a:#x}"); }
            }
            assert_eq!(a & BASS_ASIO_FORMAT_DITHER, 0, "{a:#x} overlaps DITHER");
        }
    }

    #[test]
    fn reset_flags_are_powers_of_two() {
        let flags = [
            BASS_ASIO_RESET_ENABLE, BASS_ASIO_RESET_JOIN, BASS_ASIO_RESET_PAUSE,
            BASS_ASIO_RESET_FORMAT, BASS_ASIO_RESET_RATE, BASS_ASIO_RESET_VOLUME,
        ];
        for f in &flags {
            assert!(*f != 0 && (f & (f - 1)) == 0, "{f:#x} is not a power of 2");
            assert_eq!(f & BASS_ASIO_RESET_JOINED, 0, "{f:#x} collides with RESET_JOINED");
        }
    }

    #[test]
    fn struct_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<BASS_ASIO_CHANNELINFO>(), 40);
        assert_eq!(size_of::<BASS_ASIO_INFO>(), 64);
        #[cfg(target_pointer_width = "64")]
        assert_eq!(size_of::<BASS_ASIO_DEVICEINFO>(), 16);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(size_of::<BASS_ASIO_DEVICEINFO>(), 8);
    }

    /// Verify re-exported types from bass-sys are accessible.
    #[test]
    fn reexported_types_accessible() {
        let _: DWORD = BASS_OK as u32;
        let _: BOOL  = TRUE;
        let _: HSTREAM = 0;
        let _: BassError = BassError(0);
    }
}
