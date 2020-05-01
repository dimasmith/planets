use crate::loader::state::LoadingState;
use graphics::character::CharacterCache;
use graphics::Transformed;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::input::RenderArgs;

pub struct LoadingScreen<'l> {
    character_cache: GlyphCache<'l>,
}

impl<'l> LoadingScreen<'l> {
    pub fn new() -> Self {
        LoadingScreen {
            character_cache: LoadingScreen::character_cache(),
        }
    }

    pub fn render(&mut self, state: &LoadingState, args: RenderArgs, gl: &mut GlGraphics) {
        let glyphs = &mut self.character_cache;
        let mut context = gl.draw_begin(args.viewport());
        let progress_line = format!("Loading... ({}%)", (state.progress() * 100.0) as i32);
        let text_width = glyphs.width(32, progress_line.as_str()).unwrap();
        context = context.trans(
            args.window_size[0] / 2.0 - text_width / 2.0,
            args.window_size[1] / 2.0,
        );
        graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
        graphics::text(
            [1.0, 1.0, 1.0, 1.0],
            32,
            progress_line.as_str(),
            glyphs,
            context.transform,
            gl,
        )
        .unwrap();
        gl.draw_end();
    }

    fn character_cache() -> GlyphCache<'l> {
        let font_data = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        GlyphCache::from_bytes(font_data, (), texture_settings).expect("could not load font")
    }
}
