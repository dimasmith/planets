use graphics::character::CharacterCache;
use graphics::types::{Color, Radius, Rectangle};
use graphics::{Context, Ellipse};
use hecs::World;
use opengl_graphics::GlGraphics;

use crate::physics::motion::Position;
use crate::render::render_box::RenderBoxComponent;

pub struct CircleComponent {
    pub circle: Ellipse,
    pub bound: Rectangle,
    radius: Radius,
}

struct CircleTraceElement {
    pub circle: Ellipse,
    pub bound: Rectangle,
}

/// component that enables circle traces
///
/// # Example
/// ```
/// CircleTrace::new()
/// ```
pub struct CircleTrace {
    elements: Vec<CircleTraceElement>,
    samples: usize,
    interval: usize,
    steps: usize,
}

impl CircleTrace {
    pub fn new() -> Self {
        CircleTrace {
            elements: vec![],
            samples: 16,
            interval: 16,
            steps: 0,
        }
    }

    fn update(&mut self, sprite: &CircleComponent) {
        self.steps += 1;
        if self.steps % self.interval != 0 {
            return;
        }
        self.steps = 0;

        for ct in self.elements.iter_mut() {
            ct.circle.color[3] -= 0.01;
        }

        let mut ct = CircleTraceElement {
            bound: sprite.bound,
            circle: sprite.circle,
        };
        ct.circle.color[3] = self.samples as f32 / 100.0;
        self.elements.push(ct);
        if self.elements.len() > self.samples {
            self.elements.remove(0);
        }
    }
}

impl CircleComponent {
    pub fn new(color: Color, radius: Radius) -> Self {
        let circle = Ellipse::new(color);
        let bound = graphics::rectangle::centered_square(0.0, 0.0, radius);
        CircleComponent {
            circle,
            bound,
            radius,
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
        for (id, (sprite, rendering_position)) in
            &mut world.query::<(&mut CircleComponent, &RenderBoxComponent)>()
        {
            sprite.set_position(rendering_position.position());
            sprite
                .circle
                .draw(sprite.bound, draw_state, context.transform, gl);
        }
    }
}

pub struct CircleTraceSystem {}

impl CircleTraceSystem {
    pub fn new() -> Self {
        CircleTraceSystem {}
    }

    pub fn update(&self, world: &mut World, context: Context, gl: &mut GlGraphics) {
        let draw_state = &context.draw_state;
        for (id, (sprite, trace)) in &mut world.query::<(&CircleComponent, &mut CircleTrace)>() {
            trace.update(&sprite);
            for element in trace.elements.iter() {
                element
                    .circle
                    .draw(element.bound, draw_state, context.transform, gl);
            }
        }
    }
}
