use crate::image::hittable::{HitRecord, HittableObjects};
use crate::image::utility;
use crate::image::vector::{Color, Vector};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    pub fn origin(&self) -> Vector {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }

    pub fn color(&self, world: &HittableObjects) -> Color {
        let mut rec = HitRecord::default();
        if (world.hit(
            *self,
            utility::Interval::from(0.0, utility::INFINITY),
            &mut rec,
        )) {
            return Color::from_unit_vector(rec.normal);
        }
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::white()
            + a * Color {
                r: 128,
                g: 179,
                b: 255,
            }
    }
}
