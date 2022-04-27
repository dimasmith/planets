use opengl_graphics::{GlGraphics, OpenGL};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

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

    pub fn resolution_from_str(&mut self, resolution_string: &str) {
        let components: Vec<&str> = resolution_string.split('x').collect();
        if components.len() != 2 {
            panic!("incorrect resolution string {}", resolution_string);
        }
        let width = u32::from_str(components[0]).unwrap();
        let height = u32::from_str(components[1]).unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }
}

impl TryFrom<&str> for ScreenResolution {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut resolution = ScreenResolution::default();
        resolution.resolution_from_str(value);
        resolution.fullscreen = false;
        Ok(resolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_resolution() {
        let resolution_str = "800x600";
        let mut resolution = ScreenResolution::default();
        resolution.resolution_from_str(resolution_str);

        assert_eq!(resolution.resolution(), [800, 600]);
    }
}
