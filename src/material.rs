use crate::point3::{random_unit_vector, reflect, Point3};
use crate::ray::Ray;
use crate::hittable::HitRecord;

// If you are confused about the lifetimes, think about it this way: 
// multiple objects could use the same material, which means that the material pointer needs to outlive everything else
pub struct ScatteredRayAndAttenuation { // think of a better name?
    pub ray: Ray,
    pub attenuation: Point3
}

pub enum ScatterResult {
    DidNotScatter,
    DidScatter(ScatteredRayAndAttenuation)
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterResult;
}

// Perfect black body at 0K: absorbs all incoming rays and does not emit anything
pub struct BlackBody {} 

impl Material for BlackBody {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord) -> ScatterResult {
        return ScatterResult::DidNotScatter;
    }
}

pub struct Lambertian {
    pub albedo: Point3
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> ScatterResult {
        // Do it this way to avoid defining a mutable varible
        let scatter_direction: Point3 = {
            let temp: Point3 = record.normal + random_unit_vector();
            // Catch degenerate scatter direction
            if temp.is_near_zero() {
                record.normal
            } else {
                temp
            }
        };

        let scattered: Ray = Ray{origin: record.p, direction: scatter_direction};
        let sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation{ray: scattered, attenuation: self.albedo};

        return ScatterResult::DidScatter(sca_att);
    }
}

pub struct Metal {
    pub albedo: Point3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterResult {
        let reflected: Point3 = reflect(ray_in.direction, record.normal);
        let scattered: Ray = Ray{origin: record.p, direction: reflected};

        let sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation{ray: scattered, attenuation: self.albedo};

        return ScatterResult::DidScatter(sca_att);
    }
}