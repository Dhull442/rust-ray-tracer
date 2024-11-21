use rand::Rng;
use std::f64::consts;

// Utility Functions
pub fn degree_to_radians(degree: f64) -> f64 {
    (degree * consts::PI) / 180.0
}

pub fn random() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_interval(min: f64, max: f64) -> f64{
    rand::thread_rng().gen_range(min..max)
}

#[derive(Default, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_from_interval(a: Self, b: Self)-> Self{
        Self{
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64)-> Self{
        let pad = delta/2.0;
        Self::new(self.min - pad, self.max+pad)
    }
}
