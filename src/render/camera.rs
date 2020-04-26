use crate::physics::motion::{Motion, Position};
use crate::render::circle::CircleComponent;
use graphics::{Context, Transformed};
use hecs::{Entity, World};
use piston::input::RenderArgs;
use std::collections::HashMap;

pub enum CameraTracking {
    Fixed,
    Tracking(Entity),
}

pub struct Camera {
    pub zoom: f64,
    pub focus: Position,
    pub tracking: CameraTracking,
}

impl Camera {
    pub fn new(zoom: f64) -> Self {
        Camera {
            zoom,
            focus: [0.0, 0.0],
            tracking: CameraTracking::Fixed,
        }
    }

    pub fn tracking(zoom: f64, entity: Entity) -> Self {
        Camera {
            zoom,
            focus: [0.0, 0.0],
            tracking: CameraTracking::Tracking(entity),
        }
    }

    /// Project physical position into the rendering position
    pub fn project(&self, coords: Position) -> Position {
        vecmath::vec2_scale(coords, self.zoom)
    }

    pub fn zoom_in(&mut self) {
        self.zoom = self.zoom * 2.0;
    }

    pub fn zoom_out(&mut self) {
        self.zoom = self.zoom * 0.5;
    }

    /// Updates camera position
    pub fn change_focus(&mut self, args: RenderArgs) {
        let x = args.window_size[0] / 2.0;
        let y = args.window_size[1] / 2.0;
        self.focus = [x, y];
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
        let mut projected_positions_by_entity = HashMap::new();
        for (id, (sprite, motion)) in &mut world.query::<(&mut CircleComponent, &Motion)>() {
            let projected_position = self.camera.project(motion.position);
            sprite.set_position(projected_position);
            projected_positions_by_entity.insert(id, projected_position);
        }

        let screen_center = vecmath::vec2_scale(args.window_size, 0.5);
        match self.camera.tracking {
            CameraTracking::Fixed => {
                self.camera.focus = screen_center;
            }
            CameraTracking::Tracking(e) => {
                let entity_position = *projected_positions_by_entity.get(&e).expect("error");
                let focus = vecmath::vec2_sub(screen_center, entity_position);
                self.camera.focus = focus;
            }
        }
        context.trans_pos(self.camera.focus)
    }
}
