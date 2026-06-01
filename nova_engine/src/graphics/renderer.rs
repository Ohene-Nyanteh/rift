use crate::graphics::color::Color;

// The renderer will hold the wgpu surface, device, queue etc.
// This is a skeleton — full wgpu setup will be wired into Engine::run
pub struct Renderer {
    pub clear_color: Color,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            clear_color: Color::BLACK,
        }
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }
}
