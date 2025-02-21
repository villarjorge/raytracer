use crate::point3::Point3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_record: HitRecord) -> bool;
}
