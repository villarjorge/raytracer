use std::f64::consts::PI;
use std::ops::Range;
use std::rc::Rc;

use crate::point3::Point3;
use crate::hittable::{HitRecord, Hittable, SurfaceCoordinate};
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb::{AABB};

/// An sphere hittable (you know the one, round etc). Constructed with sphere
pub struct Sphere {
    center: Point3,
    radius: f64,
    // I couldn't change this pointer to a reference, because if I did, then the materials in main do not live long enough
    // Perhaps clone materials into hittables?
    material: Rc<dyn Material>,
    //material: &'a dyn Material,
    bounding_box: AABB
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        let radius_vector: Point3 = Point3 { x: radius, y: radius, z: radius };
        let bounding_box: AABB = AABB::from_points(center - radius_vector, center + radius_vector);
        Sphere { center, radius: radius.max(0.0), material, bounding_box}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        // This is the 1st hottest part of the code
        // Thanks to the BVH node, this part only takes 35% of cpu time, down from 86%
        let oc: Point3 = self.center - ray.origin;
        let a: f64 = ray.direction.length_squared();
        let h: f64 = oc.dot(ray.direction);
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = h*h - a*c;

        if discriminant < 0.0 {
            return false;
        } 
        let sqrt_discriminant: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (h - sqrt_discriminant)/a;
        
        if !ray_t.contains(&root) {
            root = (h+sqrt_discriminant)/a;
            if !ray_t.contains(&root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);

        let outward_normal: Point3 = (ray.at(root) - self.center)/self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        hit_record.material = self.material.clone();

        hit_record.surface_coords = get_sphere_uv(&outward_normal);
        
        true
    }
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

/// Compute apropiate coordinates in the surface of the sphere
///     p: a given point on the sphere of radius one, centered at the origin.
///     u: returned value \[0,1\] of angle around the Y axis from X=-1.
///     v: returned value \[0,1\] of angle from Y=-1 to Y=+1.
/// 
///     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
///     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
///     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
pub fn get_sphere_uv(p: &Point3) -> SurfaceCoordinate {
    let theta: f64 = (-p.y).acos();
    let phi: f64 = (-p.z).atan2(p.x) + PI;

    SurfaceCoordinate{u: phi/(2.0*PI), v: theta/PI}
}