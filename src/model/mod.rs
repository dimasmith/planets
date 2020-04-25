use crate::model::planet::Planet;

pub mod planet;

pub struct World {
    pub planets: Vec<Planet>,
}
