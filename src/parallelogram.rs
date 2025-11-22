use std::{ops::Range, rc::Rc};

use crate::{aabb::{AABB, create_aabb_from_points, join_aabbs}, hittable::{HitResult, Hittable, SurfaceCoordinate, create_hit_record}, material::Material, point3::{Point3, cross, dot, unit_vector}, ray::Ray};

pub struct Parallelogram {
    q: Point3,
    u: Point3,
    v: Point3,
    w: Point3,
    material: Rc<dyn Material>,
    bounding_box: AABB,
    normal: Point3,
    d: f64,
}

fn create_aabb_para(q: Point3, u: Point3, v: Point3) -> AABB {
    // Create the bounding boxes for each diagonal and then join them
    let bounding_box0: AABB = create_aabb_from_points(q, q + u + v);
    let bounding_box1: AABB = create_aabb_from_points(q + u, q + v);

    join_aabbs(&bounding_box0, &bounding_box1)
}

pub fn create_parallelogram(q: Point3, u: Point3, v: Point3, material: Rc<dyn Material>) -> Parallelogram {
    let bounding_box: AABB = create_aabb_para(q, u, v);

    let n: Point3 = cross(&u, &v);
    let normal: Point3 = unit_vector(n);
    let d: f64 = dot(&normal, &q);
    let w: Point3 = n / dot(&n, &n);

    Parallelogram { q, u, v, w, material, bounding_box, normal, d }
}

impl Hittable for Parallelogram {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
        let denominator: f64 = dot(&self.normal, &ray.direction);

        // No hit if the ray is parallel to the plane
        if denominator.abs() < 1e-8 {
            return HitResult::DidNotHit;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t: f64 = (self.d - dot(&self.normal, &ray.origin))/denominator;
        if !ray_t.contains(&t) {
            return HitResult::DidNotHit;
        }
        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let planar_hitpoint_vector  = ray.at(t) - self.q;
        let alpha: f64 = dot(&self.w, &cross(&planar_hitpoint_vector, &self.v));
        let beta: f64 = dot(&self.w, &cross(&self.u, &planar_hitpoint_vector));

        if !is_interior(alpha, beta) {
            return HitResult::DidNotHit;
        }

        let surface_coords: SurfaceCoordinate = SurfaceCoordinate { u: alpha, v: beta };

        HitResult::HitRecord(create_hit_record(ray, t, self.normal, &*self.material, surface_coords))        
    }
    
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

fn is_interior(alpha: f64, beta: f64) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise set the hit record UV coordinates and return true.

    let unit_interval: Range<f64> = 0.0..1.0;

    if !unit_interval.contains(&alpha) || !unit_interval.contains(&beta) {
        return false;
    }

    true
}