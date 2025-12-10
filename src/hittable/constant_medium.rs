use std::{
    ops::Range,
    rc::Rc,
};

use rand::random_range;

use crate::{
    aabb::AABB, 
    hittable::{HitRecord, Hittable, SurfaceCoordinate}, 
    material::{Isotropic, Material}, 
    point3::{Vector3, color::Color}, 
    ray::Ray, texture::{self, SolidColor, Texture}
};

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub neg_inv_density: f64,
    // In the book phase function is typed as a shared pointer to a material, but when creating the object the material is always casted into an isotropic material
    // Needs to be typed this way to make it easier to shove it into a hit record
    pub phase_function: Rc<dyn Material> 
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, texture: Rc<dyn Texture>) -> ConstantMedium {
        ConstantMedium { boundary, neg_inv_density: -1.0/density, phase_function: Rc::new(Isotropic{texture}) }
    }

    pub fn from_color(boundary: Rc<dyn Hittable>, density: f64, color: Color) -> ConstantMedium {
        let texture: Rc<texture::SolidColor> = SolidColor::new(color);
        ConstantMedium { boundary, neg_inv_density: -1.0/density, phase_function: Rc::new(Isotropic{texture}) }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        let mut hit_record1: HitRecord = hit_record.clone();
        let mut hit_record2: HitRecord = hit_record.clone();

        if !self.boundary.hit(ray, &(-f64::INFINITY..f64::INFINITY), &mut hit_record1) {
            return false;
        }

        if !self.boundary.hit(ray, &(hit_record1.t+0.0001..f64::INFINITY), &mut hit_record2) {
            return false;
        }

        hit_record1.t = hit_record1.t.max(ray_t.start);
        hit_record2.t = hit_record2.t.min(ray_t.end);

        hit_record1.t = hit_record1.t.max(0.0);

        let ray_length: f64 = ray.direction.length();
        let distance_inside_boundary: f64 = (hit_record2.t - hit_record1.t) * ray_length;
        let x: f64 = random_range(0.0..1.0);
        let hit_distance: f64 = self.neg_inv_density * x.ln();

        if hit_distance > distance_inside_boundary { return false; }

        hit_record.t = hit_record1.t + hit_distance / ray_length;
        hit_record.p = ray.at(hit_record.t);

        hit_record.normal = Vector3{x: 1.0, y: 0.0, z: 0.0}; // Arbitrary
        hit_record.front_face = true; // Also arbitrary
        hit_record.surface_coords = SurfaceCoordinate{u: 0.0, v:0.0};

        hit_record.material = self.phase_function.clone();

        true
    }
    
    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}