use crate::physics::force::{Force, ForceComponent};
use hecs::World;

pub struct PropulsionComponent {
    pub force: Force,
}

impl PropulsionComponent {
    pub fn new(force: Force) -> Self {
        PropulsionComponent { force }
    }

    pub fn zero() -> Self {
        PropulsionComponent { force: [0.0, 0.0] }
    }
}

pub struct PropulsionSystem {}

impl PropulsionSystem {
    pub fn new() -> Self {
        PropulsionSystem {}
    }

    pub fn update(&self, world: &mut World) {
        for (id, (propulsion_component, force_component)) in
            &mut world.query::<(&PropulsionComponent, &mut ForceComponent)>()
        {
            force_component.force =
                vecmath::vec2_add(force_component.force, propulsion_component.force)
        }
    }
}
