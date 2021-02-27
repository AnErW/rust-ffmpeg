use std::ops::{Deref, DerefMut};

use ffi::*;
use libc::c_int;

use super::Opened;
use codec::Context;
use frame;
use util::format;
use {packet, AudioService, ChannelLayout, Error};
/// The audio decoder.
pub struct Audio(pub Opened);

impl Audio {
    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_decode_audio4 has been deprecated since FFmpeg 3.1; \
        consider switching to send_packet() and receive_frame()"
    )]
    pub fn decode<P: packet::Ref>(
        &mut self,
        packet: &P,
        out: &mut frame::Audio,
    ) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_decode_audio4(
                self.as_mut_ptr(),
                out.as_mut_ptr(),
                &mut got,
                packet.as_ptr(),
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }
    /// Get the sample rate of decoded audio.
    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }
    /// Get the total amount of channels.
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }
    /// Get the format of decoded audio.
    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }
    /// Set the format that the decoder will try to decode in
    /// this format if it can.
    pub fn request_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).request_sample_fmt = value.into();
        }
    }
    /// Get the frame total amount. 
    pub fn frames(&self) -> usize {
        unsafe { (*self.as_ptr()).frame_number as usize }
    }
    /// Get the number of bytes per packet.
    /// May return 0 in some WAV based audio codecs.
    pub fn align(&self) -> usize {
        unsafe { (*self.as_ptr()).block_align as usize }
    }
    /// Get the audio channel layout.
    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }
    /// Set the audio channel layout.
    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }
    /// Set the audio channel layout that the decoder will try to use this if it can.
    pub fn request_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).request_channel_layout = value.bits();
        }
    }
    /// Get the audio service type which is set by codec.
    pub fn audio_service(&mut self) -> AudioService {
        unsafe { AudioService::from((*self.as_mut_ptr()).audio_service_type) }
    }
    /// Get the max bit rate of audio.
    pub fn max_bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).rc_max_rate as usize }
    }
    /// Get the number of samples per channels in an audio frame.
    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }

    pub fn frame_start(&self) -> Option<usize> {
        unsafe {
            match (*self.as_ptr()).timecode_frame_start {
                -1 => None,
                n => Some(n as usize),
            }
        }
    }
}

impl Deref for Audio {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
