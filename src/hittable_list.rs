use std::ops::Range;

use crate::aabb::{AABB, join_aabbs};
use crate::hittable::{HitResult, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    // HittableList is a list of objects with the hittable trait. 
    // The objects can be of diferent sizes, so it is necesary to use a reference or a pointer. 
    // Using a pointer leads to dealing with less lifetimes, but to sort a type Vec<Box<dyn Hittable>> you need to 
    // take references of Box<dyn Hittable>, which is discouraged. The ownership is more clear, since Vec owns Box which owns the Hittable
    // Using references leads to more lifetimes. It also leads to worse ownership, since the vec does not own anything. 
    // Idealy all objects would be stored contiguously on the heap to make performance better. One way to do this would be to 
    // have vectors for each primitive
    // this does not need to be a vector, it is just like this to make initialization easier
    pub objects: Vec<Box<dyn Hittable>>,
    pub bounding_box: AABB
}

impl HittableList {
    pub fn clear(mut self) {
        self.objects.clear()
    }
    pub fn add<T: Hittable + 'static>(&mut self, to_add: T) {
        self.bounding_box = join_aabbs(&self.bounding_box, to_add.bounding_box());
        self.objects.push(Box::new(to_add));
    }
    pub fn add_pointer(&mut self, to_add: Box<dyn Hittable>) {
        self.bounding_box = join_aabbs(&self.bounding_box, to_add.bounding_box());
        self.objects.push(to_add);
    }
}

impl Hittable for HittableList {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
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
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}