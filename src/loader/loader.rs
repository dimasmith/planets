use crate::loader::state::LoadingState;
use crate::physics::force::ForceComponent;
use crate::physics::gravity::{Mass, MassComponent};
use crate::physics::motion::{Motion, Position, Velocity};
use crate::render::camera::TrackingComponent;
use crate::render::name::NameComponent;
use crate::render::render_box::RenderBoxComponent;
use crate::render::sprite::Sprite;
use hecs::{EntityBuilder, World};
use image::io::Reader;
use opengl_graphics::{Texture, TextureSettings};
use std::borrow::Borrow;

pub struct ModelLoader<T: ?Sized> {
    loaded: usize,
    models: Vec<Box<T>>,
}

impl<T> ModelLoader<T>
where
    T: ?Sized + ToEntityBuilder,
{
    pub fn new(models: Vec<Box<T>>) -> Self {
        ModelLoader { loaded: 0, models }
    }

    pub fn update(&mut self, loading_state: &mut LoadingState, world: &mut World) {
        if self.loaded < self.models.len() {
            let e = self.models.get(self.loaded).unwrap();
            world.spawn(e.to_entity_builder().build());
            let progress = self.loaded as f64 / self.models.len() as f64;
            loading_state.set_progress(progress);
            self.loaded += 1;
        } else {
            let heaviest_body = world
                .query::<(&MassComponent,)>()
                .iter()
                .max_by(|x, y| {
                    let a = ((x.1).0).mass;
                    let b = ((y.1).0).mass;
                    a.partial_cmp(&b).unwrap()
                })
                .unwrap()
                .0;

            world
                .insert_one(heaviest_body, TrackingComponent::new())
                .unwrap();

            loading_state.set_progress(1.0);
        }
    }
}

pub trait ToEntityBuilder {
    fn to_entity_builder(&self) -> EntityBuilder;
}

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

fn load_texture(name: &str) -> Texture {
    let texture_settings = TextureSettings::new();
    let kerbin_texture_image = Reader::open("textures/".to_string() + name + ".png")
        .unwrap()
        .decode()
        .unwrap();
    Texture::from_image(kerbin_texture_image.into_rgba().borrow(), &texture_settings)
}
