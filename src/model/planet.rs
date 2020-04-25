use crate::physics::gravity::{Gravity, Mass};
use crate::physics::motion;
use crate::physics::motion::Position;
use crate::render::renderable::Renderable;
use crate::render::{Projector, Renderer};
use graphics::math::Matrix2d;
use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::ops::Mul;

pub struct Planet {
    pub name: String,
    pub motion: motion::Motion,
    pub mass: Mass,
    trace: Vec<Position>,
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
            mass: 1.0e8,
            trace: vec![],
        }
    }

    pub fn gravity(&self) -> Gravity {
        Gravity {
            mass: self.mass,
            position: self.motion.position,
        }
    }
}

const PALE_BLUE: [f32; 4] = [0.2, 0.2, 0.9, 1.0];

impl Renderable for Planet {
    fn pre_render(
        &mut self,
        projector: &Projector,
        transform: [[f64; 3]; 2],
        context: &mut Context,
        gl: &mut GlGraphics,
    ) {
        let position: Position = projector.project(&self.motion.position);
        self.trace.push(position);

        if self.trace.len() > 25 {
            self.trace.remove(0);
        }

        let da = 1.0 / 25.0;
        let mut c = 0.0;
        for (i, trace) in self.trace.iter().enumerate() {
            c += da;
            let alpha = da * c;
            let color = [0.2, 0.2, 0.9, alpha];
            let bound = graphics::rectangle::centered_square(trace[0], trace[1], 25.0);
            graphics::ellipse(color, bound, transform, gl);
        }
    }

    fn render(
        &mut self,
        projector: &Projector,
        transform: Matrix2d,
        context: &mut Context,
        gl: &mut GlGraphics,
    ) {
        let position: Position = projector.project(&self.motion.position);

        let bound = graphics::rectangle::centered_square(position[0], position[1], 25.0);
        graphics::ellipse(PALE_BLUE, bound, transform, gl);
    }
}
