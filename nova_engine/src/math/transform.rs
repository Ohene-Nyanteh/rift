use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32, // radians
    pub scale: Vec2,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }

    pub fn at(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..Self::new()
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn rotate(&mut self, angle_radians: f32) {
        self.rotation += angle_radians;
    }
}
