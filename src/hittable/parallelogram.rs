use std::ops::Range;
use std::sync::Arc;

use crate::aabb::{AABB, join_aabbs};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{HitRecord, Hittable, SurfaceCoordinate};
use crate::material::Material;
use crate::point3::{Point3, Vector3, cross, dot, unit_vector};
use crate::ray::Ray;

/// A parallelogram object hittable. Constructed with parallelogram
pub struct Parallelogram {
    /// The constant of the plane defined by the vectors
    d: f64,
    /// Normal defined by cross(u, v)
    normal: Vector3,
    /// Starting corner
    q: Point3,
    /// Vector representing the first side
    u: Vector3,
    /// Vector representing the second side
    v: Vector3,
    /// A vector normal to the plane defined by u and v, scaled a certain way
    w: Vector3,
    /// Material of the parallelogram
    material: Arc<dyn Material>,
    /// Bounding box of the parallelogram
    bounding_box: AABB,
}

fn create_aabb_para(q: Point3, u: Point3, v: Point3) -> AABB {
    // Create the bounding boxes for each diagonal and then join them
    let bounding_box0: AABB = AABB::from_points(q, q + u + v);
    let bounding_box1: AABB = AABB::from_points(q + u, q + v);

    join_aabbs(&bounding_box0, &bounding_box1)
}

impl Parallelogram {
    pub fn new(q: Point3, u: Vector3, v: Vector3, material: Arc<dyn Material>) -> Parallelogram {
        let bounding_box: AABB = create_aabb_para(q, u, v);

        let n: Vector3 = cross(&u, &v);
        let normal: Vector3 = unit_vector(n);
        let d: f64 = dot(&normal, &q);
        let w: Vector3 = n / dot(&n, &n);

        Parallelogram {
            normal,
            d,
            q,
            u,
            v,
            w,
            material,
            bounding_box,
        }
    }
}

// To do: extend parallelogram to any polygon. How to do it efficiently and with little code?

impl Hittable for Parallelogram {
    /// Ray-parallelogram intersection will be determined in three steps:
    ///     1. Finding the plane Ax + By + Cz = d that contains that parallelogram,
    ///     2. Solving for the intersection of a ray and the parallelogram-containing plane,
    ///     3. Determining if the hit point lies inside the parallelogram.
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        let denominator: f64 = dot(&self.normal, &ray.direction);
        // let denominator: f64 = self.normal.dot(ray.direction);

        // Return false if the hit point parameter t is outside the ray interval.
        let t: f64 = (self.d - dot(&self.normal, &ray.origin)) / denominator;
        // if !ray_t.contains(&t) {
        if ray_t.end < t || t < ray_t.start {
            return false;
        }

        // No hit if the ray is parallel to the plane
        if denominator.abs() < 1e-8 {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection: Point3 = ray.at(t);
        let planar_hitpoint_vector: Vector3 = intersection - self.q;
        let alpha: f64 = dot(&self.w, &cross(&planar_hitpoint_vector, &self.v));
        let beta: f64 = dot(&self.w, &cross(&self.u, &planar_hitpoint_vector));

        if !is_interior(alpha, beta) {
            return false;
        }

        let surface_coords: SurfaceCoordinate = SurfaceCoordinate { u: alpha, v: beta };
        hit_record.surface_coords = surface_coords;

        hit_record.t = t;
        hit_record.p = intersection;
        hit_record.material = self.material.clone();
        hit_record.set_face_normal(ray, self.normal);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

/// Given the hit point in plane coordinates, return false if it is outside the primitive or true if it is inside
fn is_interior(alpha: f64, beta: f64) -> bool {
    let unit_interval: Range<f64> = 0.0..1.0;

    !(!unit_interval.contains(&alpha) || !unit_interval.contains(&beta))
}

/// Create a box consisting of six parallelograms
pub fn create_box(a: Point3, b: Point3, material: Arc<dyn Material>) -> HittableList {
    let mut sides: HittableList = HittableList::default();

    let vertex_min: Point3 = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let vertex_max: Point3 = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx: Point3 = Point3::new(vertex_max.x - vertex_min.x, 0.0, 0.0);
    let dy: Point3 = Point3::new(0.0, vertex_max.y - vertex_min.y, 0.0);
    let dz: Point3 = Point3::new(0.0, 0.0, vertex_max.z - vertex_min.z);

    // To do: think of a better way to do this with a loop and an indicator
    sides.add(Parallelogram::new(
        Point3::new(vertex_min.x, vertex_min.y, vertex_max.z),
        dx,
        dy,
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        Point3::new(vertex_max.x, vertex_min.y, vertex_max.z),
        -dz,
        dy,
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        Point3::new(vertex_max.x, vertex_min.y, vertex_min.z),
        -dx,
        dy,
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        Point3::new(vertex_min.x, vertex_min.y, vertex_min.z),
        dz,
        dy,
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        Point3::new(vertex_min.x, vertex_max.y, vertex_max.z),
        dx,
        -dz,
        material.clone(),
    ));
    sides.add(Parallelogram::new(
        Point3::new(vertex_min.x, vertex_min.y, vertex_min.z),
        dx,
        dz,
        material.clone(),
    ));

    sides
}
