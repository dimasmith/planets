use piston::input::{
    Button, ButtonEvent, ButtonState, Event, Key, MouseScrollEvent, RenderEvent, UpdateEvent,
};

use crate::core::events::EventHandler;
use crate::core::gl::SharedGraphics;
use crate::core::text::SharedGlyphCache;
use crate::core::world::SharedWorld;
use crate::physics::universe::Universe;
use crate::render::camera::Camera;
use crate::render::renderer::Renderer;

pub struct SimulationStage<'a> {
    renderer: Renderer<'a>,
    universe: Universe,
    world: SharedWorld,
}

impl<'a> SimulationStage<'a> {
    pub fn new(gl: SharedGraphics, glyphs: SharedGlyphCache<'a>, world: SharedWorld) -> Self {
        let camera = Camera::fixed(400.0 / 47.0 * 1.0e-6);

        let renderer = Renderer::camera(gl.clone(), camera, glyphs.clone());
        let universe = Universe::new();
        SimulationStage {
            renderer,
            universe,
            world,
        }
    }
}

impl<'a> EventHandler for SimulationStage<'a> {
    fn handle_event(&mut self, e: Event) -> bool {
        let renderer = &mut self.renderer;
        let universe = &mut self.universe;
        let world = &mut (self.world).borrow_mut();
        if let Some(args) = e.render_args() {
            renderer.render(args, world);
        }

        if let Some(args) = e.update_args() {
            universe.update(args, world);
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
        if let Some(args) = e.button_args() {
            match args.button {
                Button::Keyboard(key) => match key {
                    Key::Comma => {
                        universe.slow_down();
                    }
                    Key::Period => {
                        universe.speed_up();
                    }
                    Key::P => {
                        if args.state == ButtonState::Press {
                            universe.toggle_pause();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        };
        false
    }
}
