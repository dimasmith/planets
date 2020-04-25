use crate::model::World;
use crate::physics::gravity::{accelerations, Gravity};
use crate::physics::motion::Motionable;
use piston::input::UpdateArgs;

pub mod gravity;
pub mod motion;

pub struct Universe {
    pub acceleration: f64,
}
impl Universe {
    pub fn update(&mut self, args: UpdateArgs, world: &mut World) {
        let gravities: Vec<Gravity> = world.planets.iter().map(|b| b.gravity()).collect();
        let accels = accelerations(&gravities);

        for (i, planet) in world.planets.iter_mut().enumerate() {
            planet.motion.set_acceleration(accels[i]);
        }

        for body in world.planets.iter_mut() {
            body.motion.advance(args.dt * self.acceleration);
        }
    }
}
