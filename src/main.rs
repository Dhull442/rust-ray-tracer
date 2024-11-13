// use clippy;
mod image;
use image::Image;
fn main() {
    let mut image = Image::new();
    image.render();
    image.write();
}
