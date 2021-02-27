use std::ffi::CString;
use std::mem;
use std::ops::{Deref, DerefMut};

use super::common::Context;
use super::destructor;
use ffi::*;
use util::range::Range;
use {format, Codec, Error, Packet, Stream};
/// The input context which is used to receive
/// input stream/file.
pub struct Input {
    ptr: *mut AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Input {}

impl Input {
    pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
        Input {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Input),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Input {
    pub fn format(&self) -> format::Input {
        unsafe { format::Input::wrap((*self.as_ptr()).iformat) }
    }
    /// Get the video codec for input stream/file,
    /// return `None` if it's not a video stream/file
    /// or cannot find the codec for this format, otherwise
    /// `Some(Code)` will be returned if codec matches.
    pub fn video_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = av_format_get_video_codec(self.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    /// Get the audio codec for input stream/file,
    /// return `None` if it's not a audio stream/file
    /// or cannot find the codec for this format, otherwise
    /// `Some(Code)` will be returned if codec matches.
    pub fn audio_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = av_format_get_audio_codec(self.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }
    /// Similar to `audio_codec()` and `video_codec()`, but
    /// this method is used to get subtitle codec.
    pub fn subtitle_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = av_format_get_subtitle_codec(self.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }
    /// Like `audio_codec()` and `video_codec()`, but 
    /// this method is used to get data codec.
    pub fn data_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = av_format_get_data_codec(self.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }
    /// Get the probe score of input context.
    /// It's usually for ABI compatibility.
    pub fn probe_score(&self) -> i32 {
        unsafe { av_format_get_probe_score(self.as_ptr()) }
    }
    /// Get all packets in input context.
    pub fn packets(&mut self) -> PacketIter {
        PacketIter::new(self)
    }
    /// Pause the network-basd stream.
    ///
    /// To resume it, see: [play()].
    ///
    /// [play()]: self::play
    pub fn pause(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_pause(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
    /// Start playing a network-based stream at 
    /// the current position.
    /// 
    /// To stop the stream, see: [pause()].
    ///
    /// [pause()]: self::pause
    pub fn play(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_play(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
    /// Seek to timestamp ts.
    pub fn seek<R: Range<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
        unsafe {
            match avformat_seek_file(
                self.as_mut_ptr(),
                -1,
                range.start().cloned().unwrap_or(i64::min_value()),
                ts,
                range.end().cloned().unwrap_or(i64::max_value()),
                0,
            ) {
                s if s >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Deref for Input {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = (Stream<'a>, Packet);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        loop {
            match packet.read(self.context) {
                Ok(..) => unsafe {
                    return Some((
                        Stream::wrap(mem::transmute_copy(&self.context), packet.stream()),
                        packet,
                    ));
                },

                Err(Error::Eof) => return None,

                Err(..) => (),
            }
        }
    }
}

/// Dump out the detail infomation of input format, basicially
/// including duration, bitrate, streams, metadata, etc.
/// # Parameters
/// `ctx`: the input context to analyze  
/// `index`: the index of stream to dump infomation about  
/// `url`: the path to export/print the detail infomation
///
/// To dump a output context, see: [output::dump]
///
/// [output::dump]: super::output::dump
pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            0,
        );
    }
}
