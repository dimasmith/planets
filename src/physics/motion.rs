use vecmath;

pub type Position = vecmath::Vector2<f64>;
pub type Velocity = vecmath::Vector2<f64>;
pub type Acceleration = vecmath::Vector2<f64>;

#[derive(Debug)]
pub struct Motion {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

pub trait Motionable {
    fn advance(&mut self, delta: f64);

    fn set_acceleration(&mut self, acceleration: Acceleration);
}

impl Motionable for Motion {
    fn advance(&mut self, delta: f64) {
        let da = vecmath::vec2_scale(self.acceleration, delta);
        self.velocity = vecmath::vec2_add(da, self.velocity);
        let dv = vecmath::vec2_scale(self.velocity, delta);
        self.position = vecmath::vec2_add(self.position, dv);
    }

    fn set_acceleration(&mut self, acceleration: [f64; 2]) {
        self.acceleration = acceleration;
    }
}

pub fn distance_between(lhs: &Position, rhs: &Position) -> f64 {
    f64::sqrt((rhs[0] - lhs[0]) * (rhs[0] - lhs[0]) + (rhs[1] - lhs[1]) * (rhs[1] - lhs[1]))
}
