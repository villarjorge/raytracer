use std::array::{from_fn};

use rand::random_range;

use crate::point3::{Point3, Vector3, random_vector};

const POINT_COUNT: u32 = 256;

// To do: deal with the large amount of conversion done by these functions

pub struct PerlinNoise {
    pub random_vectors: [Vector3; POINT_COUNT as usize],
    pub x_perm: [u32; POINT_COUNT as usize],
    pub y_perm: [u32; POINT_COUNT as usize],
    pub z_perm: [u32; POINT_COUNT as usize],
}

pub fn create_perlin_noise() -> PerlinNoise {
    let random_vectors: [Vector3; POINT_COUNT as usize] = from_fn(|_i| random_vector(-1.0, 1.0));

    let x_perm: [u32; POINT_COUNT as usize] = perlin_generate_perm();
    let y_perm: [u32; POINT_COUNT as usize] = perlin_generate_perm();
    let z_perm: [u32; POINT_COUNT as usize] = perlin_generate_perm();

    PerlinNoise { random_vectors, x_perm, y_perm, z_perm }
}

fn perlin_generate_perm() -> [u32; POINT_COUNT as usize] {
    let mut perm: [u32; POINT_COUNT as usize] = from_fn(|i| i as u32);
    // You have to reverse it, something like n..1 will be silently initialized as empty
    for i in (1..POINT_COUNT as usize).rev() {
        let j: usize  = random_range(0..=i) as usize;
        perm.swap(i, j);
    }

    perm
}

impl PerlinNoise {
    // pub fn block_noise(&self, p: &Point3) -> f64 {
    //     let i: i64 = ((4.0*p.x) as i64) & 255;
    //     let j: i64 = ((4.0*p.y) as i64) & 255;
    //     let k: i64 = ((4.0*p.z) as i64) & 255;

    //     let float_index = self.x_perm[i as usize] ^ self.y_perm[j as usize] ^ self.z_perm[k as usize];
    //     self.random_floats[float_index as usize]
    // }

    pub fn noise(&self, p: &Point3) -> f64 {
        // Leave this here as a reminder
        // let u: f64 = p.x.fract();
        // let v: f64 = p.y.fract();
        // let w: f64 = p.z.fract();

        // Not the fractional part, since for 3.6 -> 0.6 but for -3.6 -> -3.6 - (-4) = 0.4
        let u: f64 = p.x - p.x.floor();
        let v: f64 = p.y - p.y.floor();
        let w: f64 = p.z - p.z.floor();

        let i: i64 = p.x.floor() as i64;
        let j: i64 = p.y.floor() as i64;
        let k: i64 = p.z.floor() as i64;

        let mut c: [[[Vector3; 2]; 2]; 2] = [[[Vector3::default(); 2]; 2]; 2];

        for di in 0_i64..2 {
            for dj in 0_i64..2 {
                for dk in 0_i64..2 {

                    let temp: [usize; 3] = [
                        ((i + di) & 255) as usize,
                        ((j + dj) & 255) as usize,
                        ((k + dk) & 255) as usize,
                    ];

                    let float_index: u32 = 
                        self.x_perm[temp[0]] ^ 
                        self.y_perm[temp[1]] ^ 
                        self.z_perm[temp[2]];

                    c[di as usize][dj as usize][dk as usize] = self.random_vectors[float_index as usize]
                }
            }
        }

        perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum: f64 = 0.0;
        let mut temp_p: Point3 = *p;
        let mut weight: f64 = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            // To do: implement mulassign for point3
            temp_p = 2.0*temp_p;
        }

        accum.abs()
    }
}

pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum: f64 = 0.0;

    for (i, internal1) in c.iter().enumerate() {
        for (j, internal2) in internal1.iter().enumerate() {
            for (k, coef) in internal2.iter().enumerate() {
                let i_float: f64 = i as f64;
                let j_float: f64 = j as f64;
                let k_float: f64 = k as f64;

                accum +=  (i_float*u + (1.0 - i_float)*(1.0 - u))
                        * (j_float*v + (1.0 - j_float)*(1.0 - v))
                        * (k_float*w + (1.0 - k_float)*(1.0 - w))
                        * coef;
            }
        }
    }
    accum
}

pub fn perlin_interpolation(c: [[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu: f64 = u*u*(3.0 - 2.0*u);
    let vv: f64 = v*v*(3.0 - 2.0*v);
    let ww: f64 = w*w*(3.0 - 2.0*w);

    let mut accum: f64 = 0.0;

    for (i, internal1) in c.iter().enumerate() {
        for (j, internal2) in internal1.iter().enumerate() {
            for (k, coef) in internal2.iter().enumerate() {
                let i_float: f64 = i as f64;
                let j_float: f64 = j as f64;
                let k_float: f64 = k as f64;

                let weight_v: Point3 = Point3 { x: u - i_float, y: v - j_float, z: w - k_float };

                accum +=  (i_float*uu + (1.0 - i_float)*(1.0 - uu))
                        * (j_float*vv + (1.0 - j_float)*(1.0 - vv))
                        * (k_float*ww + (1.0 - k_float)*(1.0 - ww))
                        * coef.dot(weight_v);
            }
        }
    }
    accum
}