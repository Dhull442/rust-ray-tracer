#[allow(dead_code)]
mod image;
use image::Image;
fn main() {
    let mut image = Image::new(1.0, 400, 100, 50);
    image.render();
}
