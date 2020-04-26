use hecs::World;
use vecmath;

pub type Position = vecmath::Vector2<f64>;
pub type Velocity = vecmath::Vector2<f64>;
pub type Acceleration = vecmath::Vector2<f64>;

pub struct Motion {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

impl Motion {
    pub fn new() -> Self {
        Motion {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    pub fn new_pos(position: Position) -> Self {
        Motion {
            position,
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    pub fn new_position_velocity(position: Position, velocity: Velocity) -> Self {
        Motion {
            position,
            velocity,
            acceleration: [0.0, 0.0],
        }
    }
}

pub fn distance_between(lhs: &Position, rhs: &Position) -> f64 {
    f64::sqrt((rhs[0] - lhs[0]) * (rhs[0] - lhs[0]) + (rhs[1] - lhs[1]) * (rhs[1] - lhs[1]))
}

pub struct MotionSystem {}

impl MotionSystem {
    pub fn new() -> Self {
        MotionSystem {}
    }

    pub fn update(&mut self, world: &mut World, dt: f64) {
        for (id, (motion)) in &mut world.query::<(&mut Motion)>() {
            self.advance(motion, dt);
        }
    }

    fn advance(&self, motion: &mut Motion, dt: f64) {
        let da = vecmath::vec2_scale(motion.acceleration, dt);
        motion.velocity = vecmath::vec2_add(da, motion.velocity);
        let dv = vecmath::vec2_scale(motion.velocity, dt);
        motion.position = vecmath::vec2_add(motion.position, dv);
    }
}
