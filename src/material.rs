use crate::point3::{dot, random_unit_vector, reflect, refract, Point3};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::unit_vector;

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
    pub albedo: Point3,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterResult {
        let reflected: Point3 = reflect(ray_in.direction, record.normal);
        let reflected_with_fuzz: Point3 = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered: Ray = Ray{origin: record.p, direction: reflected_with_fuzz};

        let sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation{ray: scattered, attenuation: self.albedo};

        return ScatterResult::DidScatter(sca_att);
    }
}

pub struct Dielectric {
    pub refraction_index: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterResult {
        let attenuation: Point3 = Point3 { x: 1.0, y: 1.0, z: 1.0 };

        let ratio_indexes: f64 = if record.front_face {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction: Point3 = unit_vector(ray_in.direction);
        let cos_theta: f64 = dot(&record.normal, &(-unit_direction)).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ratio_indexes*sin_theta > 1.0;
        // let reflectance_bigger_than_random: bool = reflectance(cos_theta, ratio_indexes) > rand::random_range(0.0..1.0);

        // let direction: Point3 = {
        //     if cannot_refract | reflectance_bigger_than_random {
        //         reflect(unit_direction, record.normal)
        //     }
        //     else {
        //         refract(unit_direction, record.normal, ratio_indexes)
        //     }
        // };
        let direction: Point3 = {
            if cannot_refract {
                reflect(unit_direction, record.normal)
            }
            else {
                refract(unit_direction, record.normal, ratio_indexes)
            }
        };

        let scattered: Ray = Ray { origin: record.p, direction: direction };

        return ScatterResult::DidScatter(ScatteredRayAndAttenuation { ray: scattered, attenuation: attenuation });
    }
}

// fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
//         let r0: f64 = (1.0 - refraction_index)/(1.0 + refraction_index);
//         let r0_squared: f64 = r0*r0;

//         return r0_squared + (1.0 - r0_squared)*((1.0 - cosine).powf(5.0));
// }