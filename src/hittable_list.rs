use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(mut self) -> () {
        self.objects.clear()
    }
    pub fn add<T: Hittable+ 'static>(mut self, to_add: T) -> () {
        self.objects.push(Box::new(to_add));
    }
    pub fn hit(self, ray: Ray, ray_tmin: f32, ray_tmax: f32, mut hit_record: HitRecord) -> bool {
        let mut temp_record: HitRecord;
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = ray_tmax;

        for object in self.objects {
            if (*object).hit(ray, ray_tmin, closest_so_far, hit_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                hit_record = temp_record;
            }
        }
        return hit_anything;
    }
}