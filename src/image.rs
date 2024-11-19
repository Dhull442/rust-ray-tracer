mod hittable;
mod ray;
mod utility;
mod vector;
use hittable::{HitRecord, Hittable, HittableObjects, Sphere};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
use vector::{Color, Vector};

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
    sample_per_pixel: u32,
    pixel_sample_scale: f64,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        viewport_height: f64,
        image_width: f64,
        image_height: f64,
        sample_per_pixel: u32,
        max_depth: u32,
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
            sample_per_pixel: sample_per_pixel,
            pixel_sample_scale: 1.0 / sample_per_pixel as f64,
            max_depth: max_depth,
        }
    }

    pub fn get_ray(&self, idx_width: u32, idx_height: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (idx_width as f64 + offset.x) * self.pixel_delta_u
            + (idx_height as f64 + offset.y) * self.pixel_delta_v;
        Ray::new(self.center, pixel_sample - self.center)
    }

    fn sample_square() -> Vector {
        Vector {
            x: utility::random() - 0.5,
            y: utility::random() - 0.5,
            z: 0.0,
        }
    }
}

pub struct Image {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    camera: Camera,
    buffer: RgbImage,
    world: HittableObjects,
}

impl Image {
    pub fn new(aspect_ratio: f64, image_width: u32, sample_per_pixel: u32, max_depth: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            camera: Camera::new(
                1.0,
                2.0,
                image_width as f64,
                image_height as f64,
                sample_per_pixel,
                max_depth,
            ),
            buffer: ImageBuffer::new(image_width, image_height),
            world: HittableObjects::new(),
        }
    }

    pub fn render(&mut self) {
        let pb = ProgressBar::new(self.image_height as u64);
        self.world.add(Hittable::Sphere(Sphere::new(
            Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
        )));
        self.world.add(Hittable::Sphere(Sphere::new(
            Vector {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            100.0,
        )));

        for i in 0..self.image_height {
            pb.inc(1);
            for j in 0..self.image_width {
                let mut pixel_color = Color::black();
                for _ in 0..self.camera.sample_per_pixel {
                    pixel_color = pixel_color
                        + self.camera.pixel_sample_scale
                            * self
                                .camera
                                .get_ray(j, i)
                                .color(self.camera.max_depth, &self.world);
                }
                self.buffer.put_pixel(j, i, pixel_color.as_pixel());
            }
        }
        self.buffer.save("image.png").unwrap();
        self.world.clear();
    }
}
