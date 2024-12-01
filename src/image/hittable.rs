use crate::image::ray::Ray;
use crate::image::util;
use crate::image::vector::{Color, Vector};
use std::f64::consts::{E, PI};
use std::f64::INFINITY;

mod aabb;
use aabb::AABB;
pub mod bvh;
pub mod material;

use crate::image::util::{random, random_interval, Interval};
use material::texture::Texture;
pub use material::{HitRecord, Material};

#[derive(Clone)]
pub enum HittableType {
    Sphere {
        center: Ray,
        radius: f64,
    },
    Quad {
        q: Vector,
        u: Vector,
        v: Vector,
        w: Vector,
        normal: Vector,
        d: f64,
        area: f64,
    },
}

impl Default for HittableType {
    fn default() -> Self {
        Self::Sphere {
            center: Ray::default(),
            radius: 1.0,
        }
    }
}

#[derive(Clone)]
struct Transform {
    pub offset: Vector,
    /// only Y rotation now.
    pub sin_theta: f64,
    pub cos_theta: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            offset: Vector::zero(),
            sin_theta: 0.0,
            cos_theta: 1.0,
        }
    }
}

#[derive(Default, Clone)]
struct ConstantMedium {
    neg_inv_density: f64,
    phase_function: Material,
}
#[derive(Default, Clone)]
pub struct Hittable {
    hittable: HittableType,
    transform: Transform,
    medium: ConstantMedium,
    is_medium: bool,
    material: Material,
    bbox: AABB,
}

