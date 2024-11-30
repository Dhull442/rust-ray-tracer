#[allow(dead_code)]
mod image;
use image::Image;
fn main() {
    let mut image = Image::new(1.0, 600, 800, 20);
    image.render_par();
}
