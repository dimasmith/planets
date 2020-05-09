use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use crate::core::events::EventLoop;
use crate::core::{gl, text, world};
use crate::loader::loader::ToEntityBuilder;
use crate::loader::stage::LoadingStage;
use crate::model::{Background, Planet};
use crate::simulation::SimulationStage;

pub mod core;
pub mod loader;
pub mod model;
pub mod physics;
pub mod render;
pub mod simulation;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("n-Body Simulation", [1920, 1080])
        .graphics_api(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gl = gl::create(opengl);
    let glyphs = text::create();
    let world = world::create();
    let events = Events::new(EventSettings::new());
    let mut event_loop = EventLoop::new(events);

    let mut loading_stage =
        LoadingStage::new(gl.clone(), glyphs.clone(), world.clone(), load_models());
    let mut simulation_stage = SimulationStage::new(gl.clone(), glyphs.clone(), world.clone());

    event_loop.activate_stage(&mut loading_stage, &mut window);
    event_loop.activate_stage(&mut simulation_stage, &mut window);
}

fn load_models() -> Vec<Box<dyn ToEntityBuilder>> {
    let kerbin = Box::new(Planet {
        name: "Kerbin",
        position: [0.0, 0.0],
        velocity: [0.0, 0.0],
        mass: 5.2915158e22,
        visible_radius: 48.0,
        image: "kerbin",
    });

    let mun = Box::new(Planet {
        name: "Mun",
        position: [-12.0e6, 0.0],
        velocity: [0.0, 543.0],
        mass: 9.7599066e20,
        visible_radius: 24.0,
        image: "mun",
    });

    let minmus = Box::new(Planet {
        name: "Minmus",
        position: [47.0e6, 0.0],
        velocity: [0.0, -274.0],
        mass: 2.645758e19,
        visible_radius: 16.0,
        image: "minmus",
    });

    let phobos = Box::new(Planet {
        name: "Phobos",
        position: [-47e6, 0.0],
        velocity: [0.0, 247.0],
        mass: 2.645758e19,
        visible_radius: 16.0,
        image: "phobos",
    });

    let deimos = Box::new(Planet {
        name: "Deimos",
        position: [0.0, -47e6],
        velocity: [-247.0, 0.0],
        mass: 2.645758e19,
        visible_radius: 16.0,
        image: "deimos",
    });

    let background = Box::new(Background { image: "nebula" });

    vec![kerbin, mun, minmus, phobos, deimos, background]
}
