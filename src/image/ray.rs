use crate::image::hittable::material::HitRecord;
use crate::image::hittable::HittableObjects;
use crate::image::util;
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

    pub fn color(&self, depth: u32, world: &HittableObjects, background: Color) -> Color {
        if depth == 0 {
            return Color::black();
        }
        let mut rec = HitRecord::default();
        let interval = util::Interval::new(0.001, f64::INFINITY);
        if !world.hit(self,interval, &mut rec) {
            return background;
        }
        let mut ray_scattered = Ray::default();
        let mut attenuation = Color::black();
        let color_from_emission = rec.material.emitted(rec.u, rec.v, rec.p);
        if !rec
            .material
            .scatter(self, &rec, &mut attenuation, &mut ray_scattered)
        {
            return color_from_emission;
        }
        let color_from_scatter = attenuation * ray_scattered.color(depth - 1, world, background);
        color_from_emission + color_from_scatter
    }
}
