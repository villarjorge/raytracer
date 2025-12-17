use std::ops::Range;
use std::sync::Arc;

use crate::aabb::{AABB, join_aabbs};
use crate::hittable::{HitRecord, Hittable, SurfaceCoordinate};
use crate::material::Material;
use crate::point3::{Point3, Vector3, cross, dot, unit_vector};
use crate::ray::Ray;

/// A triangle object hittable. Constructed with triangle
pub struct Triangle {
    /// Starting corner
    q: Point3,
    /// Vector representing the first side
    u: Vector3,
    /// Vector representing the second side
    v: Vector3,
    /// A vector normal to the plane defined by u and v, scaled a certain way
    w: Vector3,
    /// Material of the triangle
    material: Arc<dyn Material>,
    /// Bounding box of the triangle
    bounding_box: AABB,
    /// Normal defined by cross(u, v)
    normal: Vector3,
    /// The constant of the plane defined by the vectors
    d: f64,
}

fn create_aabb_para(q: Point3, u: Point3, v: Point3) -> AABB {
    // Create the bounding boxes for each diagonal and then join them
    let bounding_box0: AABB = AABB::from_points(q, q + u + v);
    let bounding_box1: AABB = AABB::from_points(q + u, q + v);

    join_aabbs(&bounding_box0, &bounding_box1)
}

impl Triangle {
    pub fn new(q: Point3, u: Vector3, v: Vector3, material: Arc<dyn Material>) -> Triangle {
        let bounding_box: AABB = create_aabb_para(q, u, v);

        let n: Vector3 = cross(&u, &v);
        let normal: Vector3 = unit_vector(n);
        let d: f64 = dot(&normal, &q);
        let w: Vector3 = n / dot(&n, &n);

        Triangle {
            q,
            u,
            v,
            w,
            material,
            bounding_box,
            normal,
            d,
        }
    }
    pub fn from_vertex_locations(
        v1: Point3,
        v2: Point3,
        v3: Point3,
        material: Arc<dyn Material>,
    ) -> Triangle {
        let q: Point3 = v1;
        let u: Point3 = q - v2;
        let v: Point3 = q - v3;

        let bounding_box: AABB = create_aabb_para(q, u, v);

        let n: Vector3 = cross(&u, &v);
        let normal: Vector3 = unit_vector(n);
        let d: f64 = dot(&normal, &q);
        let w: Vector3 = n / dot(&n, &n);

        Triangle {
            q,
            u,
            v,
            w,
            material,
            bounding_box,
            normal,
            d,
        }
    }
    pub fn from_vertex_locations_with_normal(
        v1: Point3,
        v2: Point3,
        v3: Point3,
        non_unit_normal: Vector3,
        material: Arc<dyn Material>,
    ) -> Triangle {
        let q: Point3 = v1;
        let u: Point3 = q - v2;
        let v: Point3 = q - v3;

        let bounding_box: AABB = create_aabb_para(q, u, v);

        let normal: Vector3 = unit_vector(non_unit_normal);
        let d: f64 = dot(&normal, &q);
        let w: Vector3 = non_unit_normal / dot(&non_unit_normal, &non_unit_normal);

        Triangle {
            q,
            u,
            v,
            w,
            material,
            bounding_box,
            normal,
            d,
        }
    }
}

impl Hittable for Triangle {
    /// Ray-triangle intersection will be determined in three steps:
    ///     1. Finding the plane Ax + By + Cz = d that contains that triangle,
    ///     2. Solving for the intersection of a ray and the triangle-containing plane,
    ///     3. Determining if the hit point lies inside the triangle.
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        let denominator: f64 = dot(&self.normal, &ray.direction);

        // No hit if the ray is parallel to the plane
        if denominator.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t: f64 = (self.d - dot(&self.normal, &ray.origin)) / denominator;
        if !ray_t.contains(&t) {
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
    alpha > 0.0 && beta > 0.0 && alpha + beta < 1.0
}
