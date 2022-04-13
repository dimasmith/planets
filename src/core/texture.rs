use image::io::Reader;
use opengl_graphics::{Filter, Texture, TextureSettings};
use std::borrow::Borrow;

/// load texture from assets
pub fn load_texture(name: String) -> Texture {
    let mut path = String::from("assets/textures/");
    path.push_str(&name);
    path.push_str(".png");
    let image = Reader::open(path).unwrap().decode().unwrap();
    Texture::from_image(
        image.into_rgba8().borrow(),
        &TextureSettings::new().filter(Filter::Linear),
    )
}
