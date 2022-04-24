use crate::physics::force::{Force, ForceComponent};
use crate::physics::motion::{distance_between, Acceleration, Motion, Position};
use hecs::{Entity, World};
use std::collections::HashMap;
use vecmath;

const G: f64 = 6.674e-11;
pub type Mass = f64;

pub struct MassComponent {
    pub mass: Mass,
}

impl MassComponent {
    pub fn new(mass: Mass) -> Self {
        MassComponent { mass }
    }
}

struct GravityCalculation {
    pub mass: Mass,
    pub position: Position,
    entity: Entity,
}

impl GravityCalculation {
    pub fn acceleration(&self, rhs: &GravityCalculation) -> Force {
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

fn accelerations(bodies: &[GravityCalculation]) -> HashMap<Entity, Acceleration> {
    let mut matrix = HashMap::new();

    for x in bodies.iter() {
        let mut acceleration: Acceleration = [0.0, 0.0];
        for y in bodies.iter() {
            acceleration = vecmath::vec2_add(acceleration, x.acceleration(y));
        }
        matrix.insert(x.entity, acceleration);
    }

    matrix
}

pub struct GravitySystem {}

impl Default for GravitySystem {
    fn default() -> Self {
        GravitySystem::new()
    }
}

impl GravitySystem {
    pub fn new() -> Self {
        GravitySystem {}
    }

    pub fn update(&mut self, world: &mut World) {
        let mut gravities: Vec<GravityCalculation> = vec![];
        for (id, (g, m)) in &mut world.query::<(&MassComponent, &Motion)>() {
            gravities.push(GravityCalculation {
                position: m.position,
                mass: g.mass,
                entity: id,
            });
        }
        let acceleration_values = accelerations(&gravities);
        for (id, (force_component,)) in &mut world.query::<(&mut ForceComponent,)>() {
            match acceleration_values.get(&id) {
                Some(accel) => {
                    force_component.force = *accel;
                }
                None => {}
            }
        }
    }
}
