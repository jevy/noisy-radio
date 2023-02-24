
use std::fs::File;
use std::io::BufReader;
use std::{io, thread, time::Duration};

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use rodio::dynamic_mixer::{self, DynamicMixer};

pub struct Radio {
    pub current_freq: i8,
    channels: Vec<RadioChannel>
}

impl Radio {
    pub fn new() -> Self {
        Radio {
            current_freq: 50,
            channels: Vec::new()
        }
    }

    pub fn tune_up(&mut self) {
        self.current_freq += (1 as i8);
    }

    pub fn tune_down(&mut self) {
        self.current_freq -= (1 as i8);
    }

    pub fn add_radio_channel(&mut self, rc: RadioChannel) {
        self.channels.push(rc);
    }
}

pub struct RadioChannel {
    center_freq: i8,
    sink: rodio::Sink,
}

impl RadioChannel {
    pub fn new(filename: String, cf: i8, stream_handle: &rodio::OutputStreamHandle) -> Self {

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

}