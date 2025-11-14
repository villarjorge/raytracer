use std::ops::Range;

use crate::point3::Point3;
use crate::hittable::{create_hit_record, HitRecord, HitResult, Hittable};
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb::{AABB, create_aabb_from_points};

pub struct Sphere {
    center: Point3,
    radius: f64,
    // I couldn't change this pointer to a reference, because if I did, then the materials in main do not live long enough
    // Perhaps clone materials into hittables?
    material: Box<dyn Material>,
    bounding_box: AABB
}

pub fn create_sphere(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
    let radius_vector: Point3 = Point3 { x: radius, y: radius, z: radius };
    let bounding_box: AABB = create_aabb_from_points(center - radius_vector, center + radius_vector);
    Sphere { center, radius: radius.max(0.0), material, bounding_box}
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
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}