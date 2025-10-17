use std::ops::Range;

use crate::point3::Point3;
use crate::ray::Ray;
use crate::material::Material;
use crate::unit_vector;

// To do: think about if the pointer to a material is necesary, or if its posible to change to something like the add funtion of hittable_list
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Point3,
    pub material: &'a Box<dyn Material>, 
    pub t: f64,
    pub front_face: bool,
}

pub fn create_hit_record<'a>(ray: &Ray, t: f64, outward_normal: Point3, material: &'a Box<dyn Material>) -> HitRecord<'a> {
    // Creates a HitRecord with all it's parameters from the colliding ray, the 
    // parameter of the ray at the point of collision, the normal at that point, and the material of the surface
    let p: Point3 = ray.at(t);

    let unit_outward_normal: Point3 = unit_vector(outward_normal);

    let front_face: bool = unit_outward_normal.dot(ray.direction) < 0.0;
    let normal: Point3 = if front_face {unit_outward_normal} else {-unit_outward_normal};

    HitRecord { p: p, normal: normal, material: material, t: t, front_face: front_face }
}

// For now, checking for a hit requires calculating it, so in the function that checks for hits return ether 
pub enum HitResult<'a> {
    DidNotHit,
    HitRecord(HitRecord<'a>)
}

// Instead of inheritance, create a trait that subsecuent objects will implement
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> HitResult;
}
