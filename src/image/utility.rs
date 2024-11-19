use rand::Rng;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// Utility Functions
pub fn degree_to_radians(degree: f64) -> f64 {
    return degree * PI / 180.0;
}

pub fn random() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

#[derive(Default, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn from(min: f64, max: f64) -> Self {
        Self { min: min, max: max }
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
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn random(interval: Self) -> f64 {
        rand::thread_rng().gen_range(interval.min..interval.max)
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
}
