use core::ops::{Add, Div, Mul, Sub};

#[derive(Default, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
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
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn write(&self) {
        println!("{} {} {}", self.r, self.g, self.b);
    }

    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self {
            r: (self.r as f64 * t) as u8,
            g: (self.g as f64 * t) as u8,
            b: (self.b as f64 * t) as u8,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}
