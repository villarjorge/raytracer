use std::f64::INFINITY;
use std::ops::Range;
use std::rc::Rc;

use crate::aabb::{AABB, aabb_from_points};
use crate::point3::{Point3, point_from_array, rotate_y};
use crate::ray::Ray;
use crate::material::Material;

// A pair of floats, so it does not matter much if you copy them
#[derive(Clone, Copy)]
pub struct SurfaceCoordinate {
    pub u: f64,
    pub v: f64
}

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Point3,
    pub material: &'a dyn Material,  // &'a Box<dyn Material>
    pub t: f64,
    pub surface_coords: SurfaceCoordinate,
    pub front_face: bool,
}

pub fn create_hit_record<'a>(ray: &Ray, t: f64, outward_normal: Point3, material: &'a dyn Material, surface_coords: SurfaceCoordinate) -> HitRecord<'a> {
    // Creates a HitRecord with all it's parameters from the colliding ray, the 
    // parameter of the ray at the point of collision, the normal at that point, and the material of the surface
    let p: Point3 = ray.at(t);

    // let unit_outward_normal: Point3 = unit_vector(outward_normal);

    let front_face: bool = outward_normal.dot(ray.direction) < 0.0;
    let normal: Point3 = if front_face {outward_normal} else {-outward_normal};

    HitRecord {p, normal, material, t, surface_coords, front_face }
}

// For now, checking for a hit requires calculating it, so in the function that checks for hits return ether 
pub enum HitResult<'a> {
    DidNotHit,
    HitRecord(HitRecord<'a>)
}

// Instead of inheritance, create a trait that subsecuent objects will implement
// To do: Think about traits versus enums for objects. Eg: scene object hittable that contains sphere, quadrilateral, disk etc 
// Having every object be a variant of Hittable could allow to have a 
// more complex hittable list with vectors for each object. The problem would be having too many variants that need to be handeled
pub trait Hittable {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_>;
    fn bounding_box(&self) -> &AABB; // Needed since hittables will be behind pointers that will be dereferenced
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Point3,
    bounding_box: AABB
}

impl Hittable for Translate {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
        // Move the ray backwards by the offset
        let offset_ray: Ray = Ray { origin: ray.origin - self.offset, direction: ray.direction };
        // Check for intersection with the new ray
        match self.object.hit(&offset_ray, ray_t) {
            HitResult::DidNotHit => HitResult::DidNotHit,
            HitResult::HitRecord(hr) => {
                // Move the intersection point forwards by the offset
                HitResult::HitRecord(HitRecord { p: hr.p + self.offset, normal: hr.normal, material: hr.material, t: hr.t, surface_coords: hr.surface_coords, front_face: hr.front_face })
            }
        }
    }
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

pub fn create_translation(object: Rc<dyn Hittable>, offset: Point3) -> Translate {
    let bounding_box: AABB = (*object.bounding_box()).clone();
    Translate { object, offset, bounding_box: bounding_box + offset }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB
}

impl Hittable for RotateY {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
        // To do: compare for performance: using rotate_y or copy pasting the same block of code four times to rotate
        let origin: Point3 = rotate_y(&ray.origin, self.cos_theta, self.sin_theta);
        let direction: Point3 = rotate_y(&ray.direction, self.cos_theta, self.sin_theta);

        let rotated_ray: Ray = Ray { origin, direction };

        match self.object.hit(&rotated_ray, ray_t) {
            HitResult::DidNotHit => HitResult::DidNotHit,
            HitResult::HitRecord(object_space_hit_record) => {
                let world_space_hit_record: HitRecord = HitRecord { 
                    p: rotate_y(&object_space_hit_record.p, self.cos_theta, -self.sin_theta),
                    normal: rotate_y(&object_space_hit_record.normal, self.cos_theta, -self.sin_theta), 
                    material: object_space_hit_record.material, 
                    t: object_space_hit_record.t, 
                    surface_coords: object_space_hit_record.surface_coords, 
                    front_face: object_space_hit_record.front_face
                 };
                 HitResult::HitRecord(world_space_hit_record)
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

pub fn create_rotate_y(object: Rc<dyn Hittable>, angle_in_degrees: f64) -> RotateY {
    let radians: f64 = angle_in_degrees.to_radians();
    let sin_theta: f64 = radians.sin();
    let cos_theta: f64 = radians.cos();
    let bounding_box: AABB = object.bounding_box().clone();

    let mut minimum: [f64; 3] = [ INFINITY, INFINITY, INFINITY ];
    let mut maximum: [f64; 3] = [ -INFINITY, -INFINITY, -INFINITY ];

    for i_int in 0..2 {
        for j_int in 0..2 {
            for k_int in 0..2 {
                let i: f64 = i_int as f64;
                let j: f64 = j_int as f64;
                let k: f64 = k_int as f64;

                // To do: make the fields of bounding box public to not index them like this
                // To do: possible opportunity to use arrays here for better performance (the compiler will paralelize?)
                let x: f64 = i*bounding_box[0].end + (1.0 - i)*bounding_box[0].start;
                let y: f64 = j*bounding_box[1].end + (1.0 - j)*bounding_box[1].start;
                let z: f64 = k*bounding_box[2].end + (1.0 - k)*bounding_box[2].start;

                let x_new =  cos_theta*x + sin_theta*z;
                let z_new: f64 = -sin_theta*x + cos_theta*z;

                let tester: Point3 = Point3 { x:x_new, y:y, z:z_new };

                for c in 0..2 {
                    minimum[c] = minimum[c].min(tester[c as u8]);
                    maximum[c] = maximum[c].min(tester[c as u8]);
                }
            }
        }
    }

    RotateY { object, sin_theta, cos_theta, bounding_box:aabb_from_points(point_from_array(minimum), point_from_array(maximum)) }
}