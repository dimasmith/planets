use crate::physics::motion::Position;
use vecmath::Vector2;

pub type RenderingPosition = Vector2<f64>;
/// body rendering position and bounds
pub struct RenderBoxComponent {
    position: RenderingPosition,
}

impl RenderBoxComponent {
    /// create rendering position component
    pub fn new() -> Self {
        RenderBoxComponent {
            position: [0.0, 0.0],
        }
    }

    /// get current rendering position
    pub fn position(&self) -> RenderingPosition {
        self.position
    }

    /// change rendering position to a new coordinates
    pub fn move_to(&mut self, position: RenderingPosition) {
        self.position = position;
    }
}
