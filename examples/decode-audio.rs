extern crate ffmpeg_next as ffmpeg;
use ffmpeg::{codec::*, filter, format, frame, media, util};
fn main() {
    ffmpeg::init().unwrap();
    let mut input = format::input(&"./examples/test.mp3").unwrap();
    let input_stream = input.streams().best(media::Type::Audio).unwrap();
    let audio_stream_index = input_stream.index();
    let mut decoder = input_stream.codec().decoder().audio().unwrap();


    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Audio| -> Result<(), ffmpeg::Error> {
                let mut decoded = ffmpeg::util::frame::Audio::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let packed = decoded.format().packed();
                    decoded.set_format(packed);
                    println!("{{{:?} \n {:?} \n {:?} }}",decoded.metadata(),decoded,decoded.data(0));
                    frame_index += 1;
                }
                
                Ok(())
            };
    for (stream,packet) in input.packets(){
        if stream.index() == audio_stream_index {
            decoder.send_packet(&packet).unwrap();
            receive_and_process_decoded_frames(&mut decoder).unwrap();
        }
    };
    decoder.send_eof().unwrap();
    receive_and_process_decoded_frames(&mut decoder).unwrap();

}
 