// To do: implement general quadric
// http://skuld.bmsc.washington.edu/people/merritt/graphics/quadrics.html

use std::{
    ops::Range, 
    rc::Rc
};

use crate::{
    aabb::{AABB, aabb_from_points}, 
    hittable::{HitRecord, HitResult, Hittable, SurfaceCoordinate, create_hit_record}, 
    material::Material, 
    point3::{Point3, Vector3}, 
    ray::Ray
};

pub struct Quadric {
    p1: Point3,
    p2: Point3,
    p3: Point3,
    j: f64,
    material: Rc<dyn Material>,
    bounding_box: AABB
}

// Plan: instead of making some complex function to determine bounds of the quadric, let the bounding box define its extent

impl Hittable for Quadric {
    fn hit(&'_ self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
        // Calculate coeficients of quadratic equation: at^2 + bt + c = 0
        let o: Point3 = ray.origin;
        let d: Point3 = ray.direction;

        // Use sympy_quadric.py to get these coeficients
        // To do: clean this up further like in sphere b = -2h => h = -1/2 b
        let a: f64 = self.p1.dot(d*d) + self.p2.dot(prod1(&d, &d));
        let b: f64 = 2.0*self.p1.dot(d*o) + self.p2.dot(anticross(&d, &o)) + self.p3.dot(d);
        let c: f64 = self.p1.dot(o*o) + self.p2.dot(anticross(&o, &o)) + self.p3.dot(o) + self.j;

        let discriminant: f64 = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return HitResult::DidNotHit;
        } 
        let sqrt_discriminant: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (- b - sqrt_discriminant)/(2.0*a);

        if !ray_t.contains(&root) {
            root = (- b + sqrt_discriminant)/(2.0*a);
            if !ray_t.contains(&root) {
                return HitResult::DidNotHit;
            }
        }

        let p: Point3 = ray.at(root);

        // This is also in sympy_quadric.py
        let outward_normal: Vector3 = Point3 { 
            x: 2.0*self.p1.x*p.x + self.p2.x*p.y + self.p2.y*p.z,
            y: 2.0*self.p1.y*p.y + self.p2.x*p.x + self.p2.z*p.z,
            z: 2.0*self.p1.z*p.z + self.p2.y*p.x + self.p2.z*p.y,
        } + self.p3;

        // To do: ☠☠ find equations for coordinates in a general quadric and implement them ☠☠
        let surface_coords: SurfaceCoordinate = SurfaceCoordinate {u: 0.0, v: 0.0};

        // To do: To deal with the material, dereference the pointer, then create a reference. Change this so you don't
        let record: HitRecord = create_hit_record(ray, root, outward_normal, &*self.material, surface_coords);

        HitResult::HitRecord(record)
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

pub fn y_cylinder(radius: f64, material: Rc<dyn Material>) -> Quadric {
    let bounding_box: AABB = aabb_from_points(Point3 { x: 0.0, y: 0.0, z: 0.0 }, Point3 { x: radius, y: radius, z: radius });

    Quadric { 
        p1: Point3 { x: 1.0, y: 0.0, z: 1.0 }, 
        p2: Point3::default(), 
        p3: Point3::default(), 
        j: -radius, 
        material, 
        bounding_box
    }
}