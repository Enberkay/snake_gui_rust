use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;
use std::collections::VecDeque;

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

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    color: Color,
    hover_color: Color,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: String) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text,
            color: GRAY,
            hover_color: LIGHTGRAY,
        }
    }

    fn draw(&self) {
        let mouse_pos = mouse_position();
        let is_hovered = mouse_pos.0 >= self.x && mouse_pos.0 <= self.x + self.width
            && mouse_pos.1 >= self.y && mouse_pos.1 <= self.y + self.height;
        
        let color = if is_hovered { self.hover_color } else { self.color };
        
        draw_rectangle(self.x, self.y, self.width, self.height, color);
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, WHITE);
        
        let text_size = measure_text(&self.text, None, 20, 1.0);
        draw_text(
            &self.text,
            self.x + (self.width - text_size.width) / 2.0,
            self.y + (self.height + text_size.height) / 2.0,
            20.0,
            BLACK,
        );
    }

    fn is_clicked(&self) -> bool {
        let mouse_pos = mouse_position();
        let is_hovered = mouse_pos.0 >= self.x && mouse_pos.0 <= self.x + self.width
            && mouse_pos.1 >= self.y && mouse_pos.1 <= self.y + self.height;
        
        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }
}

struct SnakeGame {
    snake: VecDeque<Position>,
    dir: Direction,
    food: Position,
    game_over: bool,
    frame_counter: u8,
    state: GameState,
    start_button: Button,
    exit_button: Button,
}

impl SnakeGame {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        let food = Self::random_food(&snake);

        let start_button = Button::new(
            SCREEN_WIDTH / 2.0 - 100.0,
            SCREEN_HEIGHT / 2.0 - 30.0,
            200.0,
            50.0,
            "Start".to_string(),
        );

        let exit_button = Button::new(
            SCREEN_WIDTH / 2.0 - 100.0,
            SCREEN_HEIGHT / 2.0 + 40.0,
            200.0,
            50.0,
            "Exit".to_string(),
        );

        SnakeGame {
            snake,
            dir: Direction::Right,
            food,
            game_over: false,
            frame_counter: 0,
            state: GameState::Menu,
            start_button,
            exit_button,
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

    fn reset_game(&mut self) {
        let mut snake = VecDeque::new();
        snake.push_back(Position {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        self.snake = snake;
        self.dir = Direction::Right;
        self.food = Self::random_food(&self.snake);
        self.game_over = false;
        self.frame_counter = 0;
    }

    fn update(&mut self) {
        self.frame_counter += 1;

        // ควบคุมความเร็วงูแยกจาก FPS (งูจะเดินทุก 8 เฟรม)
        if self.frame_counter < 10 {
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

        if self.snake.contains(&new_head) {
            self.game_over = true;
            self.state = GameState::GameOver;
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

    fn draw_menu(&self) {
        clear_background(BLACK);
        
        // วาดกรอบสนาม
        draw_rectangle_lines(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, 2.0, WHITE);
        
        // วาดชื่อเกม
        draw_text(
            "SNAKE GAME",
            SCREEN_WIDTH / 2.0 - 120.0,
            SCREEN_HEIGHT / 2.0 - 100.0,
            40.0,
            GREEN,
        );
        
        // วาดปุ่ม
        self.start_button.draw();
        self.exit_button.draw();
        
        // แสดง FPS
        draw_text(
            &format!("FPS: {}", get_fps()),
            SCREEN_WIDTH - 100.0,
            20.0,
            20.0,
            YELLOW,
        );
    }

    fn draw_game(&self) {
        clear_background(BLACK);

        // วาดกรอบสนาม
        draw_rectangle_lines(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, 2.0, WHITE);

        // วาดอาหาร
        draw_rectangle(
            self.food.x as f32 * CELL_SIZE,
            self.food.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            RED,
        );

        // วาดงู
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

        // แสดงคะแนน
        draw_text(
            &format!("Score: {}", self.snake.len() - 1),
            10.0,
            SCREEN_HEIGHT - 10.0,
            20.0,
            WHITE,
        );

        // แสดง FPS (ด้านบนขวา)
        draw_text(
            &format!("FPS: {}", get_fps()),
            SCREEN_WIDTH - 100.0,
            20.0,
            20.0,
            YELLOW,
        );
    }

    fn draw_game_over(&self) {
        self.draw_game();
        
        // แสดง Game Over
        draw_text(
            "GAME OVER",
            SCREEN_WIDTH / 2.0 - 100.0,
            SCREEN_HEIGHT / 2.0 - 50.0,
            40.0,
            WHITE,
        );
        draw_text(
            "Press ENTER to Restart",
            SCREEN_WIDTH / 2.0 - 130.0,
            SCREEN_HEIGHT / 2.0,
            25.0,
            GRAY,
        );
        draw_text(
            "Press ESC for Menu",
            SCREEN_WIDTH / 2.0 - 100.0,
            SCREEN_HEIGHT / 2.0 + 30.0,
            25.0,
            GRAY,
        );
    }

    fn draw(&self) {
        match self.state {
            GameState::Menu => self.draw_menu(),
            GameState::Playing => self.draw_game(),
            GameState::GameOver => self.draw_game_over(),
        }
    }

    fn handle_input(&mut self) {
        match self.state {
            GameState::Menu => {
                if self.start_button.is_clicked() {
                    self.reset_game();
                    self.state = GameState::Playing;
                } else if self.exit_button.is_clicked() {
                    std::process::exit(0);
                }
            },
            GameState::Playing => {
                if is_key_pressed(KeyCode::Up) {
                    self.change_direction(Direction::Up);
                } else if is_key_pressed(KeyCode::Down) {
                    self.change_direction(Direction::Down);
                } else if is_key_pressed(KeyCode::Left) {
                    self.change_direction(Direction::Left);
                } else if is_key_pressed(KeyCode::Right) {
                    self.change_direction(Direction::Right);
                } else if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            },
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset_game();
                    self.state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            },
        }
    }
}

#[macroquad::main("Snake Game with Menu")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    next_frame().await;

    let mut game = SnakeGame::new();

    loop {
        game.handle_input();

        if game.state == GameState::Playing {
            game.update();
        }

        game.draw();
        next_frame().await;
    }
}
