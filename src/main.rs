use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use crate::core::events::EventLoop;
use crate::core::{gl, text, world};
use crate::loader::stage::LoadingStage;
use crate::model::Simulation;
use crate::simulation::SimulationStage;
use assets_manager::AssetCache;

pub mod core;
pub mod loader;
pub mod model;
pub mod physics;
pub mod render;
pub mod simulation;

fn main() {
    let assets_cache = AssetCache::new("assets").unwrap();
    let asset_lock = assets_cache.load::<Simulation>("simulation").unwrap();
    let simulation = asset_lock.read();

    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("n-Body Simulation", [1920, 1080])
        .graphics_api(opengl)
        .vsync(true)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gl = gl::create(opengl);
    let glyphs = text::create();
    let world = world::create();
    let events = Events::new(EventSettings::new());
    let mut event_loop = EventLoop::new(events);

    let mut loading_stage = LoadingStage::new(
        gl.clone(),
        glyphs.clone(),
        world.clone(),
        simulation.models(),
    );
    let mut simulation_stage = SimulationStage::new(gl.clone(), glyphs.clone(), world.clone());

    event_loop.activate_stage(&mut loading_stage, &mut window);
    event_loop.activate_stage(&mut simulation_stage, &mut window);
}
