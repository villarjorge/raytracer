use std::array::{from_fn};

use rand::random_range;
//use rand::seq::SliceRandom;

use crate::point3::Point3;

const POINT_COUNT: u32 = 256;

// To do: deal with the large amount of conversion done by these functions

pub struct PerlinNoise {
    pub random_floats: [f64; POINT_COUNT as usize],
    pub x_perm: [u32; POINT_COUNT as usize],
    pub y_perm: [u32; POINT_COUNT as usize],
    pub z_perm: [u32; POINT_COUNT as usize],
}

pub fn create_perlin_noise() -> PerlinNoise {
    let random_floats: [f64; POINT_COUNT as usize] = from_fn(|_i| random_range(0.0_f64..1.0));

    let x_perm: [u32; POINT_COUNT as usize] = perlin_generate_perm();
    let y_perm: [u32; POINT_COUNT as usize] = perlin_generate_perm();
    let z_perm: [u32; POINT_COUNT as usize] = perlin_generate_perm();

    PerlinNoise { random_floats, x_perm, y_perm, z_perm }
}

fn perlin_generate_perm() -> [u32; POINT_COUNT as usize] {
    let mut perm: [u32; POINT_COUNT as usize] = from_fn(|i| i as u32);
    //perm.shuffle(&mut rand::rng());
    // You have to reverse it, something like n..1 will be silently initialized as empty
    for i in (1..POINT_COUNT as usize).rev() {
        let j: usize  = random_range(0..=i) as usize;
        perm.swap(i, j);
    }

    perm
}

impl PerlinNoise {
    pub fn noise(&self, p: &Point3) -> f64 {
        let i: i64 = ((4.0*p.x) as i64) & 255;
        let j: i64 = ((4.0*p.y) as i64) & 255;
        let k: i64 = ((4.0*p.z) as i64) & 255;

        let float_index = self.x_perm[i as usize] ^ self.y_perm[j as usize] ^ self.z_perm[k as usize];
        self.random_floats[float_index as usize]
    }

    pub fn noise2(&self, p: &Point3) -> f64 {
        let u: f64 = p.x.fract();
        let v: f64 = p.y.fract();
        let w: f64 = p.z.fract();

        let i: usize = p.x.floor() as usize;
        let j: usize = p.y.floor() as usize;
        let k: usize = p.z.floor() as usize;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0_usize..2 {
            for dj in 0_usize..2 {
                for dk in 0_usize..2 {
                    let float_index: u32 = 
                        self.x_perm[(i + di) & 255] ^ 
                        self.y_perm[(j + dj) & 255] ^ 
                        self.z_perm[(k + dk) & 255];
                    c[di][dj][dk] = self.random_floats[float_index as usize]
                }
            }
        }

        return trilinear_interp(c, u, v, w);
    }
}

pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum: f64 = 0.0;

    for i in 0_usize..2 {
        for j in 0_usize..2 {
            for k in 0_usize..2 {
                let i_float: f64 = i as f64;
                let j_float: f64 = j as f64;
                let k_float: f64 = k as f64;

                accum +=  (i_float*u + (1.0 - i_float)*(1.0 - u))
                        * (j_float*v + (1.0 - j_float)*(1.0 - v))
                        * (k_float*w + (1.0 - k_float)*(1.0 - w))
                        * c[i][j][k];
            }
        }
    }
    accum
}