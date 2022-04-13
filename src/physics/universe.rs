use crate::physics::force::ForceSystem;
use crate::physics::gravity::GravitySystem;
use crate::physics::motion::MotionSystem;
use hecs::World;
use piston::input::UpdateArgs;

const ACCELERATIONS: [f64; 14] = [
    1.0e1, 1.0e2, 2.5e2, 5.0e2, 7.5e2, 1.0e3, 2.5e3, 5.0e3, 7.5e3, 1.0e4, 2.5e4, 5.0e4, 7.5e4,
    1.0e5,
];

pub struct Universe {
    pub acceleration: f64,
    motion: MotionSystem,
    gravity: GravitySystem,
    force: ForceSystem,
    selected_acceleration: usize,
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            acceleration: ACCELERATIONS[9],
            motion: MotionSystem::default(),
            gravity: GravitySystem::default(),
            force: ForceSystem::default(),
            selected_acceleration: 9,
        }
    }

    #[allow(dead_code)]
    pub fn acceleration(&self) -> f64 {
        self.acceleration
    }

    pub fn change_acceleration(&mut self, new_acceleration: f64) {
        self.acceleration = new_acceleration;
    }

    pub fn speed_up(&mut self) {
        if self.selected_acceleration < ACCELERATIONS.len() - 1 {
            self.selected_acceleration += 1;
            self.change_acceleration(ACCELERATIONS[self.selected_acceleration]);
        }
    }

    pub fn slow_down(&mut self) {
        if self.selected_acceleration > 0 {
            self.selected_acceleration -= 1;
            self.change_acceleration(ACCELERATIONS[self.selected_acceleration]);
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.acceleration > 0.0 {
            self.acceleration = 0.0;
        } else {
            self.change_acceleration(ACCELERATIONS[self.selected_acceleration]);
        }
    }

    pub fn update(&mut self, args: UpdateArgs, world: &mut World) {
        let dt = args.dt * self.acceleration;
        self.force.reset(world);
        self.gravity.update(world);
        self.force.update(world);
        self.motion.update(world, dt);
    }
}
