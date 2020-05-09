use piston::input::Event;

/// implementations should handle game events.
pub trait EventHandler {
    /// handle game event
    /// return true if event loop should stop
    fn handle_event(&mut self, event: Event) -> bool;
}
