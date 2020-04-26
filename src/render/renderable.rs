use crate::render::Projector;
use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait Renderable {
    fn pre_render(&mut self, _projector: &Projector, _context: &mut Context, _gl: &mut GlGraphics) {
    }

    fn render_all(&mut self, projector: &Projector, context: &mut Context, gl: &mut GlGraphics) {
        self.pre_render(projector, context, gl);
        self.render(projector, context, gl);
        self.post_render(projector, context, gl);
    }

    fn render(&mut self, projector: &Projector, context: &mut Context, gl: &mut GlGraphics);

    fn post_render(
        &mut self,
        _projector: &Projector,
        _context: &mut Context,
        _gl: &mut GlGraphics,
    ) {
    }
}
