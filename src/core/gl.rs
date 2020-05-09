use opengl_graphics::{GlGraphics, OpenGL};
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedGraphics = Rc<RefCell<GlGraphics>>;

pub fn create(opengl: OpenGL) -> SharedGraphics {
    Rc::new(RefCell::new(GlGraphics::new(opengl)))
}
