use crate::image::ray::Ray;
use crate::image::util;
use crate::image::vector::{Color, Vector};

#[derive(Default, Copy, Clone)]
pub enum MaterialType {
    #[default]
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Default, Copy, Clone)]
pub struct Material {
    material: MaterialType,
    albedo: Color,
    fuzz: f64,
    refraction_index: f64,
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self {
            material: MaterialType::Lambertian,
            albedo,
            fuzz: 0.0,
            refraction_index: 0.0,
        }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Self {
            material: MaterialType::Metal,
            albedo,
            fuzz: fuzz.min(1.0),
            refraction_index: 0.0,
        }
    }

    pub fn new_dielectric(refraction_index: f64) -> Self {
        Self {
            material: MaterialType::Dielectric,
            albedo: Color::white(),
            fuzz: 0.0,
            refraction_index,
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
                self.scatter_lambertian(ray_in, rec, attenuation, ray_scattered)
            }
            MaterialType::Metal => {
                self.scatter_metal(ray_in, rec, attenuation, ray_scattered)
            }
            MaterialType::Dielectric => {
                self.scatter_dielectric(ray_in, rec, attenuation, ray_scattered)
            }
        }
    }
    pub fn scatter_lambertian(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vector::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *ray_scattered = Ray::new_time(rec.p, scatter_direction, ray_in.time());
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
        *ray_scattered = Ray::new_time(rec.p, reflected, ray_in.time());
        *attenuation = self.albedo;
        ray_scattered.direction().dot(rec.normal) > 0.0
    }

    pub fn scatter_dielectric(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = (-1.0 * unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > util::random() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };
        *attenuation = self.albedo;
        *ray_scattered = Ray::new_time(rec.p, direction, ray_in.time());
        true
    }

    fn reflectance(cosine: f64, ri: f64) -> f64 {
        let r0 = ((1.0 - ri) / (1.0 + ri)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub p: Vector,
    pub t: f64,
    pub normal: Vector,
    pub front_face: bool,
    pub material: Material,
    pub u: f64,
    pub v: f64,
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