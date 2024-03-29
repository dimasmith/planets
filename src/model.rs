use crate::core::texture::load_texture;
use crate::loader::model_loader::ToEntityBuilder;
use crate::physics::force::ForceComponent;
use crate::physics::gravity::{Mass, MassComponent};
use crate::physics::motion::Motion;
use crate::render::background::BackgroundComponent;
use crate::render::name::NameComponent;
use crate::render::render_box::RenderBoxComponent;
use crate::render::sprite::Sprite;
use assets_manager::{loader, Asset};
use hecs::EntityBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Planet {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub name: String,
    pub mass: Mass,
    pub image: String,
    pub visible_radius: f64,
}

impl ToEntityBuilder for Planet {
    fn to_entity_builder(&self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();
        let position = [self.position.0, self.position.1];
        let velocity = [self.velocity.0, self.velocity.1];
        builder
            .add(NameComponent::new(String::from(self.name.as_str())))
            .add(MassComponent::new(self.mass))
            .add(Motion::new_position_velocity(position, velocity))
            .add(ForceComponent::zero())
            .add(RenderBoxComponent::centered_square(self.visible_radius))
            .add(Sprite::image(load_texture(String::from(
                self.image.as_str(),
            ))));
        builder
    }
}

impl Asset for Planet {
    const EXTENSION: &'static str = "ron";
    type Loader = loader::RonLoader;
}

#[derive(Deserialize)]
pub struct Background {
    pub image: String,
}

impl Asset for Background {
    const EXTENSION: &'static str = "ron";
    type Loader = loader::RonLoader;
}

impl ToEntityBuilder for Background {
    fn to_entity_builder(&self) -> EntityBuilder {
        let image_path = String::from(self.image.as_str());
        let mut builder = EntityBuilder::new();
        builder.add(BackgroundComponent::image(load_texture(image_path)));
        builder
    }
}

#[derive(Deserialize)]
pub struct Simulation {
    planets: Vec<Planet>,
    background: Background,
}

impl Asset for Simulation {
    const EXTENSION: &'static str = "ron";
    type Loader = loader::RonLoader;
}

impl Simulation {
    pub fn models(&self) -> Vec<&dyn ToEntityBuilder> {
        let mut models: Vec<&dyn ToEntityBuilder> = vec![&self.background];
        self.planets.iter().for_each(|planet| models.push(planet));
        models
    }
}
