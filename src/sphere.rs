use std::ops::Range;

use crate::point3::Point3;
use crate::hittable::{create_hit_record, HitRecord, HitResult, Hittable};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a Box<dyn Material>
}

impl <'a>Hittable for Sphere<'_> {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> HitResult {
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
        let mut root: f64 = (h-sqrt_discriminant)/a;
        if ray_t.contains(&root) == false {
            root = (h+sqrt_discriminant)/a;
            if ray_t.contains(&root) == false {
                return HitResult::DidNotHit;
            }
        }

        let outward_normal: Point3 = (ray.at(root) - self.center)/self.radius;
        let record: HitRecord = create_hit_record(ray, root, outward_normal, self.material);

        return HitResult::HitRecord(record);
    }
}