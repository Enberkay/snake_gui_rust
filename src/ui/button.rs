use macroquad::prelude::*;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub color: Color,
    pub hover_color: Color,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: String) -> Self {
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

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self) {
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

    pub fn is_clicked(&self) -> bool {
        let mouse_pos = mouse_position();
        let is_hovered = mouse_pos.0 >= self.x && mouse_pos.0 <= self.x + self.width
            && mouse_pos.1 >= self.y && mouse_pos.1 <= self.y + self.height;
        
        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }
} 