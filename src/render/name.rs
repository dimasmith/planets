use crate::render::render_box::{RenderBoxComponent, RenderingPosition};
use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::{Context, Transformed};
use hecs::World;
use opengl_graphics::{GlGraphics, GlyphCache};

pub struct NameComponent<'n> {
    pub name: &'n str,
}

impl<'n> NameComponent<'n> {
    pub fn new(name: &'n str) -> Self {
        NameComponent { name }
    }
}

pub struct NameSystem {}

const FONT_SIZE: FontSize = 16;

impl NameSystem {
    pub fn new() -> Self {
        NameSystem {}
    }

    pub fn update(
        &self,
        world: &mut World,
        glyphs: &mut GlyphCache,
        context: Context,
        gl: &mut GlGraphics,
    ) {
        for (_id, (name_component, render_box)) in
            &mut world.query::<(&NameComponent, &RenderBoxComponent)>()
        {
            let name = name_component.name;
            let bound = render_box.bound();
            let text_length: f64 = glyphs
                .width(FONT_SIZE, name)
                .or::<f64>(Ok(self.fallback_name_size(name)))
                .unwrap();
            let x = (bound[0] + bound[2] / 2.0) - text_length / 2.0;
            let y = bound[1] + bound[3] + FONT_SIZE as f64;
            let position: RenderingPosition = [x, y];

            graphics::text(
                [1.0, 1.0, 1.0, 1.0],
                16,
                name,
                glyphs,
                context.trans_pos(position).transform,
                gl,
            )
            .expect("can't render the name")
        }
    }

    fn fallback_name_size(&self, name: &str) -> f64 {
        let len = name.len() as f64;
        let width = len * FONT_SIZE as f64;
        width / 2.0
    }
}
