use crate::image::ray::Ray;
use crate::image::util;
use crate::image::vector::Vector;
use std::f64::consts::PI;
mod aabb;
use aabb::AABB;
pub mod bvh;
pub mod material;
pub mod texture;

use crate::image::hittable::bvh::BvhNode;
pub use material::{HitRecord, Material};

#[derive(Default, Clone)]
enum HittableType {
    #[default]
    Sphere,
}
#[derive(Default, Clone)]
pub struct Hittable {
    hittable: HittableType,
    center: Ray,
    radius: f64,
    material: Material,
    bbox: AABB,
}

impl Hittable {
    pub fn new_sphere(center: Vector, radius: f64, material: Material) -> Self {
        let rvec = Vector::new(radius, radius, radius);
        Self {
            hittable: HittableType::Sphere,
            center: Ray::new(center, Vector::zero()),
            radius,
            material,
            bbox: AABB::new_from_vector(center - rvec, center + rvec),
        }
    }
    pub fn new_moving_sphere(
        center1: Vector,
        center2: Vector,
        radius: f64,
        material: Material,
    ) -> Self {
        let rvec = Vector::new(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1);
        let bbox1 = AABB::new_from_vector(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let bbox2 = AABB::new_from_vector(center.at(1.0) - rvec, center.at(1.0) + rvec);
        Self {
            hittable: HittableType::Sphere,
            center,
            radius,
            material,
            bbox: AABB::new_from_aabb(&bbox1, &bbox2),
        }
    }
    pub fn hit_sphere(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let oc = current_center - ray.origin();
        let a = ray.direction().len_squared();
        let h = ray.direction().dot(oc);
        let c = oc.len_squared() - self.radius.powf(2.0);
        let discriminant = h.powf(2.0) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let d_sqrt = discriminant.sqrt();
        let mut root = (h - d_sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + d_sqrt) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(*ray, outward_normal);
        self.get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();
        true
    }
    pub fn hit(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        match self.hittable {
            HittableType::Sphere => self.hit_sphere(ray, ray_t, rec),
        }
    }

    pub fn get_sphere_uv(&self, p: Vector, u: &mut f64, v: &mut f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }

    pub fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[derive(Clone)]
pub struct HittableObjects {
    objects: Vec<Hittable>,
    bvh: BvhNode,
}

impl HittableObjects {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh: BvhNode::empty(),
        }
    }

    pub fn add(&mut self, hittable: Hittable) {
        self.objects.push(hittable);
    }

    pub fn bounding_box(&self) -> AABB {
        self.bvh.bounding_box()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bvh = BvhNode::empty();
    }

    pub fn init_bvh(&mut self) {
        self.bvh = BvhNode::new(self);
    }

    pub fn hit(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        let mut closest_so_far = ray_t.max;
        let mut hit_something = false;
        for object in self.objects.iter() {
            if object.hit(ray, util::Interval::new(ray_t.min, closest_so_far), rec) {
                closest_so_far = rec.t;
                hit_something = true;
            }
        }
        hit_something
    }
}
