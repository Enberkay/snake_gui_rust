use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;

mod utils;
mod audio;
mod ui;
mod game;

use utils::*;
use audio::SoundManager;
use ui::Button;
use game::{Snake, Food, PowerUpManager, GameState, GameMode, Direction, PowerUpType};

struct SnakeGame {
    snake: Snake,
    food: Food,
    game_over: bool,
    frame_counter: u8,
    state: GameState,
    start_button: Button,
    exit_button: Button,
    sound_button: Button,
    mode_button: Button,
    high_score: usize,
    sound_manager: SoundManager,
    power_up_manager: PowerUpManager,
    game_mode: GameMode,
    obstacles: Vec<game::Position>,
}

impl SnakeGame {
    fn new() -> Self {
        let snake = Snake::new();
        let food = Food::new();
        let power_up_manager = PowerUpManager::new();

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

        let high_score = load_high_score();
        let sound_manager = SoundManager::new();

        SnakeGame {
            snake,
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
            power_up_manager,
            game_mode: GameMode::Normal,
            obstacles: Vec::new(),
        }
    }

    fn generate_obstacles(&self) -> Vec<game::Position> {
        let mut obstacles = Vec::new();
        let mut rng = thread_rng();
        
        let num_obstacles = rng.gen_range(5..9);
        
        for _ in 0..num_obstacles {
            loop {
                let pos = game::Position {
                    x: rng.gen_range(0..GRID_WIDTH),
                    y: rng.gen_range(0..GRID_HEIGHT),
                };
                if !self.snake.contains(&pos) && pos != self.food.position && !obstacles.contains(&pos) {
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
            save_high_score(self.high_score);
        }
    }

    fn reset_game(&mut self) {
        self.snake.reset();
        self.food.respawn(&self.snake.body);
        self.game_over = false;
        self.frame_counter = 0;
        self.power_up_manager.reset();
        
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

        let speed_threshold = (10.0 / self.power_up_manager.speed_multiplier) as u8;
        if self.frame_counter < speed_threshold {
            return;
        }
        self.frame_counter = 0;

        self.power_up_manager.update(&self.snake.body, &self.food.position);

        let new_head = self.snake.move_snake();

        if (self.snake.contains(&new_head) || self.obstacles.contains(&new_head)) && !self.power_up_manager.ghost_mode {
            self.sound_manager.play_crash_sound();
            self.save_current_score();
            self.game_over = true;
            self.state = GameState::GameOver;
            return;
        }

        self.snake.grow(new_head);

        // ตรวจสอบการชนกับ Power-up
        if let Some(power_type) = self.power_up_manager.check_collision(&new_head) {
            match power_type {
                PowerUpType::SpeedBoost => {
                    self.sound_manager.play_power_up_sound("speed boost");
                }
                PowerUpType::Shrink => {
                    for _ in 0..2 {
                        self.snake.shrink();
                    }
                    self.sound_manager.play_power_up_sound("shrink");
                }
                PowerUpType::GhostMode => {
                    self.sound_manager.play_power_up_sound("ghost mode");
                }
            }
        }

        if new_head == self.food.position {
            self.food.respawn(&self.snake.body);
            self.sound_manager.play_eat_sound();
        } else {
            self.snake.shrink();
        }
    }

    fn draw_menu(&self) {
        clear_background(BLACK);
        
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        draw_rectangle_lines(0.0, 0.0, screen_w, screen_h, 2.0, WHITE);
        
        draw_text(
            "SNAKE GAME",
            screen_w / 2.0 - 120.0,
            screen_h / 2.0 - 100.0,
            40.0,
            GREEN,
        );
        
        self.start_button.draw();
        self.exit_button.draw();
        self.sound_button.draw();
        self.mode_button.draw();
        
        draw_text(
            &format!("High Score: {}", self.high_score),
            screen_w / 2.0 - 90.0,
            screen_h / 2.0 - 60.0,
            30.0,
            YELLOW,
        );
        
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

        let game_width = GRID_WIDTH as f32 * cell_size;
        let game_height = GRID_HEIGHT as f32 * cell_size;
        let offset_x = (screen_w - game_width) / 2.0;
        let offset_y = (screen_h - game_height) / 2.0;

        draw_rectangle_lines(offset_x, offset_y, game_width, game_height, 2.0, WHITE);

        // วาดอาหาร
        draw_rectangle(
            offset_x + self.food.position.x as f32 * cell_size,
            offset_y + self.food.position.y as f32 * cell_size,
            cell_size,
            cell_size,
            RED,
        );

        // วาดงู
        for (i, seg) in self.snake.body.iter().enumerate() {
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
        for power_up in &self.power_up_manager.power_ups {
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

        draw_text(
            &format!("Score: {}", self.snake.len() - 1),
            10.0,
            screen_h - 10.0,
            20.0,
            WHITE,
        );

        let mut y_offset = 30.0;
        for (power_type, duration) in &self.power_up_manager.active_power_ups {
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
        
        draw_text(
            "GAME OVER",
            screen_w / 2.0 - 100.0,
            screen_h / 2.0 - 50.0,
            40.0,
            WHITE,
        );
        
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
                    self.snake.change_direction(Direction::Up);
                } else if is_key_pressed(KeyCode::Down) {
                    self.snake.change_direction(Direction::Down);
                } else if is_key_pressed(KeyCode::Left) {
                    self.snake.change_direction(Direction::Left);
                } else if is_key_pressed(KeyCode::Right) {
                    self.snake.change_direction(Direction::Right);
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
        game.update_button_positions();
        game.handle_input();

        if game.state == GameState::Playing {
            game.update();
        }

        game.draw();
        next_frame().await;
    }
} 