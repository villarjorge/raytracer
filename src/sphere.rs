use crate::point3::Point3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray, ray_tmin: f32, ray_tmax: f32, mut hit_record: HitRecord) -> bool {
        let oc: Point3 = self.center - ray.origin;
        let a: f32 = ray.direction.length_squared();
        let h: f32 = oc.dot(ray.direction);
        let c: f32 = oc.length_squared() - self.radius*self.radius;
        let discriminant: f32 = h*h - a*c;

        if discriminant < 0.0 {
            return false;
        } 
        let sqrt_discriminant: f32 = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root: f32 = (h-sqrt_discriminant)/a;
        // If the first root is less than the minimum or more than the max, check other root the same way 
        if root <= ray_tmin || ray_tmax <= root {
            root = (h+sqrt_discriminant)/a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal: Point3 = (hit_record.p - self.center)/self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        return true;
    }
}