use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{MouseScrollEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::physics::force::ForceComponent;
use crate::physics::gravity::MassComponent;
use crate::physics::motion::Motion;
use crate::physics::propulsion::PropulsionComponent;
use crate::physics::Universe;
use crate::render::camera::Camera;
use crate::render::circle::{CircleComponent, CircleTrace};
use crate::render::Renderer;
use rust_sandbox::info::NameComponent;

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
        NameComponent::new("Kerbin".to_string()),
        Motion::position(kerbin_position),
        MassComponent::new(5.2915158e22),
        CircleComponent::new([0.2, 0.2, 0.9, 1.0], 20.0),
        CircleTrace::new(),
        ForceComponent::zero(),
    ));

    let mun_position = [-12.0e6, 0.0];
    let mun = world.spawn((
        NameComponent::new("Mun".to_string()),
        Motion::new_position_velocity(mun_position, [0.0, 543.0]),
        MassComponent::new(9.7599066e20),
        CircleComponent::new([0.5, 0.5, 0.5, 1.0], 10.0),
        CircleTrace::new(),
        ForceComponent::zero(),
    ));

    let minmus_position = [47.0e6, 0.0];
    world.spawn((
        NameComponent::new("Minmus".to_string()),
        Motion::new_position_velocity(minmus_position, [0.0, -274.0]),
        MassComponent::new(2.645758e19),
        CircleComponent::new([0.5, 1.0, 0.5, 1.0], 8.0),
        CircleTrace::new(),
        ForceComponent::zero(),
    ));

    // world.spawn((
    //     NameComponent::new("Comet".to_string()),
    //     Motion::new_position_velocity([-47e6, 0.0], [0.0, 247.0 * 0.75]),
    //     MassComponent::new(2.645758e19),
    //     CircleComponent::new([1.0, 0.2, 0.2, 1.0], 8.0),
    //     CircleTrace::new(),
    //     ForceComponent::zero(),
    // ));
    //
    // world.spawn((
    //     NameComponent::new("Comet2".to_string()),
    //     Motion::new_position_velocity([0.0, -47e6], [-247.0 * 0.75, 0.0]),
    //     MassComponent::new(2.645758e19),
    //     CircleComponent::new([1.0, 1.0, 0.2, 1.0], 8.0),
    //     CircleTrace::new(),
    //     ForceComponent::zero(),
    // ));

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
                let mut camera = renderer.camera_as_mut();
                camera.zoom_out();
            }
            if args[1] > 0.0 {
                let mut camera = renderer.camera_as_mut();
                camera.zoom_in();
            }
        }
    }
}
