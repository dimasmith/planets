use graphics::Context;
use hecs::World;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

use crate::physics::motion::Position;
use crate::render::camera::{Camera, CameraSystem};
use crate::render::circle::CircleSystem;
use crate::render::name::NameSystem;
use crate::render::trace::{RenderTraceSystem, TraceSpawnSystem};
use graphics::character::CharacterCache;

pub mod camera;
pub mod circle;
pub mod name;
pub mod render_box;
pub mod trace;

pub struct Renderer<'r> {
    pub gl: GlGraphics,
    camera_system: CameraSystem,
    circle_system: CircleSystem,
    name_system: NameSystem,
    trace_system: RenderTraceSystem,
    trace_spawn_system: TraceSpawnSystem,
    glyphs: GlyphCache<'r>,
}

const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

impl Renderer<'_> {
    pub fn camera(gl: GlGraphics, camera: Camera) -> Renderer<'static> {
        Renderer {
            gl,
            camera_system: CameraSystem::new(camera),
            circle_system: CircleSystem::new(),
            name_system: NameSystem::new(),
            trace_system: RenderTraceSystem::new(),
            trace_spawn_system: TraceSpawnSystem::new(),
            glyphs: Renderer::character_cache(),
        }
    }

    fn character_cache() -> GlyphCache<'static> {
        let font_data = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        GlyphCache::from_bytes(font_data, (), texture_settings).expect("could not load font")
    }

    pub fn camera_as_mut(&mut self) -> &mut Camera {
        &mut self.camera_system.camera
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let mut gl = &mut self.gl;
        let glyphs = &mut self.glyphs;

        let mut context = gl.draw_begin(args.viewport());
        context = self.camera_system.update(context, world, args);

        // clear the screen
        graphics::clear(BACK, gl);

        self.trace_spawn_system.update(world);
        self.trace_system.update(world, context, gl);
        self.circle_system.update(world, context, gl);
        self.name_system.update(world, glyphs, context, gl);

        gl.draw_end();
    }
}
