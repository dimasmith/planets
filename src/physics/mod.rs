use crate::physics::gravity::GravitySystem;
use crate::physics::motion::MotionSystem;
use hecs::World;
use piston::input::UpdateArgs;

pub mod gravity;
pub mod motion;

pub struct Universe {
    pub acceleration: f64,
    motion: MotionSystem,
    gravity: GravitySystem,
}
impl Universe {
    pub fn new() -> Self {
        Universe {
            acceleration: 10.0,
            motion: MotionSystem::new(),
            gravity: GravitySystem::new(),
        }
    }

    pub fn update(&mut self, args: UpdateArgs, world: &mut World) {
        let dt = args.dt * self.acceleration;
        self.gravity.update(world, dt);
        self.motion.update(world, dt);
    }
}
