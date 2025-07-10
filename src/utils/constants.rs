use macroquad::prelude::*;

pub const GRID_WIDTH: i32 = 40;
pub const GRID_HEIGHT: i32 = 30;

pub fn get_cell_size() -> f32 {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let cell_w = screen_w / GRID_WIDTH as f32;
    let cell_h = screen_h / GRID_HEIGHT as f32;
    cell_w.min(cell_h) // ใช้ขนาดที่เล็กกว่าเพื่อรักษาอัตราส่วน
} 