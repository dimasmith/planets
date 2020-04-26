use crate::physics::force::ForceSystem;
use crate::physics::gravity::GravitySystem;
use crate::physics::motion::MotionSystem;
use crate::physics::propulsion::PropulsionSystem;
use hecs::World;
use piston::input::UpdateArgs;

pub mod force;
pub mod gravity;
pub mod motion;
pub mod propulsion;

pub struct Universe {
    pub acceleration: f64,
    motion: MotionSystem,
    gravity: GravitySystem,
    force: ForceSystem,
    propulsion: PropulsionSystem,
}
impl Universe {
    pub fn new() -> Self {
        Universe {
            acceleration: 5.0e3,
            motion: MotionSystem::new(),
            gravity: GravitySystem::new(),
            force: ForceSystem::new(),
            propulsion: PropulsionSystem::new(),
        }
    }

    pub fn update(&mut self, args: UpdateArgs, world: &mut World) {
        let dt = args.dt * self.acceleration;
        self.force.reset(world);
        self.gravity.update(world);
        self.propulsion.update(world);
        self.force.update(world);
        self.motion.update(world, dt);
    }
}
