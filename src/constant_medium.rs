use std::{
    ops::Range,
    sync::Arc,
};

use rand::random_range;

use crate::{
    aabb::AABB, 
    hittable::{HitResult, Hittable, SurfaceCoordinate, create_hit_record}, 
    material::{Isotropic, Material}, 
    point3::{Vector3, color::Color}, 
    ray::Ray, texture::{self, Texture, create_solid_color}
};

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub neg_inv_density: f64,
    // In the book phase function is typed as a shared pointer to a material, but when creating the object the material is always casted into an isotropic material
    // Needs to be typed this way to make it easier to shove it into a hit record
    pub phase_function: Arc<dyn Material> 
}

pub fn constant_medium(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> ConstantMedium {
    ConstantMedium { boundary, neg_inv_density: -1.0/density, phase_function: Arc::new(Isotropic{texture}) }
}

pub fn constant_medium_from_color(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> ConstantMedium {
    let texture: Arc<texture::SolidColor> = create_solid_color(color);
    ConstantMedium { boundary, neg_inv_density: -1.0/density, phase_function: Arc::new(Isotropic{texture}) }
}

impl Hittable for ConstantMedium {
    fn hit(&'_ self, ray: &Ray, ray_t: &Range<f64>) -> HitResult<'_> {
        // To do: improve this nested structure
        match self.boundary.hit(ray, &(-f64::INFINITY..f64::INFINITY)) {
            HitResult::DidNotHit => HitResult::DidNotHit,
            HitResult::HitRecord(hit_record1) => {
                match self.boundary.hit(ray, &(hit_record1.t+0.0001..f64::INFINITY)) {
                    HitResult::DidNotHit => HitResult::DidNotHit,
                    HitResult::HitRecord(hit_record2) => {
                        let record1_t: f64 = hit_record1.t.max(ray_t.start);
                        let record2_t: f64 = hit_record2.t.min(ray_t.end);

                        if record1_t >= record2_t {
                            return HitResult::DidNotHit;
                        }

                        let record1_t_nonzero: f64 = record1_t.max(0.0);

                        let ray_length: f64 = ray.direction.length();
                        let distance_inside_boundary: f64 = (record2_t - record1_t_nonzero) * ray_length;
                        // To avoid error[E0689]: can't call method `ln` on ambiguous numeric type `{float}`
                        // Use a taylor expansion around x = 0.5
                        let x: f64 = random_range(0.0..1.0);
                        // let expansion: f64 = 
                        //     - 0.693147 
                        //     + 2.0*(x - 0.5) 
                        //     - 2.0*(x - 0.5).powi(2) 
                        //     + 2.66667*(x - 0.5).powi(3) 
                        //     - 4.0*(x - 0.5).powi(4) 
                        //     + 6.4*(x - 0.5).powi(5);
                        // Turns out that to avoid the error you just had to define the variable with its type
                        // To do: improve the error message for error [E0689]
                        let hit_distance: f64 = self.neg_inv_density * x.ln();

                        if hit_distance > distance_inside_boundary { return HitResult::DidNotHit;}

                        let outward_normal: Vector3 = Vector3{x: 1.0, y: 0.0, z: 0.0}; // Arbitrary
                        let surface_coords: SurfaceCoordinate = hit_record1.surface_coords;

                        // To do: more dereference pointer to take reference of underlying
                        HitResult::HitRecord(
                            create_hit_record(ray, record1_t_nonzero + hit_distance/ray_length, outward_normal, &*self.phase_function, surface_coords)
                        )
                    }
                }
            }
        }
    }
    
    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}