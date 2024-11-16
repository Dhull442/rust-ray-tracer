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

    fn hit_sphere(&self, center: Vector, radius: f64) -> bool {
        let oc = center - self.origin();
        let a = self.direction.dot(self.direction);
        let b = -2.0 * self.direction.dot(oc);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b*b - 4.0*a*c;
        discriminant >= 0.0
    }

    pub fn color(&self) -> Color {
        if self.hit_sphere(Vector{x:0.0,y:0.0,z:-5.0},0.5) {
            return Color::red();
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
