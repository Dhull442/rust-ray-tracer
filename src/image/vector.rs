use crate::image::util;
use crate::image::util::Interval;
use core::ops::{Add, Div, Mul, Sub};
use image::Rgb;

#[derive(Default, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
    pub fn print(&self) {
        println!("Vector: {} {} {}", self.x, self.y, self.z);
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.z * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn random() -> Self {
        Self::new(util::random(), util::random(), util::random())
    }

    pub fn random_interval(interval: Interval) -> Self {
        Self {
            x: Interval::random(interval),
            y: Interval::random(interval),
            z: Interval::random(interval),
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let vector = Vector::random_interval(Interval::from(-1.0, 1.0));
            let vector_len = vector.len_squared();
            if 1e-160 < vector_len && vector_len <= 1.0 {
                return vector.unit_vector();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Vector::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -1.0 * on_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Self{
        loop {
            let mut vector = Vector::random_interval(Interval::from(-1.0,1.0));
            vector.z = 0.0;
            if vector.len_squared() < 1.0 {
                return vector;
            }

        }
    }

    pub fn near_zero(&self) -> bool {
        let ep: f64 = 1e-8;
        self.x.abs() < ep && self.y.abs() < ep && self.z.abs() < ep
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-1.0 * (self.dot(normal))).min(1.0);
        let ray_out_perp = etai_over_etat * (*self + (cos_theta * normal));
        let ray_out_parallel = -1.0 * (1.0 - ray_out_perp.len_squared()).abs().sqrt() * normal;
        ray_out_perp + ray_out_parallel
    }

}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        other * self
    }
}
impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r: r, g: g, b: b }
    }

    pub fn as_pixel(&self) -> Rgb<u8> {
        let intensity = util::Interval::from(0.000, 0.999);
        let write_r = 256.0 * intensity.clamp(Self::linear_to_gamma(self.r));
        let write_g = 256.0 * intensity.clamp(Self::linear_to_gamma(self.g));
        let write_b = 256.0 * intensity.clamp(Self::linear_to_gamma(self.b));
        Rgb([write_r as u8, write_g as u8, write_b as u8])
    }

    pub fn from_unit_vector(unit_vector: Vector) -> Self {
        Self::new(unit_vector.x, unit_vector.y, unit_vector.z)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    fn linear_to_gamma(linear: f64) -> f64 {
        if linear > 0.0 {
            return linear.sqrt();
        }
        0.0
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}
