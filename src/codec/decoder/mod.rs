//! Decoder module.
//!
//! # About
//! This module is for decoding an audio/video file.  
//! Use [util::Frame] to receive the decoded context.  
//! For more infomation about decoding,checkout the examples.  
//!
//! [util::Frame]: crate::Frame
//! # Example
//! 
//! ## Audio Decoding Example (MPEG-3)
//! ```rust
//! extern crate ffmpeg_next as ffmpeg;
//! use ffmpeg::{codec::*, filter, format, frame, media, util};
//! fn main() {
//!     // Initial ffmpeg service.
//!     ffmpeg::init().unwrap();
//!     // Read the file.
//!     let mut input = format::input(&"./examples/test.mp3").unwrap();
//!     // Get the stream of audio.
//!     let input_stream = input.streams().best(media::Type::Audio).unwrap();
//!     // Get the audio stream index.
//!     let audio_stream_index = input_stream.index();
//!     // Find the decoder of input context.
//!     let mut decoder = input_stream.codec().decoder().audio().unwrap();
//!
//!     // the frame index.
//!     let mut frame_index = 0;
//!
//!     // The main function of decoding. In this example we will convert the sample into packed format 
//!     // and print out all the data.
//!     let mut receive_and_process_decoded_frames =
//!             |decoder: &mut ffmpeg::decoder::Audio| -> Result<(), ffmpeg::Error> {
//!                 // the context for store decoded context.
//!                 let mut decoded = ffmpeg::util::frame::Audio::empty();
//!                 // call the decoder to check out the decoded frame.
//!                 while decoder.receive_frame(&mut decoded).is_ok() {
//!                     // take the sample format into packed.
//!                     let packed = decoded.format().packed();
//!                     // now set the new format of sample.
//!                     decoded.set_format(packed);
//!                     println!("{{{:?} \n {:?} \n {:?} }}",decoded.metadata(),decoded,decoded.data(audio_stream_index));
//!                     // move to next frame, like iter.next().
//!                     frame_index += 1;
//!                 }
//!                 Ok(())
//!             };
//!
//!     // Begin of decoding.
//!     for (stream,packet) in input.packets(){
//!         if stream.index() == audio_stream_index {
//!             decoder.send_packet(&packet).unwrap();
//!             receive_and_process_decoded_frames(&mut decoder).unwrap();
//!         }
//!     };
//!     // When we comes to the end of decoding, we need to send eof to decoder to announce it.
//!     decoder.send_eof().unwrap();
//!     receive_and_process_decoded_frames(&mut decoder).unwrap();
//! }
//! ```
/// Decoder
pub mod decoder;
pub use self::decoder::Decoder;
/// Video decoder context
pub mod video;
pub use self::video::Video;
/// Audio decoder context
pub mod audio;
pub use self::audio::Audio;
/// Subtitle decoder context
pub mod subtitle;
pub use self::subtitle::Subtitle;
/// Flags of slice context
pub mod slice;
/// Error flags of concealment
pub mod conceal;
pub use self::conceal::Conceal;
/// Flags of verify context
pub mod check;
pub use self::check::Check;
/// The context of decoder
pub mod opened;
pub use self::opened::Opened;

use std::ffi::CString;

use codec::Context;
use codec::Id;
use ffi::*;
use Codec;

/// Init a new `Decoder` context.
pub fn new() -> Decoder {
    Context::new().decoder()
}

/// Find a codec by id, return `Some(Codec)`
/// or `None` if there is no matches.
pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let ptr = avcodec_find_decoder(id.into());

        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}
/// Find a codec by name, return `Some(Codec)`
/// or `None` if there is no matches.
pub fn find_by_name(name: &str) -> Option<Codec> {
    unsafe {
        let name = CString::new(name).unwrap();
        let ptr = avcodec_find_decoder_by_name(name.as_ptr());

        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}
