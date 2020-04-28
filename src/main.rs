use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{MouseScrollEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::physics::force::ForceComponent;
use crate::physics::gravity::MassComponent;
use crate::physics::motion::Motion;
use crate::physics::Universe;
use crate::render::camera::Camera;
use crate::render::circle::CircleComponent;
use crate::render::name::NameComponent;
use crate::render::render_box::RenderBoxComponent;
use crate::render::trace::SpawnTraceSystem;
use crate::render::Renderer;
use rusttype::Font;

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

    let mut world = hecs::World::new();
    let kerbin_position = [0.0, 0.0];
    let kerbin = world.spawn((
        Motion::position(kerbin_position),
        NameComponent::new("Kerbin"),
        MassComponent::new(5.2915158e22),
        CircleComponent::new([0.2, 0.2, 0.9, 1.0]),
        ForceComponent::zero(),
        RenderBoxComponent::new(),
    ));

    let mun_position = [-12.0e6, 0.0];
    let mun = world.spawn((
        Motion::new_position_velocity(mun_position, [0.0, 543.0]),
        NameComponent::new("Mun"),
        MassComponent::new(9.7599066e20),
        CircleComponent::new([0.5, 0.5, 0.5, 1.0]),
        ForceComponent::zero(),
        RenderBoxComponent::centered_square(10.0),
        SpawnTraceSystem::new(),
    ));

    let minmus_position = [47.0e6, 0.0];
    world.spawn((
        Motion::new_position_velocity(minmus_position, [0.0, -274.0]),
        NameComponent::new("Minmus"),
        MassComponent::new(2.645758e19),
        CircleComponent::new([0.5, 1.0, 0.5, 1.0]),
        ForceComponent::zero(),
        RenderBoxComponent::centered_square(8.0),
        SpawnTraceSystem::new(),
    ));

    world.spawn((
        NameComponent::new("Phobos"),
        Motion::new_position_velocity([-47e6, 0.0], [0.0, 247.0 * 0.75]),
        MassComponent::new(2.645758e19),
        CircleComponent::new([1.0, 0.2, 0.2, 1.0]),
        ForceComponent::zero(),
        RenderBoxComponent::centered_square(8.0),
        SpawnTraceSystem::new(),
    ));

    world.spawn((
        NameComponent::new("Deimos"),
        Motion::new_position_velocity([0.0, -47e6], [-247.0 * 0.75, 0.0]),
        MassComponent::new(2.645758e19),
        CircleComponent::new([1.0, 1.0, 0.2, 1.0]),
        ForceComponent::zero(),
        RenderBoxComponent::centered_square(8.0),
        SpawnTraceSystem::new(),
    ));

    let mut camera = Camera::tracking(400.0 / 47.0 * 1.0e-6, kerbin);
    let mut renderer = Renderer::camera(GlGraphics::new(opengl), camera);
    let mut universe = Universe::new();

    let mut events = Events::new(EventSettings::new());
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
