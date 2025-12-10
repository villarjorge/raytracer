use std::sync::Arc;

use crate::point3::color::Color;
use crate::point3::{Point3, Vector3, dot, random_unit_vector, reflect, refract, unit_vector};
use crate::ray::Ray;
use crate::hittable::{HitRecord, SurfaceCoordinate};
use crate::texture::{Texture, SolidColor};

// If you are confused about the lifetimes, think about it this way: 
// multiple objects could use the same material, which means that the material pointer needs to outlive everything else
pub struct ScatteredRayAndAttenuation { // think of a better name?
    pub scattered_ray: Ray,
    pub attenuation: Color
}

/// The default for scatter is ScatterResult::DidNotScatter and for emitted Point3 { x: 0.0, y: 0.0, z: 0.0 } (black)
pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord, _sca_att: &mut ScatteredRayAndAttenuation) -> bool {
        false
    }

    fn emitted(&self, _surface_coords: SurfaceCoordinate, _p: &Point3) -> Color {
        Color { x: 0.0, y: 0.0, z: 0.0 }
    }
}


// To do: have a temperature parameter, which then gets transformed into color
// Resources: 
// - http://www.vendian.org/mncharity/dir3/blackbody/
// - https://web.archive.org/web/20010821031240/http://astronomy.swin.edu.au:80/pbourke/colour/conversion.html
/// Perfect black body at 0K: absorbs all incoming rays and does not emit anything
pub struct BlackBody {} 

/// A Lambertian or ideal diffuse material.  
pub struct Lambertian {
    pub texture: Arc<dyn Texture>
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord, sca_att: &mut ScatteredRayAndAttenuation) -> bool {
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

        sca_att.scattered_ray = Ray::new(record.p, scatter_direction);
        sca_att.attenuation = self.texture.value(record.surface_coords, &record.p);

        true
    }
}

impl Lambertian {
    pub fn from_color(color: Color) -> Arc<Lambertian> {
        Arc::new(Lambertian{ texture: SolidColor::new(color) })
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Arc<Lambertian> {
        Arc::new(Lambertian { texture })
    }
}

// pub fn lambertian(texture: Arc<dyn Texture>) -> Arc<Lambertian> {
//     Arc::new(Lambertian{ texture })
// }

/// A metal material: it reflects according to Snell's law, with some randomness added, controled by the fuzz parameter
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, sca_att: &mut ScatteredRayAndAttenuation) -> bool {
        let reflected: Vector3 = reflect(ray_in.direction, record.normal);
        let reflected_with_fuzz: Vector3 = unit_vector(reflected) + (self.fuzz * random_unit_vector());

        sca_att.scattered_ray = Ray::new(record.p, reflected_with_fuzz);
        sca_att.attenuation = self.albedo;

        true
    }
}

pub fn metal(albedo: Color, fuzz: f64) -> Arc<Metal> {
    Arc::new(Metal { albedo, fuzz })
}

/// A dielectric material: it reflets or refracts depending on the angle:
///     - Reflects: Angle is shallow enough or reflectance is bigger that some random value
///     - Refracts: otherwise
/// Reflectance is calculated with Schlick's aproximation
pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    pub refraction_index: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, sca_att: &mut ScatteredRayAndAttenuation) -> bool {
        let ratio_indexes: f64 = if record.front_face {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction: Vector3 = unit_vector(ray_in.direction);
        let cos_theta: f64 = dot(&record.normal, &(-unit_direction)).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ratio_indexes*sin_theta > 1.0;
        let reflectance_bigger_than_random: bool = reflectance(cos_theta, ratio_indexes) > rand::random_range(0.0..1.0);

        let direction: Vector3 = {
            if cannot_refract | reflectance_bigger_than_random {
                reflect(unit_direction, record.normal)
            }
            else {
                // To do: Make the ratio of indexes dependant on the color of the incoming ray to model chromatic aberration
                refract(unit_direction, record.normal, ratio_indexes)
            }
        };

        sca_att.scattered_ray = Ray::new(record.p, direction);
        sca_att.attenuation = Color { x: 1.0, y: 1.0, z: 1.0 };

        true
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0: f64 = (1.0 - refraction_index)/(1.0 + refraction_index);
        let r0_squared: f64 = r0*r0;

        r0_squared + (1.0 - r0_squared)*((1.0 - cosine).powf(5.0))
}

pub fn dielectric(refraction_index: f64) -> Arc<Dielectric> {
    Arc::new(Dielectric { refraction_index })
}

// A diffuse light: always emmits some texture, does not scatter
pub struct DiffuseLight {
    texture: Arc<dyn Texture>
}

impl DiffuseLight {    
    pub fn from_color(color: Point3) -> Arc<DiffuseLight> {
        Arc::new(DiffuseLight { texture: SolidColor::new(color) })
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, surface_coords: SurfaceCoordinate, p: &Point3) -> Color {
        self.texture.value(surface_coords, p)
    }
}

/// An isotropic material: Scatters light in a uniform random direction
pub struct Isotropic {
    pub texture: Arc<dyn Texture>
}

impl Material for Isotropic {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord, sca_att: &mut ScatteredRayAndAttenuation) -> bool {
        // Scatter in a uniform random direction
        sca_att.scattered_ray = Ray::new(record.p, random_unit_vector());
        sca_att.attenuation = self.texture.value(record.surface_coords, &record.p);

        true
    }
}