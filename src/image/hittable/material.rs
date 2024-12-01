use crate::image::ray::Ray;
use crate::image::util;
use crate::image::vector::{Color, Vector};
// use onb::ONB;
use std::f64::consts::PI;
use texture::Texture;
use crate::image::hittable::material::pdf::PDF;

pub mod onb;
pub mod pdf;
pub mod texture;

#[derive(Clone)]
pub enum MaterialType {
    Lambertian { texture: Texture },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
    DiffuseLight { texture: Texture },
    Isotropic { texture: Texture },
}
impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::Dielectric {
            refraction_index: 1.0,
        }
    }
}

#[derive(Default, Clone)]
pub struct Material {
    material: MaterialType,
}

impl Material {
    pub fn new_lambertian(texture: Texture) -> Self {
        Self {
            material: MaterialType::Lambertian { texture },
        }
    }

    pub fn new_diffuse_light(texture: Texture) -> Self {
        Self {
            material: MaterialType::DiffuseLight { texture },
        }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Self {
            material: MaterialType::Metal {
                albedo,
                fuzz: fuzz.min(1.0),
            },
        }
    }

    pub fn new_dielectric(refraction_index: f64) -> Self {
        Self {
            material: MaterialType::Dielectric { refraction_index },
        }
    }

    pub fn new_isotropic(texture: Texture) -> Self {
        Self {
            material: MaterialType::Isotropic { texture },
        }
    }

    pub fn scattering_pdf(&self, ray_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        match self.material {
            MaterialType::Lambertian { .. } => {
                let cos_theta = rec.normal.dot(scattered.direction().unit_vector());
                f64::max(0.0, cos_theta / PI)
            }
            MaterialType::Isotropic { .. } => 1.0 / (4.0 * PI),
            _ => 1.0,
        }
    }
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        match self.material {
            MaterialType::Lambertian { .. } => {
                self.scatter_lambertian(ray_in, hit_record, scatter_record)
            }
            MaterialType::Metal { .. } => {
                self.scatter_metal(ray_in, hit_record, scatter_record)
            }
            MaterialType::Dielectric { .. } => {
                self.scatter_dielectric(ray_in, hit_record, scatter_record)
            }
            MaterialType::Isotropic { .. } => {
                self.scatter_isotropic(ray_in, hit_record, scatter_record)
            }
            _ => false,
        }
    }

    pub fn emitted(&self, ray_in: &Ray, rec: &HitRecord) -> Color {
        match &self.material {
            MaterialType::DiffuseLight { .. } => self.emitted_diffuse_light(ray_in, rec),
            _ => Color::black(),
        }
    }

    fn emitted_diffuse_light(&self, ray_in: &Ray, rec: &HitRecord) -> Color {
        let MaterialType::DiffuseLight { texture } = &self.material else {
            return Color::black();
        };
        if !rec.front_face {
            return Color::black();
        }
        texture.value(rec.u, rec.v, rec.p)
    }

    fn scatter_isotropic(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        let MaterialType::Isotropic { texture } = &self.material else {
            return false;
        };
        // *ray_scattered = Ray::new_time(rec.p, Vector::random_unit_vector(), ray_in.time());
        scatter_record.attenuation = texture.value(rec.u, rec.v, rec.p);
        scatter_record.skip_pdf = false;
        scatter_record.pdf = PDF::new_sphere();
        true
    }
    fn scatter_lambertian(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        let MaterialType::Lambertian { texture } = self.material.clone() else {
            return false;
        };
        // let uvw = ONB::new(rec.normal);
        // let scatter_direction = uvw.transform(Vector::random_unit_vector());
        // *ray_scattered = Ray::new_time(rec.p, scatter_direction.unit_vector(), ray_in.time());
        scatter_record.skip_pdf=false;
        scatter_record.pdf = PDF::new_cosine(rec.normal);
        scatter_record.attenuation = texture.value(rec.u, rec.v, rec.p);
        true
    }
    fn scatter_metal(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        let MaterialType::Metal { albedo, fuzz } = self.material else {
            return false;
        };
        let mut reflected = Vector::reflect(&(ray_in.direction()), rec.normal);
        reflected = reflected.unit_vector() + fuzz * Vector::random_unit_vector();
        scatter_record.attenuation = albedo;
        scatter_record.skip_pdf = true;
        scatter_record.skip_pdf_ray = Ray::new_time(rec.p, reflected, ray_in.time());
        true
    }

    fn scatter_dielectric(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        let MaterialType::Dielectric { refraction_index } = self.material else {
            return false;
        };
        let ri = if rec.front_face {
            1.0 / refraction_index
        } else {
            refraction_index
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
        scatter_record.attenuation = Color::white();
        scatter_record.skip_pdf = true;
        scatter_record.skip_pdf_ray = Ray::new_time(rec.p, direction, ray_in.time());
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

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: PDF,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
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
