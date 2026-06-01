use glam::Vec2;

pub struct Camera2D {
    pub position: Vec2,
    pub zoom: f32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn follow(&mut self, target: Vec2, lerp_speed: f32) {
        self.position = self.position.lerp(target, lerp_speed);
    }
}
