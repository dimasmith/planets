use crate::model::World;
use crate::physics::motion::Position;
use graphics::types::{Color, Radius, Rectangle};
use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

pub struct CircleComponent {
    pub circle: Ellipse,
    pub bound: Rectangle,
    radius: Radius,
    trace: Option<Trace>,
}

struct CircleTrace {
    pub circle: Ellipse,
    pub bound: Rectangle,
    radius: Radius,
}

struct Trace {
    elements: Vec<CircleTrace>,
    length: usize,
    steps: usize,
}

impl Trace {
    pub fn new() -> Self {
        Trace {
            elements: vec![],
            length: 16,
            steps: 0,
        }
    }

    pub fn add(&mut self, mut element: CircleTrace) {
        self.steps += 1;
        if self.steps % 16 != 0 {
            return;
        }
        self.steps = 0;

        self.elements.push(element);
        let length = self.length;
        if self.elements.len() > length {
            self.elements.remove(0);
        }
        let da = 0.25 / length as f64;
        for (i, e) in self.elements.iter_mut().enumerate() {
            let alpha = da as f32 * i as f32;
            e.circle.color[3] = alpha;
        }
    }
}

impl CircleComponent {
    pub fn new(color: Color, center: Position, radius: Radius) -> Self {
        let circle = Ellipse::new(color);
        let bound = graphics::rectangle::centered_square(center[0], center[1], radius);
        CircleComponent {
            circle,
            bound,
            radius,
            trace: Some(Trace::new()),
        }
    }

    fn update_trace(&mut self) {
        let ct = CircleTrace {
            bound: self.bound,
            circle: self.circle,
            radius: self.radius,
        };
        if let Some(trace) = self.trace.as_mut() {
            trace.add(ct);
        }
    }

    pub fn set_position(&mut self, position: Position) {
        let radius = self.radius;
        self.bound = graphics::rectangle::centered_square(position[0], position[1], radius);
    }
}

pub struct CircleSystem {}

impl CircleSystem {
    pub fn new() -> Self {
        CircleSystem {}
    }

    pub fn update(&self, world: &mut World, context: Context, gl: &mut GlGraphics) {
        let draw_state = &context.draw_state;
        for sprite in world.sprites().iter_mut() {
            sprite.update_trace();
            if let Some(trace) = sprite.trace.as_ref() {
                for t in trace.elements.iter() {
                    t.circle.draw(t.bound, draw_state, context.transform, gl);
                }
            }
            sprite
                .circle
                .draw(sprite.bound, draw_state, context.transform, gl);
        }
    }
}
