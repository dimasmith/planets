use crate::render::Projector;
use graphics::math::Matrix2d;
use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait Renderable {
    fn pre_render(
        &mut self,
        projector: &Projector,
        transform: Matrix2d,
        context: &mut Context,
        gl: &mut GlGraphics,
    ) {
    }

    fn render_all(
        &mut self,
        projector: &Projector,
        transform: Matrix2d,
        context: &mut Context,
        gl: &mut GlGraphics,
    ) {
        self.pre_render(projector, transform, context, gl);
        self.render(projector, transform, context, gl);
        self.post_render(projector, transform, context, gl);
    }

    fn render(
        &mut self,
        projector: &Projector,
        transform: Matrix2d,
        context: &mut Context,
        gl: &mut GlGraphics,
    );

    fn post_render(
        &mut self,
        projector: &Projector,
        transform: Matrix2d,
        context: &mut Context,
        gl: &mut GlGraphics,
    ) {
    }
}
