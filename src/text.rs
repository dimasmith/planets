use opengl_graphics::{Filter, GlyphCache, TextureSettings};
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedGlyphCache<'g> = Rc<RefCell<GlyphCache<'g>>>;

pub fn create_font_cache() -> SharedGlyphCache<'static> {
    let font_data = include_bytes!("../assets/fonts/font.ttf");
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    Rc::new(RefCell::new(
        GlyphCache::from_bytes(font_data, (), texture_settings).expect("could not load font"),
    ))
}
