use std::ops::Range;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::point3::{Point3, Vector3, point_from_array, rotate_y};
use crate::ray::Ray;

#[derive(Clone, Copy)]
/// The coordinates in a given surface
pub struct SurfaceCoordinate {
    pub u: f64,
    pub v: f64,
}

/// A grouping of parameters related to the ray-object intersection
#[derive(Clone)]
pub struct HitRecord {
    /// Point of intersection
    pub p: Point3,
    /// Normal at that point
    pub normal: Vector3,
    /// Material at the given intersection
    // To do: change this to a Rc<dyn Material>
    pub material: Arc<dyn Material>, // &'a Box<dyn Material>
    /// Parameter of the ray at the intersection
    pub t: f64,
    /// Coordinates of the point of intersection in the surface
    pub surface_coords: SurfaceCoordinate,
    /// Whether the intersection is on the front or back of the surface
    pub front_face: bool,
}

/// Creates a HitRecord given some of it's parameters:
///     - ray,
///     - t: parameter of the ray,
///     - outward_normal: normal of the surface, assumed to be unit lenght
///     - material
///     - surface_coords
impl HitRecord {
    // pub fn new<'a>(ray: &Ray, t: f64, outward_normal: Vector3, material: &'a dyn Material, surface_coords: SurfaceCoordinate) -> HitRecord<'a> {
    //     let p: Point3 = ray.at(t);

    //     // let unit_outward_normal: Point3 = unit_vector(outward_normal);

    //     let front_face: bool = outward_normal.dot(ray.direction) < 0.0;
    //     let normal: Vector3 = if front_face {outward_normal} else {-outward_normal};

    //     HitRecord {p, normal, material, t, surface_coords, front_face}
    // }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = outward_normal.dot(ray.direction) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

// Instead of inheritance, create a trait that subsecuent objects will implement
// To do: Think about traits versus enums for objects. Eg: scene object hittable that contains sphere, quadrilateral, disk etc
// Having every object be a variant of Hittable could allow to have a
// more complex hittable list with vectors for each object. The problem would be having too many variants that need to be handeled
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &AABB; // Needed since hittables will be behind pointers that will be dereferenced
}

/// An instance of translation
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector3,
    bounding_box: AABB,
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_ray: Ray = Ray::new(ray.origin - self.offset, ray.direction);

        // Check for intersection with the new ray
        if !self.object.hit(&offset_ray, ray_t, hit_record) {
            return false;
        }

        // Move the intersection point forwards by the offset
        hit_record.p = hit_record.p + self.offset;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector3) -> Translate {
        let bounding_box: AABB = (*object.bounding_box()).clone();
        Translate {
            object,
            offset,
            bounding_box: bounding_box + offset,
        }
    }
}

/// An instance of rotation on the y axis
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        // To do: compare for performance: using rotate_y or copy pasting the same block of code four times to rotate
        // Transform the ray from world space to object space.
        let origin: Point3 = rotate_y(&ray.origin, self.cos_theta, self.sin_theta);
        let direction: Point3 = rotate_y(&ray.direction, self.cos_theta, self.sin_theta);

        let rotated_ray: Ray = Ray::new(origin, direction);

        // Determine whether an intersection exists in object space (and if so, where).
        if !self.object.hit(&rotated_ray, ray_t, hit_record) {
            return false;
        }
        // Transform the intersection from object space back to world space.
        hit_record.p = rotate_y(&hit_record.p, self.cos_theta, -self.sin_theta);
        hit_record.normal = rotate_y(&hit_record.normal, self.cos_theta, -self.sin_theta);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle_in_degrees: f64) -> RotateY {
        let radians: f64 = angle_in_degrees.to_radians();
        let sin_theta: f64 = radians.sin();
        let cos_theta: f64 = radians.cos();
        let bounding_box: AABB = object.bounding_box().clone();

        let mut minimum: [f64; 3] = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut maximum: [f64; 3] = [-f64::INFINITY, -f64::INFINITY, -f64::INFINITY];

        for i_int in 0..2 {
            for j_int in 0..2 {
                for k_int in 0..2 {
                    let i: f64 = i_int as f64;
                    let j: f64 = j_int as f64;
                    let k: f64 = k_int as f64;

                    // To do: make the fields of bounding box public to not index them like this
                    // To do: possible opportunity to use arrays here for better performance (the compiler will paralelize?)
                    let x: f64 = i * bounding_box[0].end + (1.0 - i) * bounding_box[0].start;
                    let y: f64 = j * bounding_box[1].end + (1.0 - j) * bounding_box[1].start;
                    let z: f64 = k * bounding_box[2].end + (1.0 - k) * bounding_box[2].start;

                    let x_new = cos_theta * x + sin_theta * z;
                    let z_new: f64 = -sin_theta * x + cos_theta * z;

                    let tester: Vector3 = Vector3 {
                        x: x_new,
                        y,
                        z: z_new,
                    };

                    for c in 0..2 {
                        minimum[c] = minimum[c].min(tester[c as u8]);
                        maximum[c] = maximum[c].min(tester[c as u8]);
                    }
                }
            }
        }

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bounding_box: AABB::from_points(point_from_array(minimum), point_from_array(maximum)),
        }
    }
}

// To do: the same way you can rotate and translate, create a scale instance.
// Making a hittable bigger is the same as making the ray smaller. You would also have to scale up the bounding box

pub mod constant_medium;
pub mod hittable_list;
pub mod parallelogram;
pub mod quadric;
pub mod sphere;
pub mod triangle;
