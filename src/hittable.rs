use crate::point3::Point3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(mut self, ray: Ray, outward_normal: Point3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face =  outward_normal.dot(ray.direction) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal*(-1.0)};
    }
}

pub trait Hittable {
    fn hit(self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_record: HitRecord) -> bool;
}
