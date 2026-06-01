use nova_engine::{Engine, Sprite, Color};

fn main() {
    let engine = Engine::new("Nova Engine - Hello Sprite", 800, 600);

    let mut player = Sprite::new(400.0, 300.0, 64.0, 64.0)
        .with_color(Color::RED);

    engine.run(move |delta| {
        // Move player right over time
        player.move_by(100.0 * delta, 0.0);
    });
}
