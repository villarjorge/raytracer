use std::{ops::Range, rc::Rc};

use crate::aabb::{AABB, aabb_from_points, join_aabbs};
use crate::hittable_list::HittableList;
use crate::hittable::{HitResult, Hittable, SurfaceCoordinate, create_hit_record};
use crate::material::Material;
use crate::point3::{Point3, Vector3, cross, dot, unit_vector};
use crate::ray::Ray;

pub struct Parallelogram {
    q: Point3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    material: Rc<dyn Material>,
    bounding_box: AABB,
    normal: Vector3,
    d: f64,
}

fn create_aabb_para(q: Point3, u: Point3, v: Point3) -> AABB {
    // Create the bounding boxes for each diagonal and then join them
    let bounding_box0: AABB = aabb_from_points(q, q + u + v);
    let bounding_box1: AABB = aabb_from_points(q + u, q + v);

    join_aabbs(&bounding_box0, &bounding_box1)
}

pub fn create_parallelogram(q: Point3, u: Vector3, v: Vector3, material: Rc<dyn Material>) -> Parallelogram {
    let bounding_box: AABB = create_aabb_para(q, u, v);

    let n: Vector3 = cross(&u, &v);
    let normal: Vector3 = unit_vector(n);
    let d: f64 = dot(&normal, &q);
    let w: Vector3 = n / dot(&n, &n);

    Parallelogram { q, u, v, w, material, bounding_box, normal, d }
}

// To do: extend parallelogram to any polygon. How to do it efficiently and with little code?

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
        let planar_hitpoint_vector: Vector3  = ray.at(t) - self.q;
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

pub fn create_box(a: Point3, b: Point3, material: Rc<dyn Material>) -> HittableList {
    let mut sides: HittableList = HittableList::default();

    let vertex_min: Point3 = Point3 { x: a.x.min(b.x), y: a.y.min(b.y), z: a.z.min(b.z) };
    let vertex_max: Point3 = Point3 { x: a.x.max(b.x), y: a.y.max(b.y), z: a.z.max(b.z) };

    let dx: Point3 = Point3 { x: vertex_max.x - vertex_min.x, y: 0.0, z: 0.0 };
    let dy: Point3 = Point3 { x: 0.0, y: vertex_max.y - vertex_min.y, z: 0.0 };
    let dz: Point3 = Point3 { x: 0.0, y: 0.0, z: vertex_max.z - vertex_min.z };

    // To do: think of a better way to do this with a loop and an indicator
    sides.add(create_parallelogram(Point3 { x: vertex_min.x, y: vertex_min.y, z: vertex_max.z }, dx, dy, material.clone()));
    sides.add(create_parallelogram(Point3 { x: vertex_max.x, y: vertex_min.y, z: vertex_max.z }, -dz, dy, material.clone()));
    sides.add(create_parallelogram(Point3 { x: vertex_max.x, y: vertex_min.y, z: vertex_min.z }, -dx, dy, material.clone()));
    sides.add(create_parallelogram(Point3 { x: vertex_min.x, y: vertex_min.y, z: vertex_min.z }, dz, dy, material.clone()));
    sides.add(create_parallelogram(Point3 { x: vertex_min.x, y: vertex_max.y, z: vertex_max.z }, dx, -dz, material.clone()));
    sides.add(create_parallelogram(Point3 { x: vertex_min.x, y: vertex_min.y, z: vertex_min.z }, dx, dz, material.clone()));

    sides
}