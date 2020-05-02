use crate::loader::loader::ToEntityBuilder;
use crate::physics::force::ForceComponent;
use crate::physics::gravity::{Mass, MassComponent};
use crate::physics::motion::{Motion, Position, Velocity};
use crate::render::background::BackgroundComponent;
use crate::render::name::NameComponent;
use crate::render::render_box::RenderBoxComponent;
use crate::render::sprite::Sprite;
use hecs::EntityBuilder;
use image::io::Reader;
use opengl_graphics::{Texture, TextureSettings};
use std::borrow::Borrow;

pub struct Planet {
    pub position: Position,
    pub velocity: Velocity,
    pub name: &'static str,
    pub mass: Mass,
    pub image: &'static str,
    pub visible_radius: f64,
}

impl ToEntityBuilder for Planet {
    fn to_entity_builder(&self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();
        builder
            .add(NameComponent::new(self.name))
            .add(MassComponent::new(self.mass))
            .add(Motion::new_position_velocity(self.position, self.velocity))
            .add(ForceComponent::zero())
            .add(RenderBoxComponent::centered_square(self.visible_radius))
            .add(Sprite::image(load_texture(self.image)));
        builder
    }
}

pub struct Background {
    pub image: &'static str,
}

impl ToEntityBuilder for Background {
    fn to_entity_builder(&self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();
        builder.add(BackgroundComponent::image(load_texture(self.image)));
        builder
    }
}

fn load_texture(name: &str) -> Texture {
    let texture_settings = TextureSettings::new();
    let kerbin_texture_image = Reader::open("textures/".to_string() + name + ".png")
        .unwrap()
        .decode()
        .unwrap();
    Texture::from_image(kerbin_texture_image.into_rgba().borrow(), &texture_settings)
}
