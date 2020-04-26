use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::physics::force::ForceComponent;
use crate::physics::gravity::MassComponent;
use crate::physics::motion::Motion;
use crate::physics::propulsion::PropulsionComponent;
use crate::physics::Universe;
use crate::render::circle::{CircleComponent, CircleTrace};
use crate::render::Renderer;

mod physics;
mod render;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("n-Body Simulation", [1980, 1024])
        .graphics_api(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world = hecs::World::new();
    let phobos_position = [-3.5e6, 1.0e6];
    world.spawn((
        Motion::position(phobos_position),
        MassComponent::new(1.0e8),
        CircleComponent::new([0.2, 0.2, 0.9, 1.0], phobos_position, 25.0),
        CircleTrace::new(),
        ForceComponent::zero(),
        PropulsionComponent::zero(),
    ));

    let deimos_position = [-3.5e6, -1.0e6];
    world.spawn((
        Motion::position(deimos_position),
        MassComponent::new(1.0e9),
        CircleComponent::new([0.9, 0.2, 0.2, 1.0], deimos_position, 25.0),
        CircleTrace::new(),
        ForceComponent::zero(),
        PropulsionComponent::zero(),
    ));

    let mut renderer = Renderer::new(GlGraphics::new(opengl));
    let mut universe = Universe::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            renderer.render(args, &mut world);
        }

        if let Some(args) = e.update_args() {
            universe.update(args, &mut world);
        }
    }
}
