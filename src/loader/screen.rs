use graphics::character::CharacterCache;
use graphics::{Context, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

use crate::loader::state::LoadingState;
use crate::text::SharedGlyphCache;

pub struct LoadingScreen<'l> {
    glyphs: SharedGlyphCache<'l>,
}

impl<'l> LoadingScreen<'l> {
    pub fn new(glyphs: SharedGlyphCache<'l>) -> Self {
        LoadingScreen { glyphs }
    }

    pub fn render(&mut self, state: &LoadingState, args: RenderArgs, gl: &mut GlGraphics) {
        let glyphs = &mut (*self.glyphs).borrow_mut();
        let progress_line = format!("Loading... ({}%)", (state.progress() * 100.0) as i32);
        let context = gl.draw_begin(args.viewport());
        draw_text(progress_line.as_str(), args, glyphs, context, gl);
        gl.draw_end();
    }
}

fn draw_text(
    text: &str,
    args: RenderArgs,
    glyphs: &mut GlyphCache,
    context: Context,
    gl: &mut GlGraphics,
) {
    let text_length: f64 = glyphs.width(32, text).or::<f64>(Ok(400.0)).unwrap();
    let ctx = context.trans(
        args.window_size[0] / 2.0 - text_length / 2.0,
        args.window_size[1] / 2.0,
    );
    graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
    graphics::text([1.0, 1.0, 1.0, 1.0], 32, text, glyphs, ctx.transform, gl).unwrap();
}
