use crate::physics::motion::Position;
use graphics::rectangle::centered_square;
use graphics::types::Rectangle;
use vecmath::Vector2;

pub type RenderingPosition = Vector2<f64>;
/// body rendering position and bounds
pub struct RenderBoxComponent {
    position: RenderingPosition,
    radius: f64,
    bound: Rectangle,
}

const DEFAULT_RADIUS: f64 = 25.0;

impl RenderBoxComponent {
    /// create rendering position component with default radius of 25.0
    pub fn new() -> Self {
        RenderBoxComponent::centered_square(DEFAULT_RADIUS)
    }

    /// create centered rendering box with a radius (half-length of the square side)
    pub fn centered_square(radius: f64) -> Self {
        let size = radius * 2.0;
        RenderBoxComponent {
            position: [0.0, 0.0],
            radius,
            bound: [-radius, -radius, size, size],
        }
    }

    /// get current rendering position
    pub fn position(&self) -> RenderingPosition {
        self.position
    }

    /// drawing bound of the render box
    pub fn bound(&self) -> Rectangle {
        self.bound
    }

    /// change rendering position to a new coordinates
    pub fn move_to(&mut self, position: RenderingPosition) {
        self.position = position;
        let [x, y] = position;
        let radius = self.radius;
        let size = radius * 2.0;
        self.bound = [x - radius, y - radius, size, size];
    }
}
