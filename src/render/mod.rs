use graphics::Context;
use hecs::World;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

use crate::physics::motion::Position;
use crate::render::camera::{Camera, CameraSystem};
use crate::render::circle::{CircleSystem, CircleTraceSystem};
use crate::render::name::NameSystem;
use graphics::character::CharacterCache;

pub mod camera;
pub mod circle;
pub mod name;

pub struct Renderer<'r> {
    pub gl: GlGraphics,
    camera_system: CameraSystem,
    circle_system: CircleSystem,
    circle_trace_system: CircleTraceSystem,
    name_system: NameSystem,
    glyphs: GlyphCache<'r>,
}

const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

impl Renderer<'_> {
    pub fn camera(gl: GlGraphics, camera: Camera) -> Renderer<'static> {
        Renderer {
            gl,
            camera_system: CameraSystem::new(camera),
            circle_system: CircleSystem::new(),
            circle_trace_system: CircleTraceSystem::new(),
            name_system: NameSystem::new(),
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

        self.circle_trace_system.update(world, context, gl);
        self.circle_system.update(world, context, gl);
        self.name_system.update(world, glyphs, context, gl);

        gl.draw_end();
    }
}
