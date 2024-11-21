use std::cmp::Ordering;
use crate::image::ray::Ray;
use crate::image::util;
use crate::image::vector::{Vector};
mod aabb;
use aabb::AABB;
pub mod material;
mod texture;

pub use material::{Material, HitRecord};
#[derive(Default,Clone)]
enum HittableType {
    #[default]
    Sphere,
}
#[derive(Default,Clone)]
pub struct Hittable {
    hittable: HittableType,
    center: Ray,
    radius: f64,
    material: Material,
    bbox: AABB,
}

impl Hittable {
    pub fn new_sphere(center: Vector, radius: f64, material: Material) -> Self {
        let rvec = Vector::new(radius,radius,radius);
        Self {
            hittable: HittableType::Sphere,
            center: Ray::new(center, Vector::zero()),
            radius,
            material,
            bbox: AABB::new_from_vector(center + rvec, center - rvec)
        }
    }
    pub fn new_moving_sphere(center1: Vector, center2: Vector, radius: f64, material: Material) -> Self{
        let rvec = Vector::new(radius,radius,radius);
        let bbox1 = AABB::new_from_vector(center1 - rvec, center1 + rvec);
        let bbox2 = AABB::new_from_vector(center2 - rvec, center2 + rvec);
        Self{
            hittable: HittableType::Sphere,
            center: Ray::new(center1,center2-center1),
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
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(*ray, (rec.p - current_center) / self.radius);
        rec.material = self.material;
        true
    }
    pub fn hit(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        match self.hittable {
            HittableType::Sphere => self.hit_sphere(ray, ray_t, rec),
        }
    }

    pub fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[derive(Clone)]
pub struct HittableObjects {
    objects: Vec<Hittable>,
    bbox: AABB,
}

impl HittableObjects {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn add(&mut self, hittable: Hittable) {
        self.bbox = AABB::new_from_aabb(&self.bbox, &hittable.bounding_box());
        self.objects.push(hittable);
    }

    pub fn bounding_box(&self) -> AABB{
        self.bbox
    }

    pub fn clear(&mut self) {
        self.objects.clear();
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
enum BVH{
    Node{left: BvhNode, right: BvhNode},
    Leaf{hittable: Hittable},
}
pub struct BvhNode {
    content: Box<BVH>,
    bbox: AABB,
}

impl BvhNode{

    pub fn new(world: &HittableObjects)-> Self{
        Self::new_from_objects(&world.objects,0, world.objects.len())
    }

    pub fn new_from_object(hittable: Hittable) -> Self{
        Self{
            bbox: hittable.bounding_box(),
            content: Box::new(BVH::Leaf{hittable}),
        }
    }
    pub fn new_from_objects(list: &Vec<Hittable>, start: usize, end: usize) -> Self{
        let left: BvhNode;
        let right: BvhNode;
        let axis = util::random_interval(0.0, 3.0) as u64;
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };
        let object_span = end - start;
        if object_span == 1 {
            return Self::new_from_object(list[start].clone());
        } else if object_span == 2 {
            left = Self::new_from_object(list[start].clone());
            right = Self::new_from_object(list[start+1].clone());
        } else {
            let mut sublist = list[start..end].to_vec();
            sublist.sort_by(|a,b| comparator(a,b));
            let mid = start + object_span / 2;
            left = Self::new_from_objects(&sublist, 0, mid-start);
            right = Self::new_from_objects(&sublist, mid-start, end-start);
        }
        let bbox = AABB::new_from_aabb(&left.bounding_box(), &right.bounding_box());
        Self{
            content: Box::new(BVH::Node{left,right}),
            bbox,
        }

    }

    pub fn box_compare(a: &Hittable, b: &Hittable, axis_index: u64) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
    }
    pub fn box_x_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
    pub fn hit(&self, ray: &Ray, ray_t: &mut util::Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }
        match &*self.content {
            BVH::Leaf{hittable} => {return hittable.hit(&ray, *ray_t,rec);},
            BVH::Node {left, right} => {return left.hit(&ray,ray_t,rec) || right.hit(&ray,ray_t,rec);},
        }
    }

    pub fn bounding_box(&self) -> AABB{
        self.bbox
    }
}

