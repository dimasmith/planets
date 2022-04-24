use crate::physics::gravity::MassComponent;
use crate::physics::motion::Motion;
use hecs::World;
use vecmath::Vector2;

pub type Force = Vector2<f64>;

pub struct ForceComponent {
    pub force: Force,
}

impl ForceComponent {
    pub fn zero() -> Self {
        ForceComponent { force: [0.0, 0.0] }
    }
}

pub struct ForceSystem {}

impl Default for ForceSystem {
    fn default() -> Self {
        ForceSystem::new()
    }
}

impl ForceSystem {
    pub fn new() -> Self {
        ForceSystem {}
    }

    pub fn reset(&mut self, world: &mut World) {
        // reset forces at the beginning
        for (_id, (force,)) in &mut world.query::<(&mut ForceComponent,)>() {
            force.force = [0.0, 0.0];
        }
    }

    pub fn update(&mut self, world: &mut World) {
        // reset forces at the beginning
        for (_id, (force_component, mass_component, motion)) in
            &mut world.query::<(&ForceComponent, &MassComponent, &mut Motion)>()
        {
            let acceleration =
                vecmath::vec2_scale(force_component.force, 1.0 / mass_component.mass);
            motion.acceleration = acceleration;
        }
    }
}
