use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

use crate::physics::gravity::{accelerations, Gravity};
use crate::physics::motion::Motionable;
use crate::render::{Renderable, Renderer};
use model::planet;
use std::borrow::BorrowMut;

mod model;
mod physics;
mod render;

struct Universe {
    planets: Vec<planet::Planet>,
    renderer: Renderer,
}
const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
impl Universe {
    fn render(&mut self, args: RenderArgs) {
        self.renderer.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BACK, gl);
        });

        for planet in self.planets.iter_mut() {
            planet.render(&mut self.renderer, args);
        }
    }

    fn update(&mut self, args: UpdateArgs) {
        let gravities: Vec<Gravity> = self.planets.iter().map(|b| b.gravity()).collect();
        let accels = accelerations(&gravities);

        for (i, planet) in self.planets.iter_mut().enumerate() {
            planet.motion.set_acceleration(accels[i]);
        }

        for body in self.planets.iter_mut() {
            body.motion.advance(args.dt * 5.0);
        }
    }

    fn handle_input(&mut self, args: ButtonArgs) {}
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("n-Body Simulation", [1980, 1024])
        .graphics_api(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let mars = planet::Planet::new("Mars".to_string(), (0.0, 0.0));
    let mut deimos = planet::Planet::new("Deimos".to_string(), (1.0e6, 0.0));
    let mut phobos = planet::Planet::new("Phobos".to_string(), (-1.0e6, 0.0));
    phobos.motion.velocity = [0.0, 0.0];
    deimos.motion.velocity = [0.0, -0.4e5];

    let mut scene = Universe {
        planets: vec![phobos, deimos],
        renderer: Renderer::new(GlGraphics::new(opengl)),
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            scene.render(args);
        }

        if let Some(args) = e.update_args() {
            scene.update(args);
        }

        if let Some(args) = e.button_args() {
            scene.handle_input(args);
        }
    }
}
