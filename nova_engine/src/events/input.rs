use std::collections::HashSet;

// Re-export winit keys so users don't need to import winit directly
pub use winit::keyboard::KeyCode as Key;
pub use winit::event::MouseButton;

pub struct Input {
    pressed: HashSet<Key>,
    just_pressed: HashSet<Key>,
    just_released: HashSet<Key>,
    pub mouse_position: (f64, f64),
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
            mouse_position: (0.0, 0.0),
        }
    }

    // Called by the engine each frame before update
    pub fn flush(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn on_key_press(&mut self, key: Key) {
        self.just_pressed.insert(key);
        self.pressed.insert(key);
    }

    pub fn on_key_release(&mut self, key: Key) {
        self.just_released.insert(key);
        self.pressed.remove(&key);
    }

    /// True every frame the key is held down
    pub fn key_held(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    /// True only on the first frame the key is pressed
    pub fn key_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    /// True only on the frame the key is released
    pub fn key_just_released(&self, key: Key) -> bool {
        self.just_released.contains(&key)
    }
}
