// Scene trait — implement this for each screen/level in your game
pub trait Scene {
    fn on_enter(&mut self);
    fn on_exit(&mut self);
    fn update(&mut self, delta: f32);
    // renderer and input will be passed in once the engine is wired up
}
