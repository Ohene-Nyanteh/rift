pub mod core;
pub mod graphics;
pub mod audio;
pub mod events;
pub mod math;

// Re-export the most commonly used types at the top level
pub use core::engine::Engine;
pub use core::time::Time;
pub use graphics::color::Color;
pub use graphics::sprite::Sprite;
pub use math::transform::Transform;
pub use events::input::{Input, Key, MouseButton};
