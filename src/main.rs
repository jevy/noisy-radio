use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    sound::{static_sound::{StaticSoundData, StaticSoundSettings}},
};

fn main() {
    let mut manager: AudioManager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
    let sound_data = StaticSoundData::from_file("./src/nature.mp3", StaticSoundSettings::new());
    match sound_data {
        Ok(it) => manager.play(it),
        Err(_) => todo!(),
    };
}

