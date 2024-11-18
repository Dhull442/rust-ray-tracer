// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// Utility Functions
pub fn degree_to_radians(degree: f64) -> f64 {
    return degree * PI / 180.0;
}

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

    pub fn Empty() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn Universe() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}
