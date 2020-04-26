use crate::physics::motion::{Motion, Position};
use crate::render::circle::CircleComponent;
use graphics::{Context, Transformed};
use hecs::World;
use piston::input::RenderArgs;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub zoom: f64,
    pub focus: Position,
}

impl Camera {
    pub fn new(zoom: f64) -> Self {
        Camera {
            zoom,
            focus: [0.0, 0.0],
        }
    }

    /// Project physical position into the rendering position
    pub fn project(&self, coords: Position) -> Position {
        vecmath::vec2_scale(coords, self.zoom)
    }

    /// Updates camera position
    pub fn change_focus(&mut self, args: RenderArgs) {
        let x = args.window_size[0] / 2.0;
        let y = args.window_size[1] / 2.0;
        self.focus = [x, y];
    }
}

pub struct CameraSystem {
    camera: Camera,
}

impl CameraSystem {
    pub fn new(camera: Camera) -> CameraSystem {
        CameraSystem { camera }
    }

    pub fn update(&mut self, context: Context, world: &mut World, args: RenderArgs) -> Context {
        for (id, (sprite, motion)) in &mut world.query::<(&mut CircleComponent, &Motion)>() {
            let projected_position = self.camera.project(motion.position);
            sprite.set_position(projected_position);
        }
        self.camera.change_focus(args);
        context.trans_pos(self.camera.focus)
    }
}
