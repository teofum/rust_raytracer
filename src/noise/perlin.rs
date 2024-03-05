use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::vec4::{Point4, Vec4};

use super::Noise3D;

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct PerlinNoise3D {
    random_vec: [Vec4; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl PerlinNoise3D {
    pub fn new(rng: &mut Pcg64Mcg) -> Self {
        let mut random_vec = [Vec4::vec(0.0, 0.0, 0.0); POINT_COUNT];
        for i in 0..POINT_COUNT {
            random_vec[i] = Vec4::random_unit(rng);
        }

        let perm_x = Self::gen_perm(rng);
        let perm_y = Self::gen_perm(rng);
        let perm_z = Self::gen_perm(rng);

        PerlinNoise3D {
            random_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn gen_perm(rng: &mut Pcg64Mcg) -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i
        }

        Self::permute(&mut p, rng);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], rng: &mut Pcg64Mcg) {
        for i in (1..POINT_COUNT).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }

    fn trilinear_interpolation(c: [Vec4; 8], uvw: Vec4) -> f64 {
        let mut acc = 0.0;
        let (u, v, w) = uvw.xyz();
        let (uu, vv, ww) = uvw.map_components(|x| x * x * (3.0 - 2.0 * x)).xyz();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (fi, fj, fk) = (i as f64, j as f64, k as f64);
                    let v_weight = Vec4::vec(u - fi, v - fj, w - fk);
                    acc += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * Vec4::dot(&c[(i << 2) + (j << 1) + k], &v_weight);
                }
            }
        }

        acc
    }
}

impl Noise3D for PerlinNoise3D {
    type Output = f64;

    fn sample(&self, p: &Point4) -> Self::Output {
        let uvw = p.map_components(|x| x - x.floor());

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [Vec4::vec(0.0, 0.0, 0.0); 8];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[(di << 2) + (dj << 1) + dk] = self.random_vec[ //.
                        self.perm_x[((di as i32 + i) & 255) as usize]
                        ^ self.perm_y[((dj as i32 + j) & 255) as usize]
                        ^ self.perm_z[((dk as i32 + k) & 255) as usize]];
                }
            }
        }

        Self::trilinear_interpolation(c, uvw)
    }

    fn sample_turbulence(&self, p: &Point4, samples: usize) -> Self::Output {
        let mut acc = 0.0;
        let mut p = *p;
        let mut weight = 1.0;

        for _ in 0..samples {
            acc += weight * self.sample(&p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }
}
