use crate::model::World;
use crate::physics::motion::Position;
use graphics::types::{Color, Radius, Rectangle};
use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

pub struct CircleComponent {
    pub circle: Ellipse,
    pub bound: Rectangle,
    radius: Radius,
}

impl CircleComponent {
    pub fn new(color: Color, center: Position, radius: Radius) -> Self {
        let circle = Ellipse::new(color);
        let bound = graphics::rectangle::centered_square(center[0], center[1], radius);
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
        for sprite in world.sprites().iter() {
            let draw_state = &context.draw_state;
            sprite
                .circle
                .draw(sprite.bound, draw_state, context.transform, gl);
        }
    }
}
