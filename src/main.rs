use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

// Sound system
struct SoundManager {
    sound_enabled: bool,
}

const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 30;

// ใช้ฟังก์ชันเพื่อรับขนาดหน้าจอปัจจุบัน
fn get_cell_size() -> f32 {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let cell_w = screen_w / GRID_WIDTH as f32;
    let cell_h = screen_h / GRID_HEIGHT as f32;
    cell_w.min(cell_h) // ใช้ขนาดที่เล็กกว่าเพื่อรักษาอัตราส่วน
}

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
    Paused, // เพิ่มสถานะ Pause
    GameOver,
}

#[derive(PartialEq)]
enum GameMode {
    Normal,
    Obstacle,
}

#[derive(Copy, Clone, PartialEq)]
enum PowerUpType {
    SpeedBoost,
    Shrink,
    GhostMode,
}

#[derive(Copy, Clone)]
struct PowerUp {
    position: Position,
    power_type: PowerUpType,
    duration: u32, // frames
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

    fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
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
    sound_button: Button,
    mode_button: Button,
    high_score: usize, // เพิ่มฟิลด์ high_score
    sound_manager: SoundManager, // เพิ่ม sound manager
    game_mode: GameMode,
    power_ups: Vec<PowerUp>,
    active_power_ups: Vec<(PowerUpType, u32)>,
    speed_multiplier: f32,
    ghost_mode: bool,
}

