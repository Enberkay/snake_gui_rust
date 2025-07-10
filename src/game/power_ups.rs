use ::rand::Rng;
use ::rand::thread_rng;
use std::collections::VecDeque;
use crate::game::game_state::{Position, PowerUp, PowerUpType};
use crate::utils::{GRID_WIDTH, GRID_HEIGHT};

pub struct PowerUpManager {
    pub power_ups: Vec<PowerUp>,
    pub active_power_ups: Vec<(PowerUpType, u32)>,
    pub speed_multiplier: f32,
    pub ghost_mode: bool,
}

impl PowerUpManager {
    pub fn new() -> Self {
        PowerUpManager {
            power_ups: Vec::new(),
            active_power_ups: Vec::new(),
            speed_multiplier: 1.0,
            ghost_mode: false,
        }
    }

    pub fn reset(&mut self) {
        self.power_ups.clear();
        self.active_power_ups.clear();
        self.speed_multiplier = 1.0;
        self.ghost_mode = false;
    }

    pub fn random_power_up(snake: &VecDeque<Position>, food: &Position) -> PowerUp {
        let mut rng = thread_rng();
        let power_types = [PowerUpType::SpeedBoost, PowerUpType::Shrink, PowerUpType::GhostMode];
        let power_type = power_types[rng.gen_range(0..power_types.len())];
        
        loop {
            let pos = Position {
                x: rng.gen_range(0..GRID_WIDTH),
                y: rng.gen_range(0..GRID_HEIGHT),
            };
            if !snake.contains(&pos) && pos != *food {
                return PowerUp {
                    position: pos,
                    power_type,
                    duration: 300, // 5 seconds at 60 FPS
                };
            }
        }
    }

    pub fn update(&mut self, snake: &VecDeque<Position>, food: &Position) {
        // อัปเดต active power-ups
        self.active_power_ups.retain_mut(|(power_type, duration)| {
            if *duration > 0 {
                *duration -= 1;
            }
            if *duration == 0 {
                // Power-up หมดเวลา
                match power_type {
                    PowerUpType::SpeedBoost => self.speed_multiplier = 1.0,
                    PowerUpType::GhostMode => self.ghost_mode = false,
                    PowerUpType::Shrink => {
                        // Shrink effect จะหายไปอัตโนมัติ
                    }
                }
                false
            } else {
                true
            }
        });

        // สร้าง Power-up ใหม่ (โอกาส 1% ต่อเฟรม)
        if self.power_ups.is_empty() && thread_rng().gen::<f32>() < 0.01 {
            self.power_ups.push(Self::random_power_up(snake, food));
        }
    }

    pub fn check_collision(&mut self, position: &Position) -> Option<PowerUpType> {
        if let Some(power_up_index) = self.power_ups.iter().position(|p| p.position == *position) {
            let power_up = self.power_ups.remove(power_up_index);
            match power_up.power_type {
                PowerUpType::SpeedBoost => {
                    self.speed_multiplier = 2.0;
                    self.active_power_ups.push((PowerUpType::SpeedBoost, power_up.duration));
                }
                PowerUpType::Shrink => {
                    // Shrink effect จะถูกจัดการใน Snake
                }
                PowerUpType::GhostMode => {
                    self.ghost_mode = true;
                    self.active_power_ups.push((PowerUpType::GhostMode, power_up.duration));
                }
            }
            Some(power_up.power_type)
        } else {
            None
        }
    }
} 