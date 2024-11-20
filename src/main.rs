#[allow(dead_code)]
mod image;
use image::Image;
fn main() {
    let mut image = Image::new(16.0 / 9.0, 800, 200, 20);
    image.render();
}
