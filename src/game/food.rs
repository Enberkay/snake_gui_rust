use ::rand::Rng;
use ::rand::thread_rng;
use std::collections::VecDeque;
use crate::game::game_state::Position;
use crate::utils::{GRID_WIDTH, GRID_HEIGHT};

pub struct Food {
    pub position: Position,
}

impl Food {
    pub fn new() -> Self {
        Food {
            position: Self::random_position(&VecDeque::new()),
        }
    }

    pub fn random_position(snake: &VecDeque<Position>) -> Position {
        let mut rng = thread_rng();
        loop {
            let pos = Position {
                x: rng.gen_range(0..GRID_WIDTH),
                y: rng.gen_range(0..GRID_HEIGHT),
            };
            if !snake.contains(&pos) {
                return pos;
            }
        }
    }

    pub fn respawn(&mut self, snake: &VecDeque<Position>) {
        self.position = Self::random_position(snake);
    }
} 