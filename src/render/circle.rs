use graphics::types::Color;
use graphics::{Context, Ellipse};
use hecs::World;
use opengl_graphics::GlGraphics;

use crate::render::render_box::RenderBoxComponent;

pub struct CircleComponent {
    pub circle: Ellipse,
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
        for (_id, (sprite, render_box)) in
            &mut world.query::<(&mut CircleComponent, &RenderBoxComponent)>()
        {
            sprite
                .circle
                .draw(render_box.bound(), draw_state, context.transform, gl);
        }
    }
}
