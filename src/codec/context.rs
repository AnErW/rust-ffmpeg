use std::ptr;
use std::rc::Rc;

use super::decoder::Decoder;
use super::encoder::Encoder;
use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use ffi::*;
use libc::c_int;
use media;
use {Codec, Error};

/// The codec context.
pub struct Context {
    ptr: *mut AVCodecContext,
    owner: Option<Rc<dyn Drop>>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut AVCodecContext, owner: Option<Rc<dyn Drop>>) -> Self {
        Context { ptr, owner }
    }
    
    pub unsafe fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr
    }
}

impl Context {
    /// Setup a new codec context.
    pub fn new() -> Self {
        unsafe {
            Context {
                ptr: avcodec_alloc_context3(ptr::null()),
                owner: None,
            }
        }
    }
    /// Take the codec context into a decoder.
    pub fn decoder(self) -> Decoder {
        Decoder(self)
    }
    /// Take the codec context into a encoder.
    pub fn encoder(self) -> Encoder {
        Encoder(self)
    }
    /// Wrap with `Codec` if the `codec` field is not null,
    /// and return `Some(Codec)`, or return `None`.
    pub fn codec(&self) -> Option<Codec> {
        unsafe {
            if (*self.as_ptr()).codec.is_null() {
                None
            } else {
                Some(Codec::wrap((*self.as_ptr()).codec as *mut _))
            }
        }
    }
    /// Get the codec type.
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }
    /// Set the AV_CODEC_FLAG_*.
    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            (*self.as_mut_ptr()).flags = value.bits() as c_int;
        }
    }
    /// Get the id of codec.
    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }
    /// Set the standard(e.g.: MPEG-4) which the codec will be strictly
    /// following.
    pub fn compliance(&mut self, value: Compliance) {
        unsafe {
            (*self.as_mut_ptr()).strict_std_compliance = value.into();
        }
    }
    /// Set the debug flags.
    ///
    /// To checkout more debug flags, see: [Debug]
    ///
    /// [Debug]: crate::codec::debug::Debug
    pub fn debug(&mut self, value: Debug) {
        unsafe {
            (*self.as_mut_ptr()).debug = value.bits();
        }
    }
    /// Set the mutitreading config which is used in 
    /// mutithreading method.
    pub fn set_threading(&mut self, config: threading::Config) {
        unsafe {
            (*self.as_mut_ptr()).thread_type = config.kind.into();
            (*self.as_mut_ptr()).thread_count = config.count as c_int;
            (*self.as_mut_ptr()).thread_safe_callbacks = if config.safe { 1 } else { 0 };
        }
    }

    /// Get the current mutithreading config.
    pub fn threading(&self) -> threading::Config {
        unsafe {
            threading::Config {
                kind: threading::Type::from((*self.as_ptr()).active_thread_type),
                count: (*self.as_ptr()).thread_count as usize,
                safe: (*self.as_ptr()).thread_safe_callbacks != 0,
            }
        }
    }
    /// Set the parameters of codec.
    pub fn set_parameters<P: Into<Parameters>>(&mut self, parameters: P) -> Result<(), Error> {
        let parameters = parameters.into();

        unsafe {
            match avcodec_parameters_to_context(self.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_free_context(&mut self.as_mut_ptr());
            }
        }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        let mut ctx = Context::new();
        ctx.clone_from(self);

        ctx
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            avcodec_copy_context(self.as_mut_ptr(), source.as_ptr());
        }
    }
}
