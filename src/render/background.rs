use graphics::{Context, Image};
use hecs::World;
use opengl_graphics::{GlGraphics, Texture};
use piston::input::RenderArgs;
use std::borrow::Borrow;

pub struct BackgroundComponent {
    texture: Texture,
}

impl BackgroundComponent {
    pub fn image(texture: Texture) -> Self {
        BackgroundComponent { texture }
    }

    pub fn texture(&self) -> &Texture {
        self.texture.borrow()
    }
}

pub struct BackgroundSystem {}

impl BackgroundSystem {
    pub fn new() -> Self {
        BackgroundSystem {}
    }

    pub fn update(
        &self,
        world: &mut World,
        context: Context,
        gl: &mut GlGraphics,
        args: RenderArgs,
    ) {
        let draw_state = &context.draw_state;
        for (_id, (background,)) in &mut world.query::<(&BackgroundComponent,)>() {
            let viewport = [0.0, 0.0, args.window_size[0], args.window_size[1]];
            let texture = background.texture();
            Image::new()
                .rect(viewport)
                .draw(texture, draw_state, context.transform, gl);
        }
    }
}
