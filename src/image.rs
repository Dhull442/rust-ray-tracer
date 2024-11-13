use log::{info, warn};
use std::cmp::min;
mod vector;
use vector::{Pixel, Vector};

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new() -> Self {
        Self {
            width: 256,
            height: 256,
            pixels: Vec::new(),
        }
    }

    // pub fn change_size(&mut self, width: usize, height: usize) {
    //     self.width = width;
    //     self.height = height;
    //     self.pixels = Vec::with_capacity(width * height);
    // }

    pub fn generate_image(&mut self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                self.pixels.push(Pixel::default());
            }
        }
    }

    pub fn render(&mut self) {
        if self.pixels.len() != self.height * self.width {
            self.generate_image();
        }
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i * self.height + j] = Pixel {
                    r: min(i as u8, 255),
                    g: min(j as u8, 255),
                    b: 0,
                };
            }
        }
    }

    // pub fn set_image(&mut self, pixels: Vec<Pixel>, width: usize, height: usize) {
    //     self.width = width;
    //     self.height = height;
    //     self.pixels = pixels;
    // }

    pub fn write(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);
        for i in 0..self.height {
            info!(target: "print", "Generating line {i:?}");
            for j in 0..self.width {
                self.pixels[i * self.height + j].write();
            }
        }
        info!(target: "print", "Generation Done!");
    }
}
