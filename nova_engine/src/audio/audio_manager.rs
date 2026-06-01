use kira::{
    manager::{AudioManager as KiraManager, AudioManagerSettings, backend::DefaultBackend},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

pub struct AudioManager {
    manager: KiraManager<DefaultBackend>,
}

impl AudioManager {
    pub fn new() -> Self {
        let manager = KiraManager::new(AudioManagerSettings::default())
            .expect("Failed to initialize audio manager");
        Self { manager }
    }

    pub fn play(&mut self, path: &str) {
        let sound = StaticSoundData::from_file(path, StaticSoundSettings::default())
            .expect("Failed to load sound");
        self.manager.play(sound).expect("Failed to play sound");
    }

    pub fn play_looped(&mut self, path: &str) {
        use kira::sound::static_sound::StaticSoundSettings;
        use kira::tween::Tween;

        let settings = StaticSoundSettings::new().loop_region(0.0..);
        let sound = StaticSoundData::from_file(path, settings)
            .expect("Failed to load sound");
        self.manager.play(sound).expect("Failed to play looped sound");
    }
}