impl Hittable {
    pub fn rotate_y(&mut self, theta: f64) {
        let radians = theta.to_radians();
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        let mut min = Vector::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vector::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * self.bbox.x().max + (1 - i) as f64 * self.bbox.x().min;
                    let y = j as f64 * self.bbox.y().max + (1 - j) as f64 * self.bbox.y().min;
                    let z = k as f64 * self.bbox.z().max + (1 - k) as f64 * self.bbox.z().min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    min.x = min.x.min(newx);
                    max.x = max.x.max(newx);
                    min.y = min.y.min(y);
                    max.y = max.y.max(y);
                    min.z = min.z.min(newz);
                    max.z = max.z.max(newz);
                }
            }
        }

        self.bbox = AABB::new_from_vector(min, max);
        self.transform.cos_theta = cos_theta;
        self.transform.sin_theta = sin_theta;
    }

    pub fn translate(&mut self, offset: Vector) {
        self.transform.offset = offset;
        self.bbox = self.bbox + offset;
    }

    pub fn add_medium(&mut self, density: f64, albedo: Color) {
        self.medium.neg_inv_density = -1.0 / density;
        self.medium.phase_function = Material::new_isotropic(Texture::new_solid(albedo));
        self.is_medium = true;
    }
    pub fn new_quad(q: Vector, u: Vector, v: Vector, material: Material) -> Self {
        let bbox_d1 = AABB::new_from_vector(q, q + u + v);
        let bbox_d2 = AABB::new_from_vector(q + u, q + v);
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        let area = n.len();
        Self {
            hittable: HittableType::Quad {
                q,
                u,
                v,
                w,
                normal,
                d,
                area,
            },
            material,
            bbox: AABB::new_from_aabb(&bbox_d1, &bbox_d2),
            transform: Default::default(),
            medium: Default::default(),
            is_medium: false,
        }
    }
    pub fn new_sphere(center: Vector, radius: f64, material: Material) -> Self {
        let rvec = Vector::new(radius, radius, radius);
        Self {
            hittable: HittableType::Sphere {
                center: Ray::new(center, Vector::zero()),
                radius,
            },
            transform: Default::default(),
            medium: Default::default(),
            is_medium: false,
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
            hittable: HittableType::Sphere { center, radius },
            transform: Default::default(),
            medium: Default::default(),
            is_medium: false,
            material,
            bbox: AABB::new_from_aabb(&bbox1, &bbox2),
        }
    }
    fn hit_sphere(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        let HittableType::Sphere { center, radius } = self.hittable else {
            return false;
        };
        let current_center = center.at(ray.time());
        let oc = current_center - ray.origin();
        let a = ray.direction().len_squared();
        let h = ray.direction().dot(oc);
        let c = oc.len_squared() - radius.powf(2.0);
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
        let outward_normal = (rec.p - current_center) / radius;
        rec.set_face_normal(*ray, outward_normal);
        self.get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();
        true
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
    fn hit_quad(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        let HittableType::Quad {
            q,
            u,
            v,
            w,
            normal,
            d,
            ..
        } = self.hittable
        else {
            return false;
        };
        let denom = normal.dot(ray.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (d - normal.dot(ray.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - q;
        let alpha = w.dot(planar_hitpt_vector.cross(v));
        let beta = w.dot(u.cross(planar_hitpt_vector));
        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }
        rec.t = t;
        rec.p = intersection;
        rec.material = self.material.clone();
        rec.set_face_normal(*ray, normal);
        true
    }
    pub fn hit(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        if self.is_medium {
            let mut rec1: HitRecord = Default::default();
            let mut rec2: HitRecord = Default::default();
            if !self.hit_object(ray, Interval::universe(), &mut rec1) {
                return false;
            }
            if !self.hit_object(ray, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2) {
                return false;
            }

            rec1.t = rec1.t.max(ray_t.min);
            rec2.t = rec2.t.min(ray_t.max);
            if rec1.t >= rec2.t {
                return false;
            }

            rec1.t = rec1.t.max(0.);

            let ray_length = ray.direction().len();
            let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
            let hit_distance = self.medium.neg_inv_density * f64::log(random(), E);

            if hit_distance > distance_inside_boundary {
                return false;
            }

            rec.t = rec1.t + hit_distance / ray_length;
            rec.p = ray.at(rec.t);
            rec.normal = Vector::new(1., 0., 0.);
            rec.front_face = true;
            rec.material = self.medium.phase_function.clone();
            true
        } else {
            self.hit_object(ray, ray_t, rec)
        }
    }
    fn hit_object(&self, ray: &Ray, ray_t: util::Interval, rec: &mut HitRecord) -> bool {
        let ray_transform = &Ray::new_time(
            ray.origin() - self.transform.offset,
            ray.direction(),
            ray.time(),
        );
        let origin = Vector::new(
            self.transform.cos_theta * ray_transform.origin().x
                - self.transform.sin_theta * ray_transform.origin().z,
            ray_transform.origin().y,
            self.transform.sin_theta * ray_transform.origin().x
                + self.transform.cos_theta * ray_transform.origin().z,
        );
        let direction = Vector::new(
            self.transform.cos_theta * ray_transform.direction().x
                - self.transform.sin_theta * ray_transform.direction().z,
            ray_transform.direction().y,
            self.transform.sin_theta * ray_transform.direction().x
                + self.transform.cos_theta * ray_transform.direction().z,
        );
        let ray_rotated = &Ray::new_time(origin, direction, ray_transform.time());
        let hit_object = match self.hittable {
            HittableType::Sphere { .. } => self.hit_sphere(ray_rotated, ray_t, rec),
            HittableType::Quad { .. } => self.hit_quad(ray_rotated, ray_t, rec),
        };

        if !hit_object {
            return false;
        }

        rec.p = Vector::new(
            self.transform.cos_theta * rec.p.x + self.transform.sin_theta * rec.p.z,
            rec.p.y,
            -self.transform.sin_theta * rec.p.x + self.transform.cos_theta * rec.p.z,
        );
        rec.p = rec.p + self.transform.offset;

        rec.normal = Vector::new(
            self.transform.cos_theta * rec.normal.x + self.transform.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.transform.sin_theta * rec.normal.x + self.transform.cos_theta * rec.normal.z,
        );

        hit_object
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

    pub fn random(&self, origin: Vector) -> Vector {
        match self.hittable {
            HittableType::Quad { q, u, v, .. } => q + (random() * u) + (random() * v) - origin,
            _ => Vector::new(1.0, 0.0, 0.0),
        }
    }

    pub fn pdf_value(&self, origin: Vector, direction: Vector) -> f64 {
        match self.hittable {
            HittableType::Quad { area, .. } => {
                let mut rec: HitRecord = Default::default();
                if (!self.hit(
                    &Ray::new(origin, direction),
                    Interval::new(0.001, INFINITY),
                    &mut rec,
                )) {
                    return 0.0;
                }
                let dist_squared = rec.t * rec.t * direction.len_squared();
                let cosine = direction.dot(rec.normal).abs() / direction.len();
                dist_squared / (cosine * area)
            }
            _ => 0.0,
        }
    }
}

#[derive(Clone)]
pub struct HittableObjects {
    objects: Vec<Hittable>,
    // bvh: BvhNode,
}

impl HittableObjects {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            // bvh: BvhNode::empty(),
        }
    }

    pub fn add(&mut self, hittable: Hittable) {
        self.objects.push(hittable);
    }

    pub fn add_hittables(&mut self, hittables: HittableObjects) {
        self.objects.extend(hittables.objects);
    }

    // pub fn bounding_box(&self) -> AABB {
    //     self.bvh.bounding_box()
    // }

    pub fn clear(&mut self) {
        self.objects.clear();
        // self.bvh = BvhNode::empty();
    }

    // pub fn init_bvh(&mut self) {
    //     self.bvh = BvhNode::new(self);
    // }

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

    pub fn new_box(a: Vector, b: Vector, material: Material) -> Self {
        let mut sides = Self::new();
        let min = Vector::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vector::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vector::new(max.x - min.x, 0., 0.);
        let dy = Vector::new(0., max.y - min.y, 0.);
        let dz = Vector::new(0., 0., max.z - min.z);

        sides.add(Hittable::new_quad(
            Vector::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        ));
        sides.add(Hittable::new_quad(
            Vector::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        ));
        sides.add(Hittable::new_quad(
            Vector::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        ));
        sides.add(Hittable::new_quad(
            Vector::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        ));
        sides.add(Hittable::new_quad(
            Vector::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        ));
        sides.add(Hittable::new_quad(
            Vector::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        ));
        sides
    }

    pub fn rotate_y(&mut self, theta: f64) {
        for object in self.objects.iter_mut() {
            object.rotate_y(theta);
        }
    }

    pub fn translate(&mut self, offset: Vector) {
        for object in self.objects.iter_mut() {
            object.translate(offset);
        }
    }

    pub fn random(&self, origin: Vector) -> Vector {
        if self.objects.is_empty() {
            return Vector::new(1.0, 0.0, 0.0);
        }
        let i = random_interval(0., self.objects.len() as f64).floor() as usize;
        self.objects[i].random(origin)
    }

    pub fn pdf_value(&self, origin: Vector, direction: Vector) -> f64 {
        if self.objects.is_empty() {
            return 0.001;
        }
        let i = random_interval(0., self.objects.len() as f64).floor() as usize;
        self.objects[i].pdf_value(origin, direction)
    }
}
