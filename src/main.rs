use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;
use std::collections::VecDeque;

// ==== CONFIG =====
const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 30;
const CELL_SIZE: f32 = 20.0;

const SCREEN_WIDTH: f32 = GRID_WIDTH as f32 * CELL_SIZE;
const SCREEN_HEIGHT: f32 = GRID_HEIGHT as f32 * CELL_SIZE;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct SnakeGame {
    snake: VecDeque<Position>,
    dir: Direction,
    food: Position,
    game_over: bool,
    frame_counter: u8,
}

impl SnakeGame {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        let food = Self::random_food(&snake);

        SnakeGame {
            snake,
            dir: Direction::Right,
            food,
            game_over: false,
            frame_counter: 0,
        }
    }

    fn random_food(snake: &VecDeque<Position>) -> Position {
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

    fn update(&mut self) {
        self.frame_counter += 1;
        if self.frame_counter < 8 {
            return;
        }
        self.frame_counter = 0;

        let mut new_head = *self.snake.front().unwrap();
        match self.dir {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // 👇 Wrap Around Logic
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

        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.food = Self::random_food(&self.snake);
        } else {
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_dir: Direction) {
        if (self.dir == Direction::Up && new_dir != Direction::Down)
            || (self.dir == Direction::Down && new_dir != Direction::Up)
            || (self.dir == Direction::Left && new_dir != Direction::Right)
            || (self.dir == Direction::Right && new_dir != Direction::Left)
        {
            self.dir = new_dir;
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        // 🔲 วาดกรอบ
        draw_rectangle_lines(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, 2.0, WHITE);

        // 🍎 วาดอาหาร
        draw_rectangle(
            self.food.x as f32 * CELL_SIZE,
            self.food.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            RED,
        );

        // 🐍 วาดงู
        for (i, seg) in self.snake.iter().enumerate() {
            let color = if i == 0 { GREEN } else { DARKGREEN };
            draw_rectangle(
                seg.x as f32 * CELL_SIZE,
                seg.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }

        // 🛑 ถ้าแพ้
        if self.game_over {
            draw_text(
                "GAME OVER",
                SCREEN_WIDTH / 2.0 - 100.0,
                SCREEN_HEIGHT / 2.0,
                40.0,
                WHITE,
            );
            draw_text(
                "Press ENTER to Restart",
                SCREEN_WIDTH / 2.0 - 130.0,
                SCREEN_HEIGHT / 2.0 + 40.0,
                25.0,
                GRAY,
            );
        }

        // 🧮 คะแนน
        draw_text(
            &format!("Score: {}", self.snake.len() - 1),
            10.0,
            SCREEN_HEIGHT - 10.0,
            20.0,
            WHITE,
        );
    }
}

#[macroquad::main("Snake Game")]
async fn main() {
    // เปลี่ยนขนาดหน้าจอให้เท่ากับขนาดสนาม
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    next_frame().await; // รอ frame ก่อนใช้ขนาดใหม่

    let mut game = SnakeGame::new();

    loop {
        if is_key_pressed(KeyCode::Up) {
            game.change_direction(Direction::Up);
        } else if is_key_pressed(KeyCode::Down) {
            game.change_direction(Direction::Down);
        } else if is_key_pressed(KeyCode::Left) {
            game.change_direction(Direction::Left);
        } else if is_key_pressed(KeyCode::Right) {
            game.change_direction(Direction::Right);
        }

        if game.game_over {
            if is_key_pressed(KeyCode::Enter) {
                game = SnakeGame::new();
            }
        } else {
            game.update();
        }

        game.draw();
        next_frame().await;
    }
}
