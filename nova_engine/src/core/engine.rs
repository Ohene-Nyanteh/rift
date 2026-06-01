use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::core::time::Time;

pub struct Engine {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        env_logger::init();
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }

    /// Start the game loop. The closure receives delta time each frame.
    pub fn run<F>(self, mut update: F)
    where
        F: FnMut(f32) + 'static,
    {
        let event_loop = EventLoop::new().unwrap();
        let _window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .build(&event_loop)
            .unwrap();

        let mut time = Time::new();

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    time.tick();
                    update(time.delta);
                }
                _ => {}
            }
        }).unwrap();
    }
}
