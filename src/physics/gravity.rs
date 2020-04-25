use crate::physics::motion::{distance_between, Acceleration, Position};
use vecmath;

const G: f64 = 1.0;
pub type Mass = f64;

pub struct Gravity {
    pub mass: Mass,
    pub position: Position,
}

impl Gravity {
    pub fn acceleration(&self, rhs: &Gravity) -> Acceleration {
        let distance = distance_between(&self.position, &rhs.position);
        if distance == 0.0 {
            return [0.0, 0.0];
        }
        let distance_squared = distance * distance;
        let mass_product = self.mass * rhs.mass;
        let force = G * mass_product / distance_squared;
        let direction = vecmath::vec2_normalized(vecmath::vec2_sub(rhs.position, self.position));
        vecmath::vec2_scale(direction, force)
    }
}

pub fn accelerations(bodies: &Vec<Gravity>) -> Vec<Acceleration> {
    let mut matrix: Vec<Acceleration> = vec![];

    for x in bodies.iter() {
        let mut acceleration: Acceleration = [0.0, 0.0];
        for y in bodies.iter() {
            acceleration = vecmath::vec2_add(acceleration, x.acceleration(y));
        }
        matrix.push(acceleration);
    }

    return matrix;
}
