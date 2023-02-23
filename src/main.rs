use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use rodio::dynamic_mixer::{self, DynamicMixer};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let nature_file = BufReader::new(File::open("src/nature.mp3").unwrap());
    let static_file = BufReader::new(File::open("src/radio-static.mp3").unwrap());
    // Decode that sound file into a source
    let nature_source = Decoder::new(nature_file).unwrap();
    let static_source = Decoder::new(static_file).unwrap();
    
    let (tx, mut rx) = dynamic_mixer::mixer(1, 48000);

    tx.add(static_source);
    tx.add(nature_source);
    sink.append(rx);

    // stream_handle.play_raw(nature_source.convert_samples());

    // TODO: Where is the sink??


    // static_sink.append(static_sound);
    // stream_handle.play_raw(source.convert_samples());

    // sink.sleep_until_end();
    // static_sink.sleep_until_end();
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sink.sleep_until_end();
    // std::thread::sleep(std::time::Duration::from_secs(5));
}