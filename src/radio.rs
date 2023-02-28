
use std::fs::File;
use std::io::BufReader;
use std::path;
use std::{io, thread, time::Duration};
use std::collections::VecDeque;

use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use rodio::source::Source;
use rodio::dynamic_mixer::{self, DynamicMixer};

use glob::glob;
use rand::thread_rng;
use rand::seq::SliceRandom;

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

        // Basic idea: We want to start at a random place, in a random song.
        // So 1. Randomize the list of files to pull from. 2. Skip to a random place in the first song

        // https://users.rust-lang.org/t/sorting-glob-result/16461/3
        let mut files_matching_glob: Result<Vec<_>, _> = glob(&pattern).expect("Glob failed").collect();
        
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018
        let mut all_files_in_dir = files_matching_glob.unwrap();
        all_files_in_dir.shuffle(&mut thread_rng());

        for (i, entry) in all_files_in_dir.iter().enumerate() {
            let file_reader = BufReader::new(File::open(entry).unwrap());
            let mut file_source = Decoder::new(file_reader).unwrap();

            if i == 0 {
                // Advance the first song by half
                rc.sink.append(file_source.skip_duration(Duration::new(60, 0)));
            } else {
                rc.sink.append(file_source);
            }

        };
        rc
    }


}