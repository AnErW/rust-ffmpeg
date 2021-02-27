//! Encoding/Decoding library

pub mod flag;
pub use self::flag::Flags;

pub mod id;
pub use self::id::Id;

pub mod packet;

pub mod subtitle;

pub mod picture;

pub mod discard;

pub mod context;
pub use self::context::Context;

pub mod capabilities;
pub use self::capabilities::Capabilities;
/// Codec Module
pub mod codec;

pub mod parameters;
pub use self::parameters::Parameters;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod audio_service;
pub mod field_order;

pub mod compliance;
pub use self::compliance::Compliance;

pub mod debug;
pub use self::debug::Debug;

pub mod profile;
pub use self::profile::Profile;

pub mod threading;

pub mod decoder;
pub mod encoder;
pub mod traits;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;
/// Return the version of `libavcodec`
pub fn version() -> u32 {
    unsafe { avcodec_version() }
}
/// Return the compile-time configuration of `libavcodec`
pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avcodec_configuration()).to_bytes()) }
}
/// Return the license of `libavcodec`
pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avcodec_license()).to_bytes()) }
}

#[test]
fn test_it() {
    println!("{},\n{},\n{}",self::version(),self::configuration(),self::license());
}