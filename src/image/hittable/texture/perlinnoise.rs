use crate::image::util::random;
use crate::image::vector::{Color, Vector};
use rand::seq::SliceRandom;

const POINT_COUNT: usize = 256;
#[derive(Clone)]
pub struct PerlinNoise {
    rand_float: Vec<Vector>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl PerlinNoise {
    fn permute(perm: &mut Vec<u32>) {
        perm.shuffle(&mut rand::thread_rng());
    }

    fn perlin_generate_perm(perm: &mut Vec<u32>) {
        for i in 0..POINT_COUNT {
            perm.push(i as u32);
        }
        Self::permute(perm);
    }

    fn crossmult(a: f64, b: f64) -> f64 {
        a * b + (1.0 - a) * (1.0 - b)
    }
    fn trilinear_interpretation(c: Vec<f64>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += Self::crossmult(i as f64, u)
                        * Self::crossmult(j as f64, v)
                        * Self::crossmult(k as f64, w)
                        * c[(i * 4 + j * 2 + k) as usize];
                }
            }
        }
        accum
    }

    fn perlin_interpretation(c: Vec<Vector>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vector::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += Self::crossmult(i as f64, uu)
                        * Self::crossmult(j as f64, vv)
                        * Self::crossmult(k as f64, ww)
                        * c[(i * 4 + j * 2 + k) as usize].dot(weight);
                }
            }
        }
        accum
    }
    pub fn new() -> Self {
        let mut rand_float = Vec::new();
        for _ in (0..POINT_COUNT).step_by(1) {
            rand_float.push(Vector::random_interval(-1.0, 1.0));
        }
        let mut perm_x = Vec::new();
        let mut perm_y = Vec::new();
        let mut perm_z = Vec::new();
        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);
        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vector) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;
        let mut c = Vec::new();
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c.push(
                        self.rand_float[(self.perm_x[((i + di) & 255) as usize]
                            ^ self.perm_y[((j + dj) & 255) as usize]
                            ^ self.perm_z[((k + dk) & 255) as usize])
                            as usize],
                    );
                }
            }
        }
        Self::perlin_interpretation(c, u, v, w)
    }
}
