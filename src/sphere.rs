use std::ops::Range;

use crate::point3::Point3;
use crate::hittable::{create_hit_record, HitRecord, HitResult, Hittable};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    // I couldn't change this pointer to a reference, because if I did, then the materials in main do not live long enough
    // Perhaps clone materials into hittables?
    pub material: Box<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
        // This is the hottest part of the code, taking 86% of cpu time
        let oc: Point3 = self.center - ray.origin;
        let a: f64 = ray.direction.length_squared();
        let h: f64 = oc.dot(ray.direction);
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = h*h - a*c;

        if discriminant < 0.0 {
            return HitResult::DidNotHit;
        } 
        let sqrt_discriminant: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (h - sqrt_discriminant)/a;
        if !ray_t.contains(&root) {
            root = (h+sqrt_discriminant)/a;
            if !ray_t.contains(&root) {
                return HitResult::DidNotHit;
            }
        }

        let outward_normal: Point3 = (ray.at(root) - self.center)/self.radius;
        // To deal with the material, dereference the pointer, then create a reference
        // To do: improve that
        let record: HitRecord = create_hit_record(ray, root, outward_normal, &*self.material);

        HitResult::HitRecord(record)
    }
}