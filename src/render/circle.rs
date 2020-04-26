use crate::physics::motion::Position;
use graphics::types::{Color, Radius, Rectangle};
use graphics::Ellipse;

pub struct CircleComponent {
    pub circle: Ellipse,
    pub bound: Rectangle,
}

impl CircleComponent {
    pub fn new(color: Color, center: Position, radius: Radius) -> Self {
        let circle = Ellipse::new(color);
        let bound = graphics::rectangle::centered_square(center[0], center[1], radius);
        CircleComponent { circle, bound }
    }
}
