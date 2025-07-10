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
    mode_button: Button, // ปุ่มเลือกโหมด
    high_score: usize, // เพิ่มฟิลด์ high_score
    sound_manager: SoundManager, // เพิ่ม sound manager
    power_ups: Vec<PowerUp>, // เพิ่ม power-ups
    active_power_ups: Vec<(PowerUpType, u32)>, // active power-ups with remaining duration
    speed_multiplier: f32, // สำหรับ speed boost
    ghost_mode: bool, // สำหรับ ghost mode
    game_mode: GameMode, // โหมดเกม
    obstacles: Vec<Position>, // อุปสรรค
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
            power_ups: Vec::new(),
            active_power_ups: Vec::new(),
            speed_multiplier: 1.0,
            ghost_mode: false,
            game_mode: GameMode::Normal,
            obstacles: Vec::new(),
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

    fn random_power_up(snake: &VecDeque<Position>, food: &Position) -> PowerUp {
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

    fn generate_obstacles(&self) -> Vec<Position> {
        let mut obstacles = Vec::new();
        let mut rng = thread_rng();
        
        // สร้าง obstacles 5-8 ตัว
        let num_obstacles = rng.gen_range(5..9);
        
        for _ in 0..num_obstacles {
            loop {
                let pos = Position {
                    x: rng.gen_range(0..GRID_WIDTH),
                    y: rng.gen_range(0..GRID_HEIGHT),
                };
                // ตรวจสอบว่าไม่ชนกับงู, อาหาร, หรือ obstacles อื่น
                if !self.snake.contains(&pos) && pos != self.food && !obstacles.contains(&pos) {
                    obstacles.push(pos);
                    break;
                }
            }
        }
        obstacles
    }

    fn save_current_score(&mut self) {
        let score = self.snake.len() - 1;
        if score > self.high_score {
            self.high_score = score;
            self.save_high_score();
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
        self.power_ups.clear();
        self.active_power_ups.clear();
        self.speed_multiplier = 1.0;
        self.ghost_mode = false;
        
        // สร้าง obstacles ตามโหมดเกม
        if self.game_mode == GameMode::Obstacle {
            self.obstacles = self.generate_obstacles();
        } else {
            self.obstacles.clear();
        }
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
        let speed_threshold = (10.0 / self.speed_multiplier) as u8;
        if self.frame_counter < speed_threshold {
            return;
        }
        self.frame_counter = 0;

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
            self.power_ups.push(Self::random_power_up(&self.snake, &self.food));
        }

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

        // ตรวจสอบการชนกับตัวเองหรือ obstacles
        if (self.snake.contains(&new_head) || self.obstacles.contains(&new_head)) && !self.ghost_mode {
            // Play crash sound
            if self.sound_manager.sound_enabled {
                println!("crash sound played!");
            }
            self.save_current_score(); // บันทึกคะแนนก่อนเกมจบ
            self.game_over = true;
            self.state = GameState::GameOver;
            return;
        }

        self.snake.push_front(new_head);

        // ตรวจสอบการชนกับ Power-up
        if let Some(power_up_index) = self.power_ups.iter().position(|p| p.position == new_head) {
            let power_up = self.power_ups.remove(power_up_index);
            match power_up.power_type {
                PowerUpType::SpeedBoost => {
                    self.speed_multiplier = 2.0;
                    self.active_power_ups.push((PowerUpType::SpeedBoost, power_up.duration));
                    if self.sound_manager.sound_enabled {
                        println!("speed boost activated!");
                    }
                }
                PowerUpType::Shrink => {
                    // ลบหางงู 2 ตัว
                    for _ in 0..2 {
                        if self.snake.len() > 1 {
                            self.snake.pop_back();
                        }
                    }
                    if self.sound_manager.sound_enabled {
                        println!("shrink activated!");
                    }
                }
                PowerUpType::GhostMode => {
                    self.ghost_mode = true;
                    self.active_power_ups.push((PowerUpType::GhostMode, power_up.duration));
                    if self.sound_manager.sound_enabled {
                        println!("ghost mode activated!");
                    }
                }
            }
        }

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

        // วาด Power-ups
        for power_up in &self.power_ups {
            let color = match power_up.power_type {
                PowerUpType::SpeedBoost => YELLOW,
                PowerUpType::Shrink => ORANGE,
                PowerUpType::GhostMode => PURPLE,
            };
            draw_rectangle(
                offset_x + power_up.position.x as f32 * cell_size,
                offset_y + power_up.position.y as f32 * cell_size,
                cell_size,
                cell_size,
                color,
            );
            // วาดสัญลักษณ์ Power-up
            let symbol = match power_up.power_type {
                PowerUpType::SpeedBoost => "speed",
                PowerUpType::Shrink => "shrink",
                PowerUpType::GhostMode => "ghost",
            };
            draw_text(
                symbol,
                offset_x + power_up.position.x as f32 * cell_size + cell_size / 4.0,
                offset_y + power_up.position.y as f32 * cell_size + cell_size / 2.0,
                cell_size / 2.0,
                BLACK,
            );
        }

        // วาด Obstacles
        for obstacle in &self.obstacles {
            draw_rectangle(
                offset_x + obstacle.x as f32 * cell_size,
                offset_y + obstacle.y as f32 * cell_size,
                cell_size,
                cell_size,
                DARKGRAY,
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

        // แสดงสถานะ Power-up ที่กำลังทำงาน
        let mut y_offset = 30.0;
        for (power_type, duration) in &self.active_power_ups {
            let text = match power_type {
                PowerUpType::SpeedBoost => format!("speed boost: {}s", duration / 60),
                PowerUpType::GhostMode => format!("ghost mode: {}s", duration / 60),
                PowerUpType::Shrink => "shrink active".to_string(),
            };
            draw_text(
                &text,
                10.0,
                screen_h - y_offset,
                16.0,
                YELLOW,
            );
            y_offset += 20.0;
        }

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
                } else if self.mode_button.is_clicked() {
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
