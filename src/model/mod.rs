use crate::model::planet::Planet;
use crate::physics::gravity::Gravity;
use crate::physics::motion::Motion;

pub mod planet;

pub struct World {
    pub planets: Vec<Planet>,
}

impl World {
    pub fn motions(&mut self) -> Vec<&mut Motion> {
        self.planets.iter_mut().map(|p| &mut p.motion).collect()
    }

    pub fn gravity(&mut self) -> Vec<(&mut Motion, &mut Gravity)> {
        self.planets
            .iter_mut()
            .map(|p| (&mut p.motion, &mut p.gravity))
            .collect()
    }
}
