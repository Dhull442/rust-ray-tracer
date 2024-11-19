use crate::image::ray::Ray;
use crate::image::utility;
use crate::image::vector::{Color, Vector};

#[derive(Default, Copy, Clone)]
pub enum MaterialType {
    #[default]
    Lambertian,
    Metal,
}

#[derive(Default, Copy, Clone)]
pub struct Material {
    material: MaterialType,
    albedo: Color,
    fuzz: f64,
}

impl Material {
    pub fn new(material: MaterialType, albedo: Color, fuzz: f64) -> Self {
        Self {
            material: material,
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        match self.material {
            MaterialType::Lambertian => {
                return self.scatter_lambertian(ray_in, rec, attenuation, ray_scattered);
            }
            MaterialType::Metal => {
                return self.scatter_metal(ray_in, rec, attenuation, ray_scattered);
            }
        }
    }
    pub fn scatter_lambertian(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vector::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *ray_scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
    pub fn scatter_metal(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vector::reflect(&(ray_in.direction()), rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vector::random_unit_vector();
        *ray_scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        ray_scattered.direction().dot(rec.normal) > 0.0
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub p: Vector,
    pub t: f64,
    pub normal: Vector,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vector) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -1.0 * outward_normal;
        }
    }
}

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Material) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
    pub fn hit(&self, ray: Ray, ray_t: utility::Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
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
        rec.set_face_normal(ray, (rec.p - self.center) / self.radius);
        rec.material = self.material;
        true
    }
}

pub enum Hittable {
    Sphere(Sphere),
    NoneObject,
}

pub struct HittableObjects {
    objects: Vec<Hittable>,
}

impl HittableObjects {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Hittable) {
        self.objects.push(hittable);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: Ray, ray_t: utility::Interval, rec: &mut HitRecord) -> bool {
        let mut closest_so_far = ray_t.max;
        let mut hit_something = false;
        for object in self.objects.iter() {
            match object {
                Hittable::Sphere(sphere) => {
                    if sphere.hit(ray, utility::Interval::from(ray_t.min, closest_so_far), rec) {
                        closest_so_far = rec.t;
                        hit_something = true;
                    }
                }
                _ => {}
            }
        }
        hit_something
    }
}
