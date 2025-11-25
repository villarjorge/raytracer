use std::rc::Rc;

use crate::point3::{dot, random_unit_vector, reflect, refract, Point3};
use crate::ray::Ray;
use crate::hittable::{HitRecord, SurfaceCoordinate};
use crate::texture::{Texture, create_solid_color};
use crate::unit_vector;

// If you are confused about the lifetimes, think about it this way: 
// multiple objects could use the same material, which means that the material pointer needs to outlive everything else
pub struct ScatteredRayAndAttenuation { // think of a better name?
    pub scattered_ray: Ray,
    pub attenuation: Point3
}

pub enum ScatterResult {
    DidNotScatter,
    DidScatter(ScatteredRayAndAttenuation)
}

pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord) -> ScatterResult {
        ScatterResult::DidNotScatter
    }

    fn emitted(&self, _surface_coords: SurfaceCoordinate, _p: &Point3) -> Point3{
        Point3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

// Perfect black body at 0K: absorbs all incoming rays and does not emit anything
// To do: have a temperature parameter, which then gets transformed into color
// Resources: 
// - http://www.vendian.org/mncharity/dir3/blackbody/
// - https://web.archive.org/web/20010821031240/http://astronomy.swin.edu.au:80/pbourke/colour/conversion.html
pub struct BlackBody {} 

impl Material for BlackBody {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord) -> ScatterResult {
        ScatterResult::DidNotScatter
    }
}

pub struct Lambertian {
    pub texture: Rc<dyn Texture>
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

        let attenuation: Point3 = self.texture.value(record.surface_coords, &record.p);
        let scattered_ray: Ray = Ray{origin: record.p, direction: scatter_direction};
        let sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation{scattered_ray, attenuation};

        ScatterResult::DidScatter(sca_att)
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
        let scattered_ray: Ray = Ray{origin: record.p, direction: reflected_with_fuzz};

        let sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation{scattered_ray, attenuation: self.albedo};

        ScatterResult::DidScatter(sca_att)
    }
}

pub struct Dielectric {
    pub refraction_index: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterResult {
        let ratio_indexes: f64 = if record.front_face {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction: Point3 = unit_vector(ray_in.direction);
        let cos_theta: f64 = dot(&record.normal, &(-unit_direction)).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ratio_indexes*sin_theta > 1.0;
        let reflectance_bigger_than_random: bool = reflectance(cos_theta, ratio_indexes) > rand::random_range(0.0..1.0);

        let direction: Point3 = {
            if cannot_refract | reflectance_bigger_than_random {
                reflect(unit_direction, record.normal)
            }
            else {
                // To do: Make the ratio of indexes dependant on the color of the incoming ray to model chromatic aberration
                refract(unit_direction, record.normal, ratio_indexes)
            }
        };
        
        let attenuation: Point3 = Point3 { x: 1.0, y: 1.0, z: 1.0 };
        let scattered_ray: Ray = Ray { origin: record.p, direction };

        let sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation{scattered_ray, attenuation};

        ScatterResult::DidScatter(sca_att)
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0: f64 = (1.0 - refraction_index)/(1.0 + refraction_index);
        let r0_squared: f64 = r0*r0;

        r0_squared + (1.0 - r0_squared)*((1.0 - cosine).powf(5.0))
}

pub struct DiffuseLight {
    texture: Rc<dyn Texture>
}

pub fn diffuse_light_from_color(color: Point3) -> Rc<DiffuseLight> {
    Rc::new(DiffuseLight { texture: create_solid_color(color) })
}

impl Material for DiffuseLight {
    fn emitted(&self, surface_coords: SurfaceCoordinate, p: &Point3) -> Point3 {
        self.texture.value(surface_coords, p)
    }
}