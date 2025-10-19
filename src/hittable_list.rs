use std::ops::Range;

use crate::hittable::{HitResult, Hittable};
// use crate::material::{BlackBody, Material};
use crate::ray::Ray;
// use crate::point3::Point3;
pub struct HittableList {
    // HittableList is a list of objects with the hittable trait. 
    // The objects can be of diferent sizes, so it is necesary to use a reference or a pointer. Using a pointer to deal with less lifetimes
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(mut self) {
        self.objects.clear()
    }
    pub fn add<T: Hittable + 'static>(&mut self, to_add: T)  {
        self.objects.push(Box::new(to_add));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> HitResult {
        let mut current_result: HitResult = HitResult::DidNotHit;
        let mut closest_so_far: f64 = ray_t.end; // max

        for object in &self.objects {
            match (*object).hit(ray, ray_t.start..closest_so_far) {
                HitResult::DidNotHit => {},
                HitResult::HitRecord(hit_record) => {
                    closest_so_far = hit_record.t;
                    current_result = HitResult::HitRecord(hit_record);
                }
            }
        }
        
        current_result
    }
}