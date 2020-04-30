use graphics::types::Rectangle;
use vecmath::Vector2;

pub type RenderingPosition = Vector2<f64>;
/// body rendering position and bounds
#[derive(Copy, Clone)]
pub struct RenderBoxComponent {
    position: RenderingPosition,
    radius: f64,
    bound: Rectangle,
}

impl RenderBoxComponent {
    /// create centered rendering box with a radius (half-length of the square side)
    pub fn centered_square(radius: f64) -> Self {
        let size = radius * 2.0;
        RenderBoxComponent {
            position: [0.0, 0.0],
            radius,
            bound: [-radius, -radius, size, size],
        }
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
