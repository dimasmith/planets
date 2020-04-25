use crate::physics::gravity::{Gravity, Mass};
use crate::physics::motion;
use crate::render::{Renderable, Renderer};
use graphics::Transformed;
use piston::input::RenderArgs;

pub struct Planet {
    pub name: String,
    pub motion: motion::Motion,
    pub mass: Mass,
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
    fn render(&mut self, renderer: &mut Renderer, args: RenderArgs) {
        let position = renderer.project(&self.motion.position);
        let bound = graphics::rectangle::centered_square(position[0], position[1], 25.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        renderer.gl.draw(args.viewport(), |c, gl| {
            let planet_transform = c.trans(x, y).transform;
            graphics::ellipse(PALE_BLUE, bound, planet_transform, gl);
        });
    }
}
