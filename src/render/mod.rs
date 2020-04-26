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
            camera_system: CameraSystem::new(Camera::new(400.0 / 47.0 * 1.0e-6)),
            circle_system: CircleSystem::new(),
            circle_trace_system: CircleTraceSystem::new(),
        }
    }

    pub fn camera(gl: GlGraphics, camera: Camera) -> Renderer {
        Renderer {
            gl,
            camera_system: CameraSystem::new(camera),
            circle_system: CircleSystem::new(),
            circle_trace_system: CircleTraceSystem::new(),
        }
    }

    pub fn camera_as_mut(&mut self) -> &mut Camera {
        &mut self.camera_system.camera
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let mut gl = &mut self.gl;

        let mut context = gl.draw_begin(args.viewport());
        context = self.camera_system.update(context, world, args);

        // clear the screen
        graphics::clear(BACK, gl);

        self.circle_trace_system.update(world, context, gl);
        self.circle_system.update(world, context, gl);

        gl.draw_end();
    }
}
