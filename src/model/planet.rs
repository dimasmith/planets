use crate::physics::gravity::Gravity;
use crate::physics::motion;
use crate::physics::motion::Position;
use crate::render::circle::{CircleComponent, CircleTrace};
use crate::render::renderable::Renderable;
use crate::render::Projector;
use graphics::Context;
use opengl_graphics::GlGraphics;

const PALE_BLUE: [f32; 4] = [0.2, 0.2, 0.9, 1.0];

pub struct Planet {
    pub name: String,
    pub motion: motion::Motion,
    pub gravity: Gravity,
    pub sprite: CircleComponent,
    pub trace: CircleTrace,
}

impl Planet {
    pub fn new(name: String, position: (f64, f64)) -> Planet {
        let movement = motion::Motion {
            position: [position.0, position.1],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        };

        Planet {
            name,
            motion: movement,
            gravity: Gravity::new(1.0e8),
            trace: CircleTrace::new(),
            sprite: CircleComponent::new(PALE_BLUE, [position.0, position.1], 25.0),
        }
    }
}
