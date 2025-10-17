use std::ops::Range;

use crate::hittable::{HitRecord, HitResult, Hittable};
use crate::material::{BlackBody, Material};
use crate::ray::Ray;
use crate::point3::Point3;
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(mut self) -> () {
        self.objects.clear()
    }
    pub fn add<T: Hittable + 'static>(&mut self, to_add: T) -> () {
        self.objects.push(Box::new(to_add));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> HitResult {
        let m: Box<dyn Material> = Box::new(BlackBody{});
        let mut current_record: HitRecord = HitRecord{p: Point3::default(), normal: Point3::default(), material: &m, t: 0.0, front_face: false};
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.end; // max

        for object in &self.objects {
            match (*object).hit(&ray, ray_t.start..closest_so_far) {
                HitResult::DidNotHit => {},
                HitResult::HitRecord(hit_record) => {
                    hit_anything = true;
                    closest_so_far = hit_record.t;
                    current_record = hit_record
                }
            }
        }

        if hit_anything {
            return HitResult::HitRecord(current_record);
        }
        return HitResult::DidNotHit;
    }
}