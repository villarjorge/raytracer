use crate::hittable::{HitRecord, HitResult, Hittable};
use crate::ray::Ray;
use crate::point3::Point3;
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(mut self) -> () {
        self.objects.clear()
    }
    pub fn add<T: Hittable+ 'static>(&mut self, to_add: T) -> () {
        self.objects.push(Box::new(to_add));
    }
    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult {
        let mut current_record: HitRecord = HitRecord{p: Point3{x:0.0, y:0.0, z:0.0}, normal:Point3{x:0.0, y:0.0, z:0.0}, t:0.0, front_face: false};
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_tmax;

        for object in &self.objects {
            match (*object).hit(&ray, ray_tmin, closest_so_far) {
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