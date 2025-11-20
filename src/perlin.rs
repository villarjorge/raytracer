use std::array::from_fn;

use rand::random_range;

use crate::point3::Point3;

const POINT_COUNT: u32 = 256;

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
    let mut perm: [u32; POINT_COUNT as usize] = from_fn(|i| i as  u32);

    for i in POINT_COUNT-1..0 {
        let target: u32 = random_range(0..i);
        let temp: u32 = perm[i as usize];
        perm[i as usize] = perm[target as usize];
        perm[target as usize] = temp;
    }

    perm
}

impl PerlinNoise {
    pub fn noise(&self, p: &Point3) -> f64 {
        let i: i64 = ((4.0*p.x) as i64) & 255;
        let j: i64 = ((4.0*p.y) as i64) & 255;
        let k: i64 = ((4.0*p.z) as i64) & 255;

        let float_index: u32 = self.x_perm[i as usize] ^ self.y_perm[j as usize] ^ self.z_perm[k as usize];

        self.random_floats[float_index as usize]
    }
}