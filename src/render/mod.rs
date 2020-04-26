use graphics::Context;
use hecs::World;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

use crate::physics::motion::Position;
use crate::render::camera::{Camera, CameraSystem};
use crate::render::circle::{CircleSystem, CircleTraceSystem};

pub mod camera;
pub mod circle;

pub struct Renderer {
    pub gl: GlGraphics,
    camera_system: CameraSystem,
    circle_system: CircleSystem,
    circle_trace_system: CircleTraceSystem,
}

const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

impl Renderer {
    pub fn new(gl: GlGraphics) -> Renderer {
        Renderer {
            gl,
            camera_system: CameraSystem::new(Camera::new(400.0 / 12.0 * 1.0e-6)),
            circle_system: CircleSystem::new(),
            circle_trace_system: CircleTraceSystem::new(),
        }
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let mut gl = &mut self.gl;

        let mut context = gl.draw_begin(args.viewport());
        context = self.camera_system.update(context, world, args);

        // clear the screen
        graphics::clear(BACK, gl);

        self.circle_trace_system.update(world, context, gl);
        self.circle_system.update(world, context, gl);

        // let mut center = GeoCenter::new(&world);
        // center.render_all(&projector, &mut context, &mut gl);

        gl.draw_end();
    }
}

// struct GeoCenter {
//     position: Position,
// }
//
// impl GeoCenter {
//     fn new(world: &World) -> GeoCenter {
//         let mut position = world
//             .planets
//             .iter()
//             .map(|p| p.motion.position)
//             .fold([0.0, 0.0], |a, p| vecmath::vec2_add(a, p));
//         position = vecmath::vec2_scale(position, 1.0 / world.planets.len() as f64);
//         GeoCenter { position }
//     }
// }
//
// impl Renderable for GeoCenter {
//     fn render(&mut self, projector: &Projector, context: &mut Context, gl: &mut GlGraphics) {
//         let position: Position = projector.project(&self.position);
//         let bound = graphics::rectangle::centered_square(position[0], position[1], 10.0);
//         graphics::ellipse([1.0, 0.0, 0.0, 1.0], bound, context.transform, gl);
//     }
// }
