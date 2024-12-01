#[allow(dead_code)]
mod image;
use image::Image;
fn main() {
    let mut image = Image::new(1.0, 800, 10, 50);
    image.render_par();
}
