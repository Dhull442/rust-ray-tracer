use crate::image::hittable::material::onb::ONB;
use crate::image::hittable::HittableObjects;
use crate::image::util::random_interval;
use crate::image::vector::Vector;
use std::f64::consts::PI;

#[derive(Clone, Default)]
pub enum PDFType {
    #[default]
    Sphere,
    Cosine {
        uvw: ONB,
    },
    Lights {
        objects: HittableObjects,
        origin: Vector,
    },
    MixPdfs {
        pdfs: Vec<PDF>
    }
}

#[derive(Default,Clone)]
pub struct PDF {
    pdf_type: PDFType,
}

impl PDF {
    pub fn new(pdf_type: PDFType) -> PDF {
        Self { pdf_type }
    }

    pub fn new_sphere() -> Self {
        Self::new(PDFType::Sphere)
    }

    pub fn new_cosine(w: Vector) -> Self {
        Self::new(PDFType::Cosine { uvw: ONB::new(w) })
    }

    pub fn new_mix() -> Self{
        Self::new(PDFType::MixPdfs {pdfs: Vec::new()})
    }

    pub fn add_to_mix(&mut self, pdf: Self){
        match &self.pdf_type {
            PDFType::MixPdfs { pdfs } => {
                let mut pdfs_new = pdfs.clone();
                pdfs_new.push(pdf);
                self.pdf_type = PDFType::MixPdfs{pdfs: pdfs_new};
            }
            _ => {}
        }
    }
    pub fn new_lights(objects: &HittableObjects, origin: Vector) -> Self {
        Self::new(PDFType::Lights {
            objects: objects.clone(),
            origin,
        })
    }

    pub fn value(&self, direction: Vector) -> f64 {
        match &self.pdf_type {
            PDFType::Cosine { uvw } => f64::max(direction.unit_vector().dot(uvw.w()) / PI, 0.0),
            PDFType::Lights { objects, origin } => objects.pdf_value(*origin, direction),
            PDFType::MixPdfs { pdfs } => {
                let n = pdfs.len() as f64;
                let frac = 1.0 / n;
                let mut value = 0.0;
                for pdf in pdfs {
                    value += frac * (pdf.value(direction));
                }
                value
            }
            _ => 0.001,
        }
    }
    pub fn generate(&self) -> Vector {
        match &self.pdf_type {
            PDFType::Sphere => Vector::random_unit_vector(),
            PDFType::Cosine { uvw } => uvw.transform(Vector::random_unit_vector()),
            PDFType::Lights { objects, origin } => objects.random(*origin),
            PDFType::MixPdfs {pdfs} => {
                let idx = random_interval(0., pdfs.len() as f64).floor() as usize;
                pdfs[idx].generate()
            }
            _ => Vector::new(1.0,0.0,0.0)
        }
    }
}
