use std::collections::VecDeque;
use crate::utils::{GRID_WIDTH, GRID_HEIGHT};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PowerUpType {
    SpeedBoost,
    Shrink,
    GhostMode,
}

#[derive(Copy, Clone)]
pub struct PowerUp {
    pub position: Position,
    pub power_type: PowerUpType,
    pub duration: u32, // frames
}

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq)]
pub enum GameMode {
    Normal,
    Obstacle,
} 