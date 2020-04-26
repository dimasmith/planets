use crate::physics::gravity::Gravity;
use crate::physics::motion;
use crate::physics::motion::Position;
use crate::render::circle::CircleComponent;
use crate::render::renderable::Renderable;
use crate::render::Projector;
use graphics::Context;
use opengl_graphics::GlGraphics;

pub struct Planet {
    pub name: String,
    pub motion: motion::Motion,
    pub gravity: Gravity,
    pub sprite: CircleComponent,
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
            gravity: Gravity::new(1.0e8),
            trace: vec![],
            sprite: CircleComponent::new(PALE_BLUE, [position.0, position.1], 25.0),
        }
    }
}

const PALE_BLUE: [f32; 4] = [0.2, 0.2, 0.9, 1.0];

impl Renderable for Planet {
    fn pre_render(&mut self, projector: &Projector, context: &mut Context, gl: &mut GlGraphics) {
        let position: Position = projector.project(&self.motion.position);
        self.trace.push(position);

        if self.trace.len() > 25 {
            self.trace.remove(0);
        }

        let da = 1.0 / 25.0;
        let mut c = 0.0;
        for trace in self.trace.iter() {
            c += da;
            let alpha = da * c;
            let color = [0.2, 0.2, 0.9, alpha];
            let bound = graphics::rectangle::centered_square(trace[0], trace[1], 25.0);
            graphics::ellipse(color, bound, context.transform, gl);
        }
    }

    fn render(&mut self, projector: &Projector, context: &mut Context, gl: &mut GlGraphics) {
        let position: Position = projector.project(&self.motion.position);

        let bound = graphics::rectangle::centered_square(position[0], position[1], 25.0);
        graphics::ellipse(PALE_BLUE, bound, context.transform, gl);
    }
}
