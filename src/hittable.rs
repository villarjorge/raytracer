use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: Point3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = outward_normal.dot(ray.direction) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal*(-1.0)};
    }
}

pub fn create_hit_record(ray: &Ray, t: f64, outward_normal: Point3) -> HitRecord {
    let p: Point3 = ray.at(t);
    let front_face: bool = outward_normal.dot(ray.direction) < 0.0;
    let normal: Point3 = if front_face {outward_normal} else {-outward_normal};

    HitRecord { p: p, normal: normal, t: t, front_face: front_face }
}

// For now, checking for a hit requires calculating it, so in the function that checks for hits return ether 
pub enum HitResult {
    DidNotHit,
    HitRecord(HitRecord)
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult;
}
