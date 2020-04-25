use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

pub struct Renderer {
    pub zoom: f64,
    pub gl: GlGraphics,
}

impl Renderer {
    pub fn new(gl: GlGraphics) -> Renderer {
        Renderer { zoom: 2.5e-4, gl }
    }

    pub fn project(&self, coords: &Vector2<f64>) -> Vector2<f64> {
        vecmath::vec2_scale(*coords, self.zoom)
    }
}

pub trait Renderable {
    fn render(&mut self, renderer: &mut Renderer, args: RenderArgs);
}
