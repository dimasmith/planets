use crate::render::circle::CircleComponent;
use graphics::character::CharacterCache;
use graphics::{Context, Transformed};
use hecs::World;
use opengl_graphics::{GlGraphics, GlyphCache};

pub struct NameComponent {
    pub name: String,
}

impl NameComponent {
    pub fn new(name: String) -> Self {
        NameComponent { name }
    }
}

pub struct NameSystem {}

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
        for (id, (name_component, sprite)) in
            &mut world.query::<(&NameComponent, &CircleComponent)>()
        {
            let position = [
                sprite.bound[0] + sprite.bound[2],
                sprite.bound[1] + sprite.bound[3],
            ];
            graphics::text(
                [1.0, 1.0, 1.0, 1.0],
                16,
                &name_component.name,
                glyphs,
                context.trans_pos(position).transform,
                gl,
            )
            .expect("can't render the name")
        }
    }
}
