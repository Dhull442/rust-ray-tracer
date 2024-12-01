#[allow(dead_code)]
mod image;
use image::Image;
fn main() {
    let mut image = Image::new(1.0, 400, 10, 10);
    image.render_par();
}
