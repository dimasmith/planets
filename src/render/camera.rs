use crate::physics::motion::{Motion, Position};
use crate::render::render_box::RenderBoxComponent;
use graphics::{Context, Transformed};
use hecs::{Entity, World};
use piston::input::RenderArgs;
use std::collections::HashMap;

pub enum TrackingMode {
    Tracking(Entity),
}

pub struct Camera {
    pub zoom: f64,
    zoom_step: f64,
    pub focus: Position,
    pub tracking: TrackingMode,
}

impl Camera {
    pub fn tracking(zoom: f64, entity: Entity) -> Self {
        Camera {
            zoom,
            zoom_step: zoom / 16.0,
            focus: [0.0, 0.0],
            tracking: TrackingMode::Tracking(entity),
        }
    }

    pub fn project(&self, coords: Position) -> Position {
        vecmath::vec2_scale(coords, self.zoom)
    }

    pub fn zoom_in(&mut self) {
        self.zoom += self.zoom_step;
    }

    pub fn zoom_out(&mut self) {
        self.zoom -= self.zoom_step;
    }
}

pub struct CameraSystem {
    pub camera: Camera,
}

impl CameraSystem {
    pub fn new(camera: Camera) -> CameraSystem {
        CameraSystem { camera }
    }

    pub fn update(&mut self, context: Context, world: &mut World, args: RenderArgs) -> Context {
        let mut projected_positions_by_entity: HashMap<Entity, Position> = HashMap::new();
        for (id, (motion, render_box)) in &mut world.query::<(&Motion, &mut RenderBoxComponent)>() {
            let projected_position = self.camera.project(motion.position);
            render_box.move_to(projected_position);
            projected_positions_by_entity.insert(id, projected_position);
        }

        let screen_center = vecmath::vec2_scale(args.window_size, 0.5);
        match self.camera.tracking {
            TrackingMode::Tracking(e) => {
                let entity_position = projected_positions_by_entity[&e];
                let focus = vecmath::vec2_sub(screen_center, entity_position);
                self.camera.focus = focus;
            }
        }
        context.trans_pos(self.camera.focus)
    }
}
