use crate::image::ray::Ray;
use crate::image::utility;
use core::ops::{Add, Div, Mul, Sub};
use image::Rgb;

#[derive(Default, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
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
            x: self.x * other.y - self.y * other.x,
            y: self.y * other.z - self.z * other.y,
            z: self.z * other.x - self.x * other.z,
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.len()
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
    pub fn as_pixel(&self) -> Rgb<u8> {
        Rgb([(self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8])
    }

    pub fn from_unit_vector(unit_vector: Vector) -> Self {
        Self {
            r: unit_vector.x,
            g: unit_vector.y,
            b: unit_vector.z,
        }
    }

    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Self {
        Self { r: 1.0, g: 0.0, b: 0.0 }
    }

    pub fn green() -> Self {
        Self { r: 0.0, g: 1.0, b: 0.0 }
    }
    pub fn blue() -> Self {
        Self { r: 0.0, g: 0.0, b: 1.0 }
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
            b: self.b  * t,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}
