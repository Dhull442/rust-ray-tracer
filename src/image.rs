mod hittable;
mod ray;
mod util;
mod vector;
use hittable::{Hittable, HittableObjects, Material};
use image::{ImageBuffer, RgbImage};
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
    vfov: f64,
    lookfrom: Vector,
    lookat: Vector,
    vup: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
}

impl Camera {
    pub fn new(
        image_width: f64,
        image_height: f64,
        sample_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Vector,
        lookat: Vector,
        vup: Vector,
    ) -> Self {
        let center = lookfrom;
        let focal_length = (lookfrom - lookat).len();
        let theta = util::degree_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width / image_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
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
            vfov: vfov,
            lookfrom: lookfrom,
            lookat: lookat,
            vup: vup,
            w: w,
            u: u,
            v: v,
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
            x: util::random() - 0.5,
            y: util::random() - 0.5,
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
                image_width as f64,
                image_height as f64,
                sample_per_pixel,
                max_depth,
                53.0,
                Vector::new(0.0, 5.0, 5.0),
                Vector::new(0.0, 0.0, -1.0),
                Vector::new(0.0, 1.0, 0.0),
            ),
            buffer: ImageBuffer::new(image_width, image_height),
            world: HittableObjects::new(),
        }
    }

    pub fn render(&mut self) {
        let pb = ProgressBar::new((self.image_height * self.image_width) as u64);
        let material_ground = Material::new_lambertian(Color::new(0.8, 0.8, 0.0));
        let material_center = Material::new_lambertian(Color::new(0.1, 0.2, 0.5));
        let material_left = Material::new_dielectric(1.5);
        let material_bubble = Material::new_dielectric(1.0 / 1.5);
        let material_right = Material::new_metal(Color::new(0.8, 0.6, 0.2), 1.0);
        let hittable_ground =
            Hittable::new_sphere(Vector::new(0.0, -100.5, -1.0), 100.0, material_ground);
        let hittable_center =
            Hittable::new_sphere(Vector::new(0.0, 0.0, -1.2), 0.5, material_center);
        let hittable_left = Hittable::new_sphere(Vector::new(-1.0, 0.0, -1.0), 0.5, material_left);
        let hittable_bubble =
            Hittable::new_sphere(Vector::new(-1.0, 0.0, -1.0), 0.3, material_bubble);
        let hittable_right = Hittable::new_sphere(Vector::new(1.0, 0.0, -1.0), 0.5, material_right);
        self.world.add(hittable_ground);
        self.world.add(hittable_center);
        self.world.add(hittable_left);
        self.world.add(hittable_bubble);
        self.world.add(hittable_right);

        for i in 0..self.image_height {
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
                pb.inc(1);
            }
        }
        self.buffer.save("image.png").unwrap();
        self.world.clear();
    }
}
