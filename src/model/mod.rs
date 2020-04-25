use crate::model::planet::Planet;
use crate::render::Renderable;

pub mod planet;

pub struct World {
    pub planets: Vec<Planet>,
}
