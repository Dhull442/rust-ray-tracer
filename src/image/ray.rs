use crate::image::hittable::{HittableObjects};
use crate::image::hittable::material::HitRecord;
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

    pub fn new_time(origin: Vector, direction: Vector, time: f64) -> Self{
        Self{
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

    pub fn time(&self) -> f64{
        self.time
    }
    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }

    pub fn color(&self, depth: u32, world: &HittableObjects) -> Color {
        if depth == 0 {
            return Color::black();
        }
        let mut rec = HitRecord::default();
        if world.hit(self, util::Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut ray_scattered = Ray::default();
            let mut attenuation = Color::black();
            if rec
                .material
                .scatter(self, &rec, &mut attenuation, &mut ray_scattered)
            {
                return attenuation * ray_scattered.color(depth - 1, world);
            }
            return Color::black();
        }
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }
}
