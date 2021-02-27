//! Utility library for mutiple usages.
#[macro_use]
pub mod dictionary;
pub mod channel_layout;
pub mod chroma;
pub mod color;
pub mod error;
pub mod format;
pub mod frame;
pub mod interrupt;
pub mod log;
pub mod mathematics;
pub mod media;
pub mod option;
pub mod picture;
pub mod range;
pub mod rational;
pub mod time;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;


#[inline(always)]
/// Get the version of `libavutil`.
pub fn version() -> u32 {
    unsafe { avutil_version() }
}

#[inline(always)]
/// Get the compile-time configuration of `libavutil`.
pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avutil_configuration()).to_bytes()) }
}

#[inline(always)]
/// Get the license of `libavutil`
pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avutil_license()).to_bytes()) }
}
