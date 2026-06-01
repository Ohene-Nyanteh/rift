use nova_engine::Engine;

fn main() {
    let engine = Engine::new("Nova Engine - Hello Window", 800, 600);

    engine.run(|delta| {
        // Your game update logic goes here
        // delta = time in seconds since last frame
        let _ = delta;
    });
}
