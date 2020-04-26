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
    let kerbin_position = [0.0, 0.0];
    world.spawn((
        Motion::position(kerbin_position),
        MassComponent::new(5.2915158e22),
        CircleComponent::new([0.2, 0.2, 0.9, 1.0], 20.0),
        CircleTrace::new(),
        ForceComponent::zero(),
    ));

    let mun_position = [-12.0e6, 0.0];
    world.spawn((
        Motion::new_position_velocity(mun_position, [0.0, 543.0]),
        MassComponent::new(9.7599066e20),
        CircleComponent::new([0.5, 0.5, 0.5, 1.0], 10.0),
        CircleTrace::new(),
        ForceComponent::zero(),
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
