use std::ops::Range;

use crate::aabb::{AABB, create_aabb_from_points, join_aabbs};
use crate::hittable::{HitResult, Hittable};
use crate::point3::Point3;
use crate::ray::Ray;

pub struct HittableList {
    // HittableList is a list of objects with the hittable trait. 
    // The objects can be of diferent sizes, so it is necesary to use a reference or a pointer. Using a pointer to deal with less lifetimes
    // this does not need to be a vector, it is just like this to make initialization easier
    pub objects: Vec<Box<dyn Hittable>>,
    pub bounding_box: AABB
}

impl HittableList {
    pub fn clear(mut self) {
        self.objects.clear()
    }
    pub fn add<T: Hittable + 'static>(&mut self, to_add: T)  {
        self.bounding_box = join_aabbs(&self.bounding_box, to_add.bounding_box());
        self.objects.push(Box::new(to_add));
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: Vec::new(), bounding_box: create_aabb_from_points(Point3::default(), Point3::default()) }
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