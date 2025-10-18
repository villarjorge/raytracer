use rand;

pub mod point3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

use crate::camera::{Camera, create_camera};
use crate::point3::random_vector;
pub use crate::point3::{Point3, unit_vector};
pub use crate::ray::Ray;
use crate::material::{Dielectric, Lambertian, Metal};
pub use crate::sphere::Sphere;
pub use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

fn main() {
    // World

    let mut world: HittableList = HittableList{objects: Vec::new()};

    let material_ground: Lambertian = Lambertian{albedo: Point3{x: 0.5, y: 0.5, z: 0.5}};
    world.add(Sphere{center: Point3{x: 0.0, y: -1000.0, z: -1.0}, radius: 1000.0, material: Box::new(material_ground)});

    const N: i32 = 11;

    for a in -N..N {
        for b in -N..N {
            let choose_mat: f64 = rand::random_range(0.0..1.0);
            let center: Point3 = Point3{x: a as f64 + 0.9*rand::random_range(0.0..1.0), y: 0.2, z: b as f64 + 0.9*rand::random_range(0.0..1.0)};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length_squared() > 0.0 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Point3 = random_vector(0.0, 1.0)*random_vector(0.0, 1.0);
                    let sphere_material: Lambertian = Lambertian{albedo: albedo};
                    world.add(Sphere{center: center, radius: 0.2, material: Box::new(sphere_material)});
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Point3 = random_vector(0.0, 1.0)*random_vector(0.0, 1.0);
                    let fuzz: f64 = rand::random_range(0.0..0.5);
                    let sphere_material: Metal = Metal{albedo: albedo, fuzz: fuzz};
                    world.add(Sphere{center: center, radius: 0.2, material: Box::new(sphere_material)});
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric { refraction_index: 1.5 };
                    world.add(Sphere{center: center, radius: 0.2, material: Box::new(sphere_material)});
                }
            }
        }
    }

    let material1: Dielectric = Dielectric { refraction_index: 1.5 };
    world.add(Sphere{center: Point3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 1.0, material: Box::new(material1)});

    let material2: Lambertian = Lambertian { albedo: Point3 { x: 0.4, y: 0.2, z: 0.1 } };
    world.add(Sphere{center: Point3 { x: -4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: Box::new(material2)});

    let material3: Metal = Metal { albedo: Point3 { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 };
    world.add(Sphere{center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: Box::new(material3)});

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 1200;
    let samples_per_pixel: u32 = 10;
    let max_depth: u32 = 50;

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.6;
    let focus_distance: f64 = 10.0;

    let look_from: Point3 = Point3 { x: 13.0, y: 2.0, z: 3.0 };
    let look_at: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };
    let view_up: Point3 = Point3 { x: 0.0, y: 1.0, z: 0.0 };

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, defocus_angle, focus_distance, look_from, look_at, view_up);
    cam.render(world);
}
