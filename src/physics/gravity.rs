use crate::model::World;
use crate::physics::motion::{distance_between, Acceleration, Position};
use vecmath;

const G: f64 = 1.0;
pub type Mass = f64;

pub struct Gravity {
    mass: Mass,
}

impl Gravity {
    pub fn new(mass: Mass) -> Self {
        Gravity { mass }
    }
}

struct GravityCalculation {
    pub mass: Mass,
    pub position: Position,
}

impl GravityCalculation {
    pub fn acceleration(&self, rhs: &GravityCalculation) -> Acceleration {
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

fn accelerations(bodies: &Vec<GravityCalculation>, dt: f64) -> Vec<Acceleration> {
    let mut matrix: Vec<Acceleration> = vec![];

    for x in bodies.iter() {
        let mut acceleration: Acceleration = [0.0, 0.0];
        for y in bodies.iter() {
            acceleration = vecmath::vec2_add(acceleration, x.acceleration(y));
        }
        acceleration = vecmath::vec2_scale(acceleration, dt);
        matrix.push(acceleration);
    }

    return matrix;
}

pub struct GravitySystem {}
impl GravitySystem {
    pub fn new() -> Self {
        GravitySystem {}
    }

    pub fn update(&mut self, world: &mut World, dt: f64) {
        let gravities: Vec<GravityCalculation> = world
            .gravity()
            .iter()
            .map(|g| GravityCalculation {
                mass: g.1.mass,
                position: g.0.position,
            })
            .collect();
        let accels = accelerations(&gravities, dt);
        for (i, motion) in world.motions().iter_mut().enumerate() {
            motion.acceleration = accels[i];
        }
    }
}
