mod image;
use image::Image;
fn main() {
    let mut image = Image::new(16.0 / 9.0, 400, 100, 50);
    image.render();
}
