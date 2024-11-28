mod hittable;
mod ray;
mod util;
mod vector;
use crate::image::hittable::texture::Texture;
use hittable::{Hittable, HittableObjects, Material};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
use vector::{Color, Vector};

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
    defocus_disk_u: Vector,
    defocus_dish_v: Vector,
    background: Color,
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
        focus_dist: f64,
        background: Color,
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
        let defocus_radius = focus_dist * f64::tan(util::degree_to_radians(defocus_angle / 2.0));
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
            background,
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
        Vector::new(util::random() - 0.5, util::random() - 0.5, 0.0)
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
                40.0,
                Vector::new(278.0, 278.0, -800.0),
                Vector::new(278.0, 278.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                0.0,
                10.0,
                Color::black(),
                // Color::new(0.7,0.8,1.0),
            ),
            buffer: ImageBuffer::new(image_width, image_height),
            world: HittableObjects::new(),
        }
    }
    fn create_scene(&mut self, case: usize) {
        match case {
            1 => {
                self.spheres();
            }
            2 => {
                self.earth();
            }
            3 => {
                self.perlin_noise();
            }
            4 => {
                self.quads();
            }
            5 => {
                self.simple_lights();
            }
            6 => {
                self.cornell_box();
            }
            _ => {}
        }
    }
    fn cornell_box(&mut self) {
        let red = Material::new_lambertian(Texture::new_solid(Color::new(0.65, 0.05, 0.05)));
        let white = Material::new_lambertian(Texture::new_solid(Color::new(0.73, 0.73, 0.73)));
        let green = Material::new_lambertian(Texture::new_solid(Color::new(0.12, 0.45, 0.15)));
        let light = Material::new_diffuse_light(Texture::new_solid(Color::new(15., 15.0, 15.0)));

        let q1 = Hittable::new_quad(
            Vector::new(555., 0., 0.),
            Vector::new(0., 555., 0.),
            Vector::new(0., 0., 555.),
            green,
        );
        let q2 = Hittable::new_quad(
            Vector::new(0., 0., 0.),
            Vector::new(0., 555., 0.),
            Vector::new(0., 0., 555.),
            red,
        );
        let q3 = Hittable::new_quad(
            Vector::new(343., 554., 332.),
            Vector::new(-130., 0., 0.),
            Vector::new(0., 0., -105.),
            light,
        );
        let q4 = Hittable::new_quad(
            Vector::new(0., 0., 0.),
            Vector::new(555., 0., 0.),
            Vector::new(0., 0., 555.),
            white.clone(),
        );
        let q5 = Hittable::new_quad(
            Vector::new(555., 555., 555.),
            Vector::new(-555., 0., 0.),
            Vector::new(0., 0., -555.),
            white.clone(),
        );
        let q6 = Hittable::new_quad(
            Vector::new(0., 0., 555.),
            Vector::new(555., 0., 0.),
            Vector::new(0., 555., 0.),
            white.clone(),
        );

        self.world.add(q1);
        self.world.add(q2);
        self.world.add(q3);
        self.world.add(q4);
        self.world.add(q5);
        self.world.add(q6);
        self.world.add_hittables(HittableObjects::new_box(
            Vector::new(130., 0., 65.),
            Vector::new(295., 165., 230.),
            white.clone(),
        ));
        self.world.add_hittables(HittableObjects::new_box(
            Vector::new(265., 0., 295.),
            Vector::new(430., 330., 460.),
            white,
        ));
    }
    fn simple_lights(&mut self) {
        self.perlin_noise();
        let difflight = Material::new_diffuse_light(Texture::new_solid(Color::new(4.0, 4., 4.)));
        let light = Hittable::new_quad(
            Vector::new(3., 1., -2.),
            Vector::new(2., 0., 0.),
            Vector::new(0., 2., 0.),
            difflight,
        );
        self.world.add(light);
    }
    fn quads(&mut self) {
        let left_red = Material::new_lambertian(Texture::new_solid(Color::red()));
        let back_green = Material::new_lambertian(Texture::new_solid(Color::green()));
        let right_blue = Material::new_lambertian(Texture::new_solid(Color::blue()));
        let upper_orange = Material::new_lambertian(Texture::new_solid(Color::new(1.0, 0.5, 0.0)));
        let lower_teal = Material::new_lambertian(Texture::new_solid(Color::new(0.2, 0.8, 0.8)));
        let left = Hittable::new_quad(
            Vector::new(-3.0, -2.0, 5.0),
            Vector::new(0.0, 0.0, -4.0),
            Vector::new(0.0, 4.0, 0.0),
            left_red,
        );
        let right = Hittable::new_quad(
            Vector::new(3.0, -2.0, 1.0),
            Vector::new(0.0, 0.0, 4.0),
            Vector::new(0.0, 4.0, 0.0),
            right_blue,
        );
        let back = Hittable::new_quad(
            Vector::new(-2.0, -2.0, 0.0),
            Vector::new(4.0, 0.0, 0.),
            Vector::new(0.0, 4.0, 0.0),
            back_green,
        );
        let upper = Hittable::new_quad(
            Vector::new(-2.0, 3.0, 1.0),
            Vector::new(4.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 4.0),
            upper_orange,
        );
        let lower = Hittable::new_quad(
            Vector::new(-2., -3., 5.),
            Vector::new(4., 0., 0.),
            Vector::new(0.0, 0.0, -4.0),
            lower_teal,
        );
        self.world.add(left);
        self.world.add(right);
        self.world.add(back);
        self.world.add(upper);
        self.world.add(lower);
    }
    fn perlin_noise(&mut self) {
        let perlin_texture = Texture::new_perlin(4.0);
        let perlin_surface = Material::new_lambertian(perlin_texture);
        self.world.add(Hittable::new_sphere(
            Vector::new(0.0, -1000.0, 0.0),
            1000.0,
            perlin_surface.clone(),
        ));
        self.world.add(Hittable::new_sphere(
            Vector::new(0.0, 2.0, 0.0),
            2.0,
            perlin_surface,
        ));
    }
    fn earth(&mut self) {
        let earth_texture = Texture::new_image("earthmap.jpg".to_string());
        let earth_surface = Material::new_lambertian(earth_texture);
        let globe = Hittable::new_sphere(Vector::new(0.0, 0.0, 0.0), 2.0, earth_surface);
        self.world.add(globe);
    }
    fn spheres(&mut self) {
        let ground_material = Material::new_lambertian(Texture::new_checker(
            0.32,
            Color::new(0.2, 0.3, 0.1),
            Color::white(),
        ));
        let ground_hittable =
            Hittable::new_sphere(Vector::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
        self.world.add(ground_hittable);

        for a in -5..5 {
            for b in -5..5 {
                let choose_mat = util::random();
                let center = Vector::new(
                    a as f64 + 0.9 * util::random(),
                    0.2,
                    b as f64 + 0.9 * util::random(),
                );
                if (center - Vector::new(4.0, 0.2, 0.0)).len() > 0.9 {
                    if choose_mat < 0.8 {
                        // lambertian
                        let sphere_material =
                            Material::new_lambertian(Texture::new_solid(Color::random()));
                        let center2 =
                            center + Vector::new(0.0, util::random_interval(0.0, 0.5), 0.0);
                        let sphere_hittable =
                            Hittable::new_moving_sphere(center, center2, 0.2, sphere_material);
                        self.world.add(sphere_hittable);
                    } else if choose_mat < 0.95 {
                        // metal
                        let sphere_material = Material::new_metal(
                            Color::random_interval(0.5, 1.0),
                            util::random_interval(0.0, 0.5),
                        );
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
        let hittable_1 = Hittable::new_sphere(Vector::new(0.0, 1.0, 0.0), 1.0, material_1);
        self.world.add(hittable_1);

        let material_2 = Material::new_lambertian(Texture::new_solid(Color::new(0.4, 0.2, 0.1)));
        let hittable_2 = Hittable::new_sphere(Vector::new(-4.0, 1.0, 0.0), 1.0, material_2);
        self.world.add(hittable_2);

        let material_3 = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
        let hittable_3 = Hittable::new_sphere(Vector::new(4.0, 1.0, 0.0), 1.0, material_3);
        self.world.add(hittable_3);
    }

    pub fn render(&mut self) {
        self.create_scene(6);
        let pb = ProgressBar::new((self.image_height) as u64);
        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let mut pixel_color = Color::black();
                for _ in 0..self.camera.sample_per_pixel {
                    pixel_color = pixel_color
                        + self.camera.pixel_sample_scale
                            * self.camera.get_ray(j, i).color(
                                self.camera.max_depth,
                                &self.world,
                                self.camera.background,
                            );
                }
                self.buffer.put_pixel(j, i, pixel_color.as_pixel());
            }
            pb.inc(1);
        }
        self.buffer.save("image.png").unwrap();
        self.world.clear();
    }
}
