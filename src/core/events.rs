use piston::event_loop::Events;
use piston::input::Event;
use piston::window::Window;

/// implementations should handle game events.
pub trait EventHandler {
    /// handle game event
    /// return true if event loop should stop
    fn handle_event(&mut self, event: Event) -> bool;
}

pub struct EventLoop {
    emitter: Events,
}

impl EventLoop {
    pub fn new(emitter: Events) -> Self {
        EventLoop { emitter }
    }

    pub fn activate_stage(&mut self, stage: &mut impl EventHandler, window: &mut impl Window) {
        while let Some(e) = self.emitter.next(window) {
            let stop = stage.handle_event(e);
            if stop {
                break;
            }
        }
    }
}
