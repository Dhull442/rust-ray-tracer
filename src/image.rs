use log::{info, warn};
mod ray;
mod vector;
use ray::Ray;
use vector::Vector;

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
    center: Vector,
    viewport_u: Vector,
    viewport_v: Vector,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    viewport_upper_left: Vector,
    pixel00_loc: Vector,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        viewport_height: f64,
        image_width: f64,
        image_height: f64,
    ) -> Self {
        let viewport_width = viewport_height * image_width / image_height;
        let center = Vector::default();
        let viewport_u = Vector {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_v = Vector {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;
        let viewport_upper_left = center
            - Vector {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            center: center,
            viewport_u: viewport_u,
            viewport_v: viewport_v,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            viewport_upper_left: viewport_upper_left,
            pixel00_loc: pixel00_loc,
        }
    }

    pub fn get_ray(&self, idx_width: u64, idx_height: u64) -> Ray {
        let pixel_center = self.pixel00_loc
            + (idx_width as f64) * self.pixel_delta_u
            + (idx_height as f64) * self.pixel_delta_v;
        Ray::new(self.center, pixel_center - self.center)
    }
}

pub struct Image {
    aspect_ratio: f64,
    image_width: u64,
    image_height: u64,
    camera: Camera,
}

impl Image {
    pub fn new(aspect_ratio: f64, image_width: u64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u64;
        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            camera: Camera::new(1.0, 2.0, image_width as f64, image_height as f64),
        }
    }

    pub fn render(&mut self) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for i in 0..self.image_height {
            info!(target: "render", "Generating Row {i:?}");
            for j in 0..self.image_width {
                self.camera.get_ray(j, i).color().write();
            }
        }
        info!(target: "render", "Generation Done!");
    }
}
