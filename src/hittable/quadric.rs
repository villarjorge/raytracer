// To do: implement general quadric
// http://skuld.bmsc.washington.edu/people/merritt/graphics/quadrics.html

use std::{
    ops::Range, 
};
use std::sync::Arc;

use crate::{
    aabb::{AABB}, 
    hittable::{HitRecord, Hittable, SurfaceCoordinate}, 
    material::Material, 
    point3::{Point3, Vector3, dot, unit_vector}, 
    ray::Ray
};

pub struct Quadric {
    p1: Point3,
    p2: Point3,
    p3: Point3,
    j: f64,
    material: Arc<dyn Material>,
    bounding_box: AABB
}

// Plan: instead of making some complex function to determine bounds of the quadric, let the bounding box define its extent

impl Hittable for Quadric {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        // Calculate coeficients of quadratic equation: at^2 + bt + c = 0
        let o: Point3 = ray.origin;
        let d: Point3 = ray.direction;

        // Use sympy_quadric.py to get these coeficients
        // To do: clean this up further like in sphere b = -2h => h = -1/2 b
        let a: f64 = self.p1.dot(d*d) + self.p2.dot(prod1(&d, &d));
        let b: f64 = 2.0*self.p1.dot(d*o) + self.p2.dot(anticross(&d, &o)) + self.p3.dot(d);
        let c: f64 = self.p1.dot(o*o) + self.p2.dot(anticross(&o, &o)) + self.p3.dot(o) + self.j;

        // Once you have those coeficients, you procede in basically the same way as in sphere
        let discriminant: f64 = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return false;
        } 

        let sqrt_discriminant: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (- b - sqrt_discriminant)/(2.0*a);

        if !ray_t.contains(&root) {
            root = (- b + sqrt_discriminant)/(2.0*a);
            if !ray_t.contains(&root) {
                return false;
            }
        }

        let p: Point3 = ray.at(root);

        // This is also in sympy_quadric.py
        let normal: Vector3 = Point3 { 
            x: 2.0*self.p1.x*p.x + self.p2.x*p.y + self.p2.y*p.z,
            y: 2.0*self.p1.y*p.y + self.p2.x*p.x + self.p2.z*p.z,
            z: 2.0*self.p1.z*p.z + self.p2.y*p.x + self.p2.z*p.y,
        } + self.p3;

        
        // To do: To deal with the material, dereference the pointer, then create a reference. Change this so you don't
        // let record: HitRecord = HitRecord::new(ray, root, outward_normal, &*self.material, surface_coords);
        hit_record.t = root;
        hit_record.p = p;

        // If the ray originates inside of the surface, reverse the normal
        let outward_normal: Point3 = if dot(&d, &p) > 0.0 { -unit_vector(normal) } else { unit_vector(normal) };
        hit_record.set_face_normal(ray, outward_normal);

        hit_record.material = self.material.clone();
        // To do: Since there is no general closed form coordinates, find some other way to get surface coordinates. Use differential geomety?
        // To do: ☠☠ once you have those coordinates, you can reverse based on them, like in Parallelogram ☠☠
        hit_record.surface_coords = SurfaceCoordinate {u: 0.0, v: 0.0};

        true
    }
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

fn prod1(u: &Point3, v: &Point3) -> Point3 {
    Point3 { x: u.x*v.y, y: u.x*v.z, z: u.y*v.z }
}

fn anticross(u: &Point3, v: &Point3) -> Point3 {
    Point3 { 
        x: u.x*v.y + u.y*v.x, 
        y: u.x*v.z + u.z*v.x, 
        z: u.y*v.z + u.z*v.y
    }
}

// p1 is basically and "indicator": cylinders in xyz correspond to (011), (101), (110); sphere is (111); Perhaps you can generalize?
/// An infinite cylinder made from a general quadric. The cylinder is perpendicular to the y axis. The bounding box of the cylinder is 
/// a cube of side equal to the diameter of the cylinder, and as such it can "shorten" it.
pub fn y_cylinder(center: Point3, radius: f64, material: Arc<dyn Material>) -> Quadric {
    let radius_vector: Point3 = Point3 { x: radius, y: radius, z: radius };
    let bounding_box: AABB = AABB::from_points(center - radius_vector, center + radius_vector);

    Quadric { 
        p1: Point3 { x: 1.0, y: 0.0, z: 1.0 }, 
        p2: Point3::default(),
        p3: center*Point3 { x: -2.0, y: 0.0, z: -2.0}, 
        j: - radius*radius + center.x*center.x + center.z*center.z, 
        material, 
        bounding_box
    }
}

/// Quadric sphere for testing purposes. The other sphere has proper surface coordinates
pub fn quadric_sphere(center: Point3, radius: f64, material: Arc<dyn Material>) -> Quadric {
    let radius_vector: Point3 = Point3 { x: radius, y: radius, z: radius };
    let bounding_box: AABB = AABB::from_points(center - radius_vector, center + radius_vector);

    Quadric { 
        p1: Point3 { x: 1.0, y: 1.0, z: 1.0 }, 
        p2: Point3::default(), 
        p3: -2.0*center, 
        j: - radius*radius + dot(&center, &center), 
        material, 
        bounding_box 
    }
}

/// A cone parallel to the y axis. Internaly represented as a general quadric
pub fn y_cone(center: Point3, offset: Point3, material: Arc<dyn Material>) -> Quadric {
    let bounding_box: AABB = AABB::from_points(center - offset, center + offset);

    let indicator: Point3 = Point3 { x: 1.0, y: -1.0, z: 1.0 };
    Quadric { 
        p1: indicator, 
        p2: Point3::default(),
        p3: -2.0*indicator*center,
        j: dot(&(center*indicator), &center),
        material, 
        bounding_box
    }
}