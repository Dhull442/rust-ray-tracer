use crate::image::hittable::material::pdf::{MixPDF, PDF};
use crate::image::hittable::material::HitRecord;
use crate::image::hittable::HittableObjects;
use crate::image::util;
use crate::image::util::random_interval;
use crate::image::vector::{Color, Vector};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_time(origin: Vector, direction: Vector, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Vector {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }

    pub fn color(
        &self,
        depth: u32,
        world: &HittableObjects,
        background: Color,
        lights: &HittableObjects,
    ) -> Color {
        if depth == 0 {
            return Color::black();
        }
        let mut rec = HitRecord::default();
        let interval = util::Interval::new(0.001, f64::INFINITY);
        if !world.hit(self, interval, &mut rec) {
            return background;
        }
        let mut ray_scattered = Ray::default();
        let mut attenuation = Color::black();
        let mut pdf = 0.;
        let color_from_emission = rec.material.emitted(self, &rec);
        if !rec
            .material
            .scatter(self, &rec, &mut attenuation, &mut ray_scattered, &mut pdf)
        {
            return color_from_emission;
        }
        let cosine_pdf = PDF::new_cosine(rec.normal);
        let light_pdf = PDF::new_lights(lights, rec.p);
        let mut mix_pdf = MixPDF::new();
        mix_pdf.add(cosine_pdf);
        mix_pdf.add(light_pdf);
        ray_scattered = Ray::new_time(rec.p, mix_pdf.generate(), self.time);
        pdf = mix_pdf.value(ray_scattered.direction);
        let scattering_pdf = rec.material.scattering_pdf(self, &rec, &ray_scattered);
        let sample_color = ray_scattered.color(depth - 1, world, background, lights);
        let color_from_scatter = (scattering_pdf * attenuation * sample_color) / pdf;
        color_from_emission + color_from_scatter
    }
}