impl SnakeGame {
    fn load_high_score() -> usize {
        if let Ok(mut file) = File::open("highscore.txt") {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(score) = contents.trim().parse::<usize>() {
                    return score;
                }
            }
        }
        0
    }

    fn save_high_score(&self) {
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open("highscore.txt") {
            let _ = write!(file, "{}", self.high_score);
        }
    }

    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        let food = Self::random_food(&snake);

        let start_button = Button::new(
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 - 30.0,
            200.0,
            50.0,
            "Start".to_string(),
        );

        let exit_button = Button::new(
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + 40.0,
            200.0,
            50.0,
            "Exit".to_string(),
        );

        let sound_button = Button::new(
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + 110.0,
            200.0,
            50.0,
            "Sound: ON".to_string(),
        );
        let mode_button = Button::new(
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + 180.0,
            200.0,
            50.0,
            "Mode: Normal".to_string(),
        );

        let high_score = Self::load_high_score();
        let sound_manager = SoundManager {
            sound_enabled: true,
        };

        SnakeGame {
            snake,
            dir: Direction::Right,
            food,
            game_over: false,
            frame_counter: 0,
            state: GameState::Menu,
            start_button,
            exit_button,
            sound_button,
            mode_button,
            high_score,
            sound_manager,
            game_mode: GameMode::Normal,
            power_ups: Vec::new(),
            active_power_ups: Vec::new(),
            speed_multiplier: 1.0,
            ghost_mode: false,
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

        // ก่อนรีเซ็ต ถ้าคะแนนมากกว่า high_score ให้บันทึก
        let score = self.snake.len() - 1;
        if score > self.high_score {
            self.high_score = score;
            self.save_high_score();
        }

        self.snake = snake;
        self.dir = Direction::Right;
        self.food = Self::random_food(&self.snake);
        self.game_over = false;
        self.frame_counter = 0;
    }

    fn update_button_positions(&mut self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        self.start_button.update_position(
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 - 30.0,
        );
        
        self.exit_button.update_position(
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 + 40.0,
        );
        
        self.sound_button.update_position(
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 + 110.0,
        );
        self.mode_button.update_position(
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 + 180.0,
        );
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
            // Play crash sound
            if self.sound_manager.sound_enabled {
                println!("💥 Crash sound played!");
            }
            self.game_over = true;
            self.state = GameState::GameOver;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.food = Self::random_food(&self.snake);
            // Play eat sound
            if self.sound_manager.sound_enabled {
                // TODO: Add actual sound playing
                println!("🎵 Eat sound played!");
            }
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
        
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // วาดกรอบสนาม
        draw_rectangle_lines(0.0, 0.0, screen_w, screen_h, 2.0, WHITE);
        
        // วาดชื่อเกม
        draw_text(
            "SNAKE GAME",
            screen_w / 2.0 - 120.0,
            screen_h / 2.0 - 100.0,
            40.0,
            GREEN,
        );
        
        // วาดปุ่ม
        self.start_button.draw();
        self.exit_button.draw();
        self.sound_button.draw();
        self.mode_button.draw();
        
        // แสดง High Score
        draw_text(
            &format!("High Score: {}", self.high_score),
            screen_w / 2.0 - 90.0,
            screen_h / 2.0 - 60.0,
            30.0,
            YELLOW,
        );
        // แสดง FPS
        draw_text(
            &format!("FPS: {}", get_fps()),
            screen_w - 100.0,
            20.0,
            20.0,
            YELLOW,
        );
    }

    fn draw_game(&self) {
        clear_background(BLACK);

        let screen_w = screen_width();
        let screen_h = screen_height();
        let cell_size = get_cell_size();

        // คำนวณตำแหน่งเริ่มต้นเพื่อให้เกมอยู่กลางหน้าจอ
        let game_width = GRID_WIDTH as f32 * cell_size;
        let game_height = GRID_HEIGHT as f32 * cell_size;
        let offset_x = (screen_w - game_width) / 2.0;
        let offset_y = (screen_h - game_height) / 2.0;

        // วาดกรอบสนาม
        draw_rectangle_lines(offset_x, offset_y, game_width, game_height, 2.0, WHITE);

        // วาดอาหาร
        draw_rectangle(
            offset_x + self.food.x as f32 * cell_size,
            offset_y + self.food.y as f32 * cell_size,
            cell_size,
            cell_size,
            RED,
        );

        // วาดงู
        for (i, seg) in self.snake.iter().enumerate() {
            let color = if i == 0 { GREEN } else { DARKGREEN };
            draw_rectangle(
                offset_x + seg.x as f32 * cell_size,
                offset_y + seg.y as f32 * cell_size,
                cell_size,
                cell_size,
                color,
            );
        }

        // แสดงคะแนน
        draw_text(
            &format!("Score: {}", self.snake.len() - 1),
            10.0,
            screen_h - 10.0,
            20.0,
            WHITE,
        );

        // แสดง FPS (ด้านบนขวา)
        draw_text(
            &format!("FPS: {}", get_fps()),
            screen_w - 100.0,
            20.0,
            20.0,
            YELLOW,
        );
    }

    fn draw_game_over(&self) {
        self.draw_game();
        
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // แสดง Game Over
        draw_text(
            "GAME OVER",
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 - 50.0,
            40.0,
            WHITE,
        );
        // แสดงคะแนนสูงสุด
        draw_text(
            &format!("High Score: {}", self.high_score),
            screen_w / 2.0 - 90.0,
            screen_h / 2.0 - 10.0,
            30.0,
            YELLOW,
        );
        draw_text(
            "Press ENTER to Restart",
            screen_w / 2.0 - 130.0,
            screen_h / 2.0,
            25.0,
            GRAY,
        );
        draw_text(
            "Press ESC for Menu",
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 + 30.0,
            25.0,
            GRAY,
        );
    }

    fn draw(&self) {
        match self.state {
            GameState::Menu => self.draw_menu(),
            GameState::Playing => self.draw_game(),
            GameState::Paused => {
                self.draw_game();
                // วาดข้อความ Paused ทับ
                let screen_w = screen_width();
                let screen_h = screen_height();
                draw_rectangle(
                    screen_w / 2.0 - 120.0,
                    screen_h / 2.0 - 60.0,
                    240.0,
                    80.0,
                    Color::new(0.0, 0.0, 0.0, 0.7),
                );
                draw_text(
                    "PAUSED",
                    screen_w / 2.0 - 70.0,
                    screen_h / 2.0,
                    50.0,
                    YELLOW,
                );
            },
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
                } else if self.sound_button.is_clicked() {
                    self.sound_manager.sound_enabled = !self.sound_manager.sound_enabled;
                    self.sound_button.text = if self.sound_manager.sound_enabled {
                        "Sound: ON".to_string()
                    } else {
                        "Sound: OFF".to_string()
                    };
                }
                if self.mode_button.is_clicked() {
                    self.game_mode = if self.game_mode == GameMode::Normal {
                        GameMode::Obstacle
                    } else {
                        GameMode::Normal
                    };
                    self.mode_button.text = if self.game_mode == GameMode::Normal {
                        "Mode: Normal".to_string()
                    } else {
                        "Mode: Obstacle".to_string()
                    };
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
                } else if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Paused;
                }
            },
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
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
    request_new_screen_size(800.0, 600.0);
    next_frame().await;

    let mut game = SnakeGame::new();

    loop {
        // อัปเดตตำแหน่งปุ่มเมื่อหน้าจอเปลี่ยนขนาด
        game.update_button_positions();
        
        game.handle_input();

        if game.state == GameState::Playing {
            game.update();
        }

        game.draw();
        next_frame().await;
    }
}
