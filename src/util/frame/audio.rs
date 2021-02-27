use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use super::Frame;
use ffi::*;
use libc::{c_int, c_ulonglong};
use util::format;
use ChannelLayout;
/// The audio frame.
#[derive(PartialEq, Eq)]
pub struct Audio(Frame);

impl Audio {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrame) -> Self {
        Audio(Frame::wrap(ptr))
    }

    #[inline]
    pub unsafe fn alloc(&mut self, format: format::Sample, samples: usize, layout: ChannelLayout) {
        self.set_format(format);
        self.set_samples(samples);
        self.set_channel_layout(layout);

        av_frame_get_buffer(self.as_mut_ptr(), 0);
    }
}

impl Audio {
    #[inline(always)]
    /// Create an empty audio frame.
    pub fn empty() -> Self {
        unsafe { Audio(Frame::empty()) }
    }

    #[inline]
    /// Create an audio frame with sample format, total sample number and layout channels map.
    pub fn new(format: format::Sample, samples: usize, layout: ChannelLayout) -> Self {
        unsafe {
            let mut frame = Audio::empty();
            frame.alloc(format, samples, layout);

            frame
        }
    }

    #[inline]
    /// Get the format of audio frame.
    pub fn format(&self) -> format::Sample {
        unsafe {
            if (*self.as_ptr()).format == -1 {
                format::Sample::None
            } else {
                format::Sample::from(mem::transmute::<_, AVSampleFormat>((*self.as_ptr()).format))
            }
        }
    }

    #[inline]
    /// Set the format of audio frame.
    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).format = mem::transmute::<AVSampleFormat, c_int>(value.into());
        }
    }

    #[inline]
    /// Get the channel layout map.
    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe {
            ChannelLayout::from_bits_truncate(
                av_frame_get_channel_layout(self.as_ptr()) as c_ulonglong
            )
        }
    }

    #[inline]
    /// Set the channel layout map of audio frame.
    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            av_frame_set_channel_layout(self.as_mut_ptr(), value.bits() as i64);
        }
    }

    #[inline]
    /// Get the total amount of channels of audio frame.
    pub fn channels(&self) -> u16 {
        unsafe { av_frame_get_channels(self.as_ptr()) as u16 }
    }

    #[inline]
    /// Set the total amount of channels of audio frame.
    pub fn set_channels(&mut self, value: u16) {
        unsafe {
            av_frame_set_channels(self.as_mut_ptr(), i32::from(value));
        }
    }

    #[inline]
    /// Get the sample rate of audio.
    pub fn rate(&self) -> u32 {
        unsafe { av_frame_get_sample_rate(self.as_ptr()) as u32 }
    }

    #[inline]
    /// Set the sample rate of audio.
    pub fn set_rate(&mut self, value: u32) {
        unsafe {
            av_frame_set_sample_rate(self.as_mut_ptr(), value as c_int);
        }
    }

    #[inline]
    /// Get the total amount of samples.
    pub fn samples(&self) -> usize {
        unsafe { (*self.as_ptr()).nb_samples as usize }
    }

    #[inline]
    /// Set the total amount of samples.
    pub fn set_samples(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).nb_samples = value as c_int;
        }
    }

    #[inline]
    /// Check if the audio frame is planar formate.
    pub fn is_planar(&self) -> bool {
        self.format().is_planar()
    }

    #[inline]
    /// Check if the audio frame is packed format.
    pub fn is_packed(&self) -> bool {
        self.format().is_packed()
    }

    #[inline]
    /// Get the total amount of planes.
    pub fn planes(&self) -> usize {
        unsafe {
            if (*self.as_ptr()).linesize[0] == 0 {
                return 0;
            }
        }

        if self.is_packed() {
            1
        } else {
            self.channels() as usize
        }
    }

    #[inline]
    /// Get the sample in the given format.
    pub fn plane<T: Sample>(&self, index: usize) -> &[T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Sample>::is_valid(self.format(), self.channels()) {
            panic!("unsupported type");
        }

        unsafe { slice::from_raw_parts((*self.as_ptr()).data[index] as *const T, self.samples()) }
    }

    #[inline]
    /// Like [plane()], but the data is mutable.
    ///
    /// [plane()]: self::plane()
    pub fn plane_mut<T: Sample>(&mut self, index: usize) -> &mut [T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Sample>::is_valid(self.format(), self.channels()) {
            panic!("unsupported type");
        }

        unsafe {
            slice::from_raw_parts_mut((*self.as_mut_ptr()).data[index] as *mut T, self.samples())
        }
    }

    #[inline]
    /// Get audio data.
    pub fn data(&self, index: usize) -> &[u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            slice::from_raw_parts(
                (*self.as_ptr()).data[index],
                (*self.as_ptr()).linesize[index] as usize,
            )
        }
    }

    #[inline]
    /// Like [data()], but the data is mutable.
    ///
    /// [data()]: self::data()
    pub fn data_mut(&mut self, index: usize) -> &mut [u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            slice::from_raw_parts_mut(
                (*self.as_mut_ptr()).data[index],
                (*self.as_ptr()).linesize[index] as usize,
            )
        }
    }
}

impl Deref for Audio {
    type Target = Frame;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl ::std::fmt::Debug for Audio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str("ffmpeg::frame::Audio { ")?;
        f.write_str(&format!("format: {:?}, ", self.format()))?;
        f.write_str(&format!("channels: {:?}, ", self.channels()))?;
        f.write_str(&format!("rate: {:?}, ", self.rate()))?;
        f.write_str(&format!("samples: {:?} ", self.samples()))?;
        f.write_str("}")
    }
}

impl Clone for Audio {
    fn clone(&self) -> Self {
        let mut cloned = Audio::new(self.format(), self.samples(), self.channel_layout());
        cloned.clone_from(self);

        cloned
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_frame_copy(self.as_mut_ptr(), source.as_ptr());
            av_frame_copy_props(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl From<Frame> for Audio {
    fn from(frame: Frame) -> Self {
        Audio(frame)
    }
}

pub unsafe trait Sample {
    fn is_valid(format: format::Sample, channels: u16) -> bool;
}

unsafe impl Sample for u8 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::U8(..))
    }
}

unsafe impl Sample for (u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for i16 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::I16(..))
    }
}

unsafe impl Sample for (i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for i32 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::I32(..))
    }
}

unsafe impl Sample for (i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for f32 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::F32(..))
    }
}

unsafe impl Sample for (f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for f64 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::F64(..))
    }
}

unsafe impl Sample for (f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}
