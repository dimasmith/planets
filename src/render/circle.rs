use graphics::character::CharacterCache;
use graphics::types::{Color, Radius, Rectangle};
use graphics::{Context, Ellipse};
use hecs::World;
use opengl_graphics::GlGraphics;

use crate::physics::motion::Position;
use crate::render::render_box::RenderBoxComponent;

pub struct CircleComponent {
    pub circle: Ellipse,
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

    fn update(&mut self, sprite: &CircleComponent, render_box: &RenderBoxComponent) {
        self.steps += 1;
        if self.steps % self.interval != 0 {
            return;
        }
        self.steps = 0;

        for ct in self.elements.iter_mut() {
            ct.circle.color[3] -= 0.01;
        }

        let mut ct = CircleTraceElement {
            bound: render_box.bound(),
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
    pub fn new(color: Color) -> Self {
        let circle = Ellipse::new(color);
        CircleComponent { circle }
    }
}

pub struct CircleSystem {}

impl CircleSystem {
    pub fn new() -> Self {
        CircleSystem {}
    }

    pub fn update(&self, world: &mut World, context: Context, gl: &mut GlGraphics) {
        let draw_state = &context.draw_state;
        for (id, (sprite, render_box)) in
            &mut world.query::<(&mut CircleComponent, &RenderBoxComponent)>()
        {
            sprite
                .circle
                .draw(render_box.bound(), draw_state, context.transform, gl);
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
        for (id, (trace, sprite, render_box)) in
            &mut world.query::<(&mut CircleTrace, &CircleComponent, &RenderBoxComponent)>()
        {
            trace.update(&sprite, &render_box);
            for element in trace.elements.iter() {
                element
                    .circle
                    .draw(element.bound, draw_state, context.transform, gl);
            }
        }
    }
}
