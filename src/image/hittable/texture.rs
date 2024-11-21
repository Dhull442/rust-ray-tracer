use crate::image::vector::{Color, Vector};

enum TextureType{
    SolidColor{color: Color},
    CheckerTexture{},
}
struct Texture {
    texture: TextureType,
    u: f64,
    v: f64,
    albedo: Color,
    p: Vector,
}

impl Texture {
    pub fn new_solid_color(albedo: Color) -> Self{
        Self{
            texture: TextureType::SolidColor,
            u: 0.0,
            v: 0.0,
            albedo,
            p: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub fn

    pub fn value(&self, u: f64, v: f64, p: &Vector) -> Color {
        self.albedo
    }
}