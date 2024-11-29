use crate::image::ray::Ray;
use crate::image::util::Interval;
use crate::image::vector::Vector;
use std::ops::Add;
#[derive(Default, Copy, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    fn pad_to_min(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
    }
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_min();
        aabb
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }
    pub fn new_from_vector(a: Vector, b: Vector) -> Self {
        Self::new(
            Interval::new(a.x.min(b.x), a.x.max(b.x)),
            Interval::new(a.y.min(b.y), a.y.max(b.y)),
            Interval::new(a.z.min(b.z), a.z.max(b.z)),
        )
    }

    pub fn new_from_aabb(box1: &Self, box2: &Self) -> Self {
        Self {
            x: Interval::new_from_interval(box1.x, box2.x),
            y: Interval::new_from_interval(box1.y, box2.y),
            z: Interval::new_from_interval(box1.z, box2.z),
        }
    }

    pub fn axis_interval(&self, n: u64) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.axis(axis);

            let t0 = (ax.min - ray_orig.axis(axis)) * adinv;
            let t1 = (ax.max - ray_orig.axis(axis)) * adinv;

            if t0 < t1 {
                ray_t.min = ray_t.min.max(t0);
                ray_t.max = ray_t.max.min(t1);
            } else {
                ray_t.min = ray_t.min.max(t1);
                ray_t.max = ray_t.max.min(t0);
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn empty() -> Self {
        Self::new(Interval::empty(), Interval::empty(), Interval::empty())
    }

    pub fn universe() -> Self {
        Self::new(
            Interval::universe(),
            Interval::universe(),
            Interval::universe(),
        )
    }

    pub fn x(&self) -> Interval {
        self.x
    }

    pub fn y(&self) -> Interval {
        self.y
    }

    pub fn z(&self) -> Interval {
        self.z
    }
    pub fn debug(&self) -> String {
        format!(
            "[{},{},{}]\t",
            self.x.debug(),
            self.y.debug(),
            self.z.debug()
        )
    }
}

impl Add<Vector> for AABB {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
