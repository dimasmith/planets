use graphics::types::Color;
use graphics::{Context, Ellipse, Image};
use hecs::World;
use opengl_graphics::{GlGraphics, Texture};

use crate::render::render_box::RenderBoxComponent;

pub enum SpriteKind {
    Circle(Ellipse, Color),
    Image(Image, Texture),
}

pub struct Sprite {
    kind: SpriteKind,
}

impl Sprite {
    /// create colored circle sprite
    #[allow(dead_code)]
    pub fn circle(color: Color) -> Self {
        let circle = Ellipse::new(color);
        Sprite {
            kind: SpriteKind::Circle(circle, color),
        }
    }

    pub fn image(texture: Texture) -> Self {
        Sprite {
            kind: SpriteKind::Image(Image::new(), texture),
        }
    }

    pub fn kind(&self) -> &SpriteKind {
        &self.kind
    }
}

pub struct SpriteSystem {}

impl SpriteSystem {
    pub fn new() -> Self {
        SpriteSystem {}
    }

    pub fn update(&self, world: &mut World, context: Context, gl: &mut GlGraphics) {
        let draw_state = &context.draw_state;
        for (_id, (sprite, render_box)) in &mut world.query::<(&mut Sprite, &RenderBoxComponent)>()
        {
            match sprite.kind() {
                SpriteKind::Circle(circle, _color) => {
                    circle.draw(render_box.bound(), draw_state, context.transform, gl);
                }
                SpriteKind::Image(image, texture) => {
                    image
                        .rect(render_box.bound())
                        .draw(texture, draw_state, context.transform, gl);
                }
            }
        }
    }
}
