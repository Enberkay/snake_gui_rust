pub struct SoundManager {
    pub sound_enabled: bool,
}

impl SoundManager {
    pub fn new() -> Self {
        SoundManager {
            sound_enabled: true,
        }
    }

    pub fn play_eat_sound(&self) {
        if self.sound_enabled {
            println!("🎵 Eat sound played!");
        }
    }

    pub fn play_crash_sound(&self) {
        if self.sound_enabled {
            println!("💥 Crash sound played!");
        }
    }

    pub fn play_power_up_sound(&self, power_type: &str) {
        if self.sound_enabled {
            println!("{} activated!", power_type);
        }
    }
} 