
use std::fs::File;
use std::io::BufReader;
use std::{io, thread, time::Duration};

use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use rodio::source::Source;
use rodio::dynamic_mixer::{self, DynamicMixer};

use glob::glob;

pub struct Radio {
    pub current_freq: i8,
    channels: Vec<RadioChannel>,
    stream_handle: OutputStreamHandle,
    static_sink: Sink,
}

impl Radio {
    const CHANNEL_BANDWIDTH: f32 = 15.0;

    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        
        let static_sink = rodio::Sink::try_new(&stream_handle).unwrap();
        Radio {
            current_freq: 50,
            channels: Vec::<RadioChannel>::new(),
            static_sink: static_sink,
            stream_handle: stream_handle,
        }
    }

    pub fn tune_up(&mut self) {
        self.current_freq += (1 as i8);
        self.adjust_volumes();
    }

    pub fn tune_down(&mut self) {
        self.current_freq -= (1 as i8);
        self.adjust_volumes();
    }

    pub fn add_radio_channel(&mut self, filename: String, center_freq: i8) {
        let rc = RadioChannel::new(filename, center_freq, &self.stream_handle);
        self.channels.push(rc);
        self.adjust_volumes();
    }

    pub fn add_radio_channel_from_directory(&mut self, directory: String, center_freq: i8) {
        let rc = RadioChannel::new_from_directory(directory, center_freq, &self.stream_handle);
        self.channels.push(rc);
        self.adjust_volumes();
    }

    pub fn add_static(&mut self, filename: String) {
        let file = BufReader::new(File::open(filename).unwrap());
        let file_source = Decoder::new_looped(file).unwrap();
        &self.static_sink.append(file_source);
    }

    //
    fn adjust_volumes(&mut self) {
        for channel in &mut self.channels {
            if channel.center_freq != 0 {
                let delta_from_center = (self.current_freq - channel.center_freq).abs() as f32;
                if delta_from_center > Radio::CHANNEL_BANDWIDTH {
                    channel.sink.set_volume(0.0);
                } else {
                    channel.sink.set_volume(((Radio::CHANNEL_BANDWIDTH - delta_from_center) / Radio::CHANNEL_BANDWIDTH) as f32);
                    &self.static_sink.set_volume(1.0-((Radio::CHANNEL_BANDWIDTH - delta_from_center) / Radio::CHANNEL_BANDWIDTH) as f32);
                }
            } else {
            };
        }

    }
}

struct RadioChannel {
    center_freq: i8, // cf of 0 means everywhere
    sink: rodio::Sink,
}

impl RadioChannel {
    fn new(filename: String, cf: i8, stream_handle: &rodio::OutputStreamHandle) -> Self {

        let rc = RadioChannel {
            center_freq: cf,
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
        };

        // let file = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
        // let file_source = rodio::Decoder::new(file).unwrap();
        // rc.sink.append(file_source);
        // return rc;

        let file = BufReader::new(File::open(filename).unwrap());
        let file_source = Decoder::new(file).unwrap();
        rc.sink.append(file_source);
        rc
    }

    fn new_from_directory(pattern: String, cf: i8, stream_handle: &rodio::OutputStreamHandle) -> Self {

        let rc = RadioChannel {
            center_freq: cf,
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
        };

        // let file = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
        // let file_source = rodio::Decoder::new(file).unwrap();
        // rc.sink.append(file_source);
        // return rc;

        for entry in glob(&pattern).unwrap() {
            if let Ok(path) = entry {
                let file_reader = BufReader::new(File::open(path).unwrap());
                let file_source = Decoder::new(file_reader).unwrap();
                rc.sink.append(file_source);
            }
        };
        rc
    }


}