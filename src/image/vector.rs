use crate::image::util::{random, random_interval, Interval};
use core::ops::{Add, Div, Mul, Neg, Sub};
use image::Rgb;

#[derive(Default, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn random() -> Self {
        Self::new(random(), random(), random())
    }

    pub fn random_interval(min: f64, max: f64) -> Self {
        Self::new(
            random_interval(min, max),
            random_interval(min, max),
            random_interval(min, max),
        )
    }

    pub fn random_unit_vector() -> Self {
        let theta = 2.0 * std::f64::consts::PI * random();
        let z = random_interval(-1.0, 1.0);
        let r = (1.0 - z.powf(2.0)).sqrt();
        Self::new(r * f64::cos(theta), r * f64::sin(theta), z)
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Vector::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let theta = 2.0 * std::f64::consts::PI * random();
        Self::new(f64::cos(theta), f64::sin(theta), 0.0)
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

    pub fn axis(&self, n: u64) -> f64 {
        if n == 0 {
            self.x
        } else if n == 1 {
            self.y
        } else {
            self.z
        }
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

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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
        Self { r, g, b }
    }

    pub fn as_pixel(&self) -> Rgb<u8> {
        let intensity = Interval::new(0.000, 0.999);
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

    pub fn cyan() -> Self {
        Self::new(0.0, 1.0, 1.0)
    }

    fn linear_to_gamma(linear: f64) -> f64 {
        if linear > 0.0 {
            return linear.sqrt();
        }
        0.0
    }

    pub fn random() -> Self {
        Self::new(random(), random(), random())
    }

    pub fn random_interval(min: f64, max: f64) -> Self {
        Self::new(
            random_interval(min, max),
            random_interval(min, max),
            random_interval(min, max),
        )
    }

    pub fn print(&self) {
        println!("{} {} {}", self.r, self.g, self.b)
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
