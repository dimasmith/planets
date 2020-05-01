use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{MouseScrollEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::loader::loader::{ModelLoader, Planet, ToEntityBuilder};
use crate::loader::screen::LoadingScreen;
use crate::loader::state::LoadingState;
use crate::physics::force::ForceComponent;
use crate::physics::gravity::MassComponent;
use crate::physics::motion::{Motion, Velocity};
use crate::physics::universe::Universe;
use crate::render::camera::Camera;
use crate::render::name::NameComponent;
use crate::render::render_box::RenderBoxComponent;
use crate::render::renderer::Renderer;
use crate::render::sprite::Sprite;
use crate::render::trace::SpawnTraceSystem;
use image::io::Reader;
use std::borrow::Borrow;

mod loader;
mod physics;
mod render;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("n-Body Simulation", [1920, 1080])
        .graphics_api(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut loading_state = LoadingState::new();
    let models = load_models();
    let mut model_loader = ModelLoader::new(models);
    let mut loading_screen = LoadingScreen::new();
    let mut world = hecs::World::new();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            loading_screen.render(&loading_state, args, &mut gl);
        }
        if let Some(args) = e.update_args() {
            model_loader.update(&mut loading_state, &mut world);
        }

        if loading_state.done() {
            break;
        }
    }

    // let camera = Camera::tracking(400.0 / 47.0 * 1.0e-6, kerbin);
    let camera = Camera::fixed(400.0 / 47.0 * 1.0e-6);

    let mut renderer = Renderer::camera(&mut gl, camera);
    let mut universe = Universe::new();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            renderer.render(args, &mut world);
        }

        if let Some(args) = e.update_args() {
            universe.update(args, &mut world);
        }
        if let Some(args) = e.mouse_scroll_args() {
            if args[1] < 0.0 {
                let camera = renderer.camera_as_mut();
                camera.zoom_out();
            }
            if args[1] > 0.0 {
                let camera = renderer.camera_as_mut();
                camera.zoom_in();
            }
        }
    }
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
        visible_radius: 16.0,
        image: "mun",
    });

    let minmus = Box::new(Planet {
        name: "Minmus",
        position: [47.0e6, 0.0],
        velocity: [0.0, -274.0],
        mass: 2.645758e19,
        visible_radius: 8.0,
        image: "minmus",
    });

    let phobos = Box::new(Planet {
        name: "Phobos",
        position: [-47e6, 0.0],
        velocity: [0.0, 247.0 * 0.75],
        mass: 2.645758e19,
        visible_radius: 8.0,
        image: "minmus",
    });

    let deimos = Box::new(Planet {
        name: "Deimos",
        position: [0.0, -47e6],
        velocity: [-247.0 * 0.75, 0.0],
        mass: 2.645758e19,
        visible_radius: 8.0,
        image: "deimos",
    });

    vec![kerbin, mun, minmus, phobos, deimos]
}
