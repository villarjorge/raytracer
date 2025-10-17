pub mod point3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

use crate::camera::{Camera, create_camera};
pub use crate::point3::{Point3, unit_vector};
pub use crate::ray::Ray;
use crate::material::{Dielectric, Lambertian, Metal};
pub use crate::sphere::Sphere;
pub use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

fn main() {
    // World

    let mut world: HittableList = HittableList{objects: Vec::new()};

    let material_ground: Lambertian = Lambertian{albedo: Point3{x: 0.8, y: 0.8, z: 0.0}};
    let material_center: Lambertian = Lambertian{albedo: Point3{x: 0.1, y: 0.2, z: 0.5}};
    let material_left: Dielectric = Dielectric { refraction_index: 1.5 };
    // let material_bubble: Dielectric = Dielectric { refraction_index: 1.0/1.5 };
    let material_right: Metal = Metal { albedo: Point3 { x: 0.8, y: 0.6, z: 0.2 }, fuzz: 1.0 };

    world.add(Sphere{center: Point3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0, material: Box::new(material_ground)});
    world.add(Sphere{center: Point3{x: 0.0, y: 0.0, z: -1.2}, radius: 0.5, material: Box::new(material_center)});
    world.add(Sphere{center: Point3{x: -1.0, y: 0.0, z: -1.0}, radius: 0.5, material: Box::new(material_left)});
    // world.add(Sphere{center: Point3{x: -1.0, y: 0.0, z: -1.0}, radius: 0.4, material: Box::new(material_bubble)});
    world.add(Sphere{center: Point3{x: 1.0, y: 0.0, z: -1.0}, radius: 0.5, material: Box::new(material_right)});

    let cam: Camera = create_camera(16.0f64/9.0f64, 400u32, 100u32, 50u32);
    cam.render(world);
}
