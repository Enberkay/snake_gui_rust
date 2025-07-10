use std::collections::VecDeque;
use crate::game::game_state::{Position, Direction};
use crate::utils::{GRID_WIDTH, GRID_HEIGHT};

pub struct Snake {
    pub body: VecDeque<Position>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back(Position {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        Snake {
            body,
            direction: Direction::Right,
        }
    }

    pub fn reset(&mut self) {
        let mut body = VecDeque::new();
        body.push_back(Position {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        self.body = body;
        self.direction = Direction::Right;
    }

    pub fn change_direction(&mut self, new_dir: Direction) {
        if (self.direction == Direction::Up && new_dir != Direction::Down)
            || (self.direction == Direction::Down && new_dir != Direction::Up)
            || (self.direction == Direction::Left && new_dir != Direction::Right)
            || (self.direction == Direction::Right && new_dir != Direction::Left)
        {
            self.direction = new_dir;
        }
    }

    pub fn move_snake(&mut self) -> Position {
        let mut new_head = *self.body.front().unwrap();
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // เดินทะลุขอบ (wrap around)
        if new_head.x < 0 {
            new_head.x = GRID_WIDTH - 1;
        } else if new_head.x >= GRID_WIDTH {
            new_head.x = 0;
        }

        if new_head.y < 0 {
            new_head.y = GRID_HEIGHT - 1;
        } else if new_head.y >= GRID_HEIGHT {
            new_head.y = 0;
        }

        new_head
    }

    pub fn grow(&mut self, new_head: Position) {
        self.body.push_front(new_head);
    }

    pub fn shrink(&mut self) {
        if self.body.len() > 1 {
            self.body.pop_back();
        }
    }

    pub fn contains(&self, position: &Position) -> bool {
        self.body.contains(position)
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }
} 