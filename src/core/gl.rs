use opengl_graphics::{GlGraphics, OpenGL};
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedGraphics = Rc<RefCell<GlGraphics>>;

pub fn create(opengl: OpenGL) -> SharedGraphics {
    Rc::new(RefCell::new(GlGraphics::new(opengl)))
}

pub struct ScreenResolution {
    width: u32,
    height: u32,
    fullscreen: bool,
}

impl Default for ScreenResolution {
    fn default() -> Self {
        ScreenResolution {
            width: 1920,
            height: 1080,
            fullscreen: true,
        }
    }
}

impl ScreenResolution {
    pub fn resolution(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }
}
