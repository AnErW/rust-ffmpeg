use std::ops::{Deref, DerefMut};
use std::ptr;

use super::{Audio, Check, Conceal, Opened, Subtitle, Video};
use codec::{traits, Context};
use ffi::*;
use {Dictionary, Discard, Error, Rational};

pub struct Decoder(pub Context);

impl Decoder {
    /// Initialize the decoder and codec context.
    pub fn open(mut self) -> Result<Opened, Error> {
        unsafe {
            match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => Ok(Opened(self)),
                e => Err(Error::from(e)),
            }
        }
    }
    /// Initialize decoder and context with given decoder.
    pub fn open_as<D: traits::Decoder>(mut self, codec: D) -> Result<Opened, Error> {
        unsafe {
            if let Some(codec) = codec.decoder() {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::DecoderNotFound)
            }
        }
    }
    /// Initialize decoder with given options and decoder.
    pub fn open_as_with<D: traits::Decoder>(
        mut self,
        codec: D,
        options: Dictionary,
    ) -> Result<Opened, Error> {
        unsafe {
            if let Some(codec) = codec.decoder() {
                let mut opts = options.disown();
                let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::DecoderNotFound)
            }
        }
    }
    /// Check if the decoder is a video decoder 
    /// and return the context if the decoder is.
    pub fn video(self) -> Result<Video, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.video())
        } else {
            Err(Error::DecoderNotFound)
        }
    }
    /// Check if the decoder is an audio decoder
    /// and return the context if the decoder is.
    pub fn audio(self) -> Result<Audio, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.audio())
        } else {
            Err(Error::DecoderNotFound)
        }
    }
    /// Check and get the subtitle context 
    /// and return the context if the decoder is.

    pub fn subtitle(self) -> Result<Subtitle, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.subtitle())
        } else {
            Err(Error::DecoderNotFound)
        }
    }
    /// Set the error flags of concealment.
    pub fn conceal(&mut self, value: Conceal) {
        unsafe {
            (*self.as_mut_ptr()).error_concealment = value.bits();
        }
    }
    /// Set the error recognition.
    pub fn check(&mut self, value: Check) {
        unsafe {
            (*self.as_mut_ptr()).err_recognition = value.bits();
        }
    }
    /// Set the skip loop filter for selected frames.
    pub fn skip_loop_filter(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_loop_filter = value.into();
        }
    }
    /// Set the skip IDCT for selected frames.
    pub fn skip_idct(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_idct = value.into();
        }
    }
    /// Set the frames that will be skipped in decoding.
    pub fn skip_frame(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_frame = value.into();
        }
    }
    /// Get the time stamp unit(in seconds) of frames.
    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }
}

impl Deref for Decoder {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Decoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Decoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
