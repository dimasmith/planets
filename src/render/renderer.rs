use hecs::World;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::render::background::BackgroundSystem;
use crate::render::camera::{Camera, CameraSystem};
use crate::render::name::NameSystem;
use crate::render::sprite::SpriteSystem;
use crate::render::trace::{RenderTraceSystem, TraceSpawnSystem};
use crate::text::SharedGlyphCache;

pub struct Renderer<'r> {
    gl: &'r mut GlGraphics,
    camera_system: CameraSystem,
    circle_system: SpriteSystem,
    name_system: NameSystem,
    trace_system: RenderTraceSystem,
    trace_spawn_system: TraceSpawnSystem,
    background: BackgroundSystem,
    glyphs: SharedGlyphCache<'r>,
}

const BACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl<'r> Renderer<'r> {
    pub fn camera(gl: &'r mut GlGraphics, camera: Camera, glyphs: SharedGlyphCache<'r>) -> Self {
        Renderer {
            gl,
            camera_system: CameraSystem::new(camera),
            circle_system: SpriteSystem::new(),
            name_system: NameSystem::new(),
            trace_system: RenderTraceSystem::new(),
            trace_spawn_system: TraceSpawnSystem::new(),
            background: BackgroundSystem::new(),
            glyphs,
        }
    }

    pub fn camera_as_mut(&mut self) -> &mut Camera {
        &mut self.camera_system.camera
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let gl = &mut *self.gl;
        let glyphs = &mut self.glyphs.borrow_mut();

        let mut context = gl.draw_begin(args.viewport());
        graphics::clear(BACK, gl);
        self.background.update(world, context, gl, args);

        context = self.camera_system.update(context, world, args);

        self.trace_spawn_system.update(world);
        self.trace_system.update(world, context, gl);
        self.circle_system.update(world, context, gl);
        self.name_system.update(world, glyphs, context, gl);

        gl.draw_end();
    }
}
