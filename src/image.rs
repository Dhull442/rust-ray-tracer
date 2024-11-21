mod hittable;
mod ray;
mod util;
mod vector;
use hittable::{Hittable, HittableObjects, Material};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
use vector::{Color, Vector};
use crate::image::hittable::BvhNode;

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
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
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u : Vector,
    defocus_dish_v : Vector,
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
        defocus_angle: f64,
        focus_dist: f64
    ) -> Self {
        let center = lookfrom;
        let theta = util::degree_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * image_width / image_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let defocus_radius = focus_dist * f64::tan(util::degree_to_radians(defocus_angle/2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_dish_v = v * defocus_radius;
        Self {
            viewport_height,
            viewport_width,
            center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
            sample_per_pixel,
            pixel_sample_scale: 1.0 / sample_per_pixel as f64,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            w,
            u,
            v,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_dish_v,
        }
    }

    pub fn get_ray(&self, idx_width: u32, idx_height: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (idx_width as f64 + offset.x) * self.pixel_delta_u
            + (idx_height as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_time = util::random();
        Ray::new_time(ray_origin, pixel_sample - ray_origin, ray_time)
    }

    fn defocus_disk_sample(&self) -> Vector {
        let p = Vector::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_dish_v * p.y
    }

    fn sample_square() -> Vector {
        Vector::new(util::random() - 0.5,util::random() - 0.5,0.0)
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
            aspect_ratio,
            image_width,
            image_height,
            camera: Camera::new(
                image_width as f64,
                image_height as f64,
                sample_per_pixel,
                max_depth,
                20.0,
                Vector::new(13.0, 2.0, 3.0),
                Vector::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                0.6,
                10.0,
            ),
            buffer: ImageBuffer::new(image_width, image_height),
            world: HittableObjects::new(),
        }
    }

    fn create_scene(&mut self) {
        let ground_material = Material::new_lambertian(Color::new(0.5,0.5,0.5));
        let ground_hittable = Hittable::new_sphere(Vector::new(0.0,-1000.0,0.0),1000.0,ground_material);
        self.world.add(ground_hittable);

        for a in -10..10 {
            for b in -10..10 {
                let choose_mat = util::random();
                let center = Vector::new(a as f64 + 0.9* util::random(), 0.2, b as f64 +0.9*util::random());
                if (center - Vector::new(4.0,0.2,0.0)).len() > 0.9 {
                    if choose_mat < 0.8 {
                        // lambertian
                        let sphere_material = Material::new_lambertian(Color::random());
                        let center2 = center + Vector::new(0.0, util::random_interval(0.0,0.5),0.0);
                        let sphere_hittable = Hittable::new_moving_sphere(center, center2, 0.2, sphere_material);
                        self.world.add(sphere_hittable);
                    } else if choose_mat < 0.95 {
                        // metal
                        let sphere_material = Material::new_metal(Color::random_interval(0.5,1.0),  util::random_interval(0.0,0.5));
                        let sphere_hittable = Hittable::new_sphere(center, 0.2, sphere_material);
                        self.world.add(sphere_hittable);
                    } else {
                        // dielectric
                        let sphere_material = Material::new_dielectric(1.5);
                        let sphere_hittable = Hittable::new_sphere(center, 0.2, sphere_material);
                        self.world.add(sphere_hittable);
                    }
                }
            }
        }

        let material_1 = Material::new_dielectric(1.5);
        let hittable_1 = Hittable::new_sphere(Vector::new(0.0,1.0,0.0),1.0,material_1);
        self.world.add(hittable_1);

        let material_2 = Material::new_lambertian(Color::new(0.4,0.2,0.1));
        let hittable_2 = Hittable::new_sphere(Vector::new(-4.0,1.0,0.0),1.0,material_2);
        self.world.add(hittable_2);

        let material_3 = Material::new_metal(Color::new(0.7,0.6,0.5),0.0);
        let hittable_3 = Hittable::new_sphere(Vector::new(4.0,1.0,0.0),1.0,material_3);
        self.world.add(hittable_3);
    }

    pub fn render(&mut self) {
        self.create_scene();
        let pb = ProgressBar::new((self.image_height) as u64);
        let bvhWorld = BvhNode::new(&self.world);
        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let mut pixel_color = Color::black();
                for _ in 0..self.camera.sample_per_pixel {
                    pixel_color = pixel_color
                        + self.camera.pixel_sample_scale
                            * self
                                .camera
                                .get_ray(j, i)
                                .color(self.camera.max_depth, &bvhWorld);
                }
                self.buffer.put_pixel(j, i, pixel_color.as_pixel());
            }
            pb.inc(1);
            // self.buffer.save("proc_image.png").unwrap();
        }
        self.buffer.save("image.png").unwrap();
        self.world.clear();
    }
}
