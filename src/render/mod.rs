use crate::model::World;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

pub struct Renderer {
    pub zoom: f64,
    pub gl: GlGraphics,
}
const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
impl Renderer {
    pub fn new(gl: GlGraphics) -> Renderer {
        Renderer { zoom: 2.5e-4, gl }
    }

    pub fn project(&self, coords: &Vector2<f64>) -> Vector2<f64> {
        vecmath::vec2_scale(*coords, self.zoom)
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACK, gl);
        });

        for planet in world.planets.iter_mut() {
            planet.render(self, args);
        }
    }
}

pub trait Renderable {
    fn render(&mut self, renderer: &mut Renderer, args: RenderArgs);
}
