use crate::image::vector::Color;
use image::{open, RgbImage};

#[derive(Clone)]
pub struct ImageData {
    data: RgbImage,
    width: u32,
    height: u32,
}

impl ImageData {
    pub fn new(filename: String) -> Self {
        let data = open(filename).unwrap().into_rgb8();
        let width = data.width();
        let height = data.height();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_data(&self, i: u32, j: u32) -> Color {
        let x = i.min(self.width - 1);
        let y = j.min(self.height - 1);

        let rgb = self.data.get_pixel(x as u32, y as u32);
        Color::new(
            rgb[0] as f64 / 255.0,
            rgb[1] as f64 / 255.0,
            rgb[2] as f64 / 255.0,
        )
    }
}
