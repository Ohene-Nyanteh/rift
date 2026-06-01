use glam::Vec2;
use crate::graphics::color::Color;

pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub texture_path: Option<String>,
    pub visible: bool,
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
            color: Color::WHITE,
            texture_path: None,
            visible: true,
        }
    }

    pub fn with_texture(mut self, path: &str) -> Self {
        self.texture_path = Some(path.to_string());
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
    }
}
