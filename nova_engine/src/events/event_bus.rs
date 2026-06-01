// Simple event bus for decoupled internal communication
// Expand this with specific event types as your engine grows

pub enum EngineEvent {
    WindowResized(u32, u32),
    WindowClosed,
    Custom(String),
}

pub struct EventBus {
    events: Vec<EngineEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn push(&mut self, event: EngineEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<EngineEvent> {
        std::mem::take(&mut self.events)
    }
}
