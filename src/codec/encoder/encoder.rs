use std::ops::{Deref, DerefMut};
use std::ptr;

use ffi::*;
use libc::c_int;

use super::{audio, subtitle, video};
use codec::Context;
use {media, packet, Error, Frame, Rational};

/// Encoder entry.
pub struct Encoder(pub Context);

impl Encoder {
    /// Check if a encoder is a video encoder and return itself if the encoder is.
    pub fn video(mut self) -> Result<video::Video, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    (*self.as_mut_ptr()).codec_type = media::Type::Video.into();
                }

                Ok(video::Video(self))
            }

            media::Type::Video => Ok(video::Video(self)),

            _ => Err(Error::InvalidData),
        }
    }

    /// Check if a encoder is a audio encoder and return itself if the encoder is.
    pub fn audio(mut self) -> Result<audio::Audio, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    (*self.as_mut_ptr()).codec_type = media::Type::Audio.into();
                }

                Ok(audio::Audio(self))
            }

            media::Type::Audio => Ok(audio::Audio(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn subtitle(mut self) -> Result<subtitle::Subtitle, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    (*self.as_mut_ptr()).codec_type = media::Type::Subtitle.into();
                }

                Ok(subtitle::Subtitle(self))
            }

            media::Type::Subtitle => Ok(subtitle::Subtitle(self)),

            _ => Err(Error::InvalidData),
        }
    }
    /// Send a raw video or audio frame to the encoder.
    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match avcodec_send_frame(self.as_mut_ptr(), frame.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    /// Sends a NULL packet to the encoder to signal end of stream and enter
    /// draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe { self.send_frame(&Frame::wrap(ptr::null_mut())) }
    }
    /// Read the encoded data from the encoder.
    /// 
    /// To send data to encoder,see: [send_frame()].
    /// 
    /// [send_frame()]: Self::send_frame
    pub fn receive_packet<P: packet::Mut>(&mut self, packet: &mut P) -> Result<(), Error> {
        unsafe {
            match avcodec_receive_packet(self.as_mut_ptr(), packet.as_mut_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    /// Set the bit rate of encoder.
    pub fn set_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).bit_rate = value as i64;
        }
    }
    /// Set the max bit rate of encoder.
    pub fn set_max_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).rc_max_rate = value as i64;
        }
    }
    /// Set the bit rate tolerance of encoder.
    ///
    /// For more infomation of bit rate tolerance, see: [Variable bitrate](https://en.wikipedia.org/wiki/Variable_bitrate)
    pub fn set_tolerance(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).bit_rate_tolerance = value as c_int;
        }
    }
    /// Set the encode global quality.
    pub fn set_quality(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).global_quality = value as c_int;
        }
    }
    /// Set the compression level of encoder.
    pub fn set_compression(&mut self, value: Option<usize>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).compression_level = value as c_int;
            } else {
                (*self.as_mut_ptr()).compression_level = -1;
            }
        }
    }
    /// Set the timestamp of frames.
    ///
    /// The unit of time should be in seconds.
    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }
    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: Option<R>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).framerate = value.into().into();
            } else {
                (*self.as_mut_ptr()).framerate.num = 0;
                (*self.as_mut_ptr()).framerate.den = 1;
            }
        }
    }
}

impl Deref for Encoder {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut *self
    }
}
