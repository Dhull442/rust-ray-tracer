mod imagedata;
use imagedata::ImageData;
mod perlinnoise;
use crate::image::hittable::material::texture::perlinnoise::PerlinNoise;
use crate::image::hittable::material::texture::TextureType::{
    ImageTexture, NoiseTexture, SolidColor,
};
use crate::image::util;
use crate::image::vector::{Color, Vector};
#[derive(Clone)]
enum TextureType {
    SolidColor {
        color: Color,
    },
    CheckerTexture {
        inv_scale: f64,
        even: Color,
        odd: Color,
    },
    ImageTexture {
        image: ImageData,
    },
    NoiseTexture {
        noise: PerlinNoise,
        scale: f64,
    },
}

impl Default for TextureType {
    fn default() -> Self {
        Self::SolidColor {
            color: Color::white(),
        }
    }
}
#[derive(Default, Clone)]
pub struct Texture {
    texture: TextureType,
    u: f64,
    v: f64,
    p: Vector,
}

impl Texture {
    fn new(texture: TextureType) -> Self {
        Self {
            texture,
            u: 0.0,
            v: 0.0,
            p: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new_solid(color: Color) -> Self {
        Self::new(TextureType::SolidColor { color })
    }

    pub fn new_checker(inv_scale: f64, even: Color, odd: Color) -> Self {
        Self::new(TextureType::CheckerTexture {
            inv_scale: 1.0 / inv_scale,
            even,
            odd,
        })
    }

    pub fn new_image(filename: String) -> Self {
        let image_data = ImageData::new(filename);
        Self::new(TextureType::ImageTexture { image: image_data })
    }

    pub fn new_perlin(scale: f64) -> Self {
        Self::new(NoiseTexture {
            noise: PerlinNoise::new(),
            scale,
        })
    }
    pub fn value(&self, u: f64, v: f64, p: Vector) -> Color {
        match self.texture {
            SolidColor { color } => color,
            TextureType::CheckerTexture { .. } => self.value_checker_texture(u, v, p),
            ImageTexture { .. } => self.value_image_texture(u, v, p),
            TextureType::NoiseTexture { .. } => self.value_noise_texture(u, v, p),
        }
    }

    fn value_checker_texture(&self, _u: f64, _v: f64, p: Vector) -> Color {
        let TextureType::CheckerTexture {
            inv_scale,
            even,
            odd,
        } = &self.texture
        else {
            return Color::black();
        };
        let xi = (inv_scale * p.x).floor() as i64;
        let yi = (inv_scale * p.y).floor() as i64;
        let zi = (inv_scale * p.z).floor() as i64;

        let is_even = (xi + yi + zi) % 2 == 0;

        if is_even {
            *even
        } else {
            *odd
        }
    }

    fn value_image_texture(&self, u: f64, v: f64, _p: Vector) -> Color {
        let ImageTexture { image } = &self.texture else {
            return Color::black();
        };
        if image.height() <= 0 {
            return Color::white();
        }
        let clamp_interval = util::Interval::new(0.0, 1.0);
        let i = (clamp_interval.clamp(u) * (image.width() as f64)) as u32;
        let j = ((1.0 - clamp_interval.clamp(v)) * (image.height() as f64)) as u32;
        image.pixel_data(i, j)
    }

    fn value_noise_texture(&self, _u: f64, _v: f64, p: Vector) -> Color {
        let NoiseTexture { noise, scale } = &self.texture else {
            return Color::black();
        };
        (1.0 + f64::sin(scale * p.z + 10.0 * noise.turb(p, 7))) * Color::new(0.5, 0.5, 0.5)
    }
}
