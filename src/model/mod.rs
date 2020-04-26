use crate::model::planet::Planet;
use crate::physics::gravity::Gravity;
use crate::physics::motion::{Motion, Position};
use crate::render::circle::{CircleComponent, CircleTrace};

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

    pub fn sprites_and_positions(&mut self) -> Vec<(&mut CircleComponent, &mut Position)> {
        self.planets
            .iter_mut()
            .map(|p| (&mut p.sprite, &mut p.motion.position))
            .collect()
    }

    pub fn sprites(&mut self) -> Vec<&mut CircleComponent> {
        self.planets.iter_mut().map(|p| &mut p.sprite).collect()
    }

    pub fn sprites_and_traces(&mut self) -> Vec<(&CircleComponent, &mut CircleTrace)> {
        self.planets
            .iter_mut()
            .map(|p| (&p.sprite, &mut p.trace))
            .collect()
    }
}
