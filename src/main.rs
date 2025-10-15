pub mod point3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;

use crate::camera::{Camera, create_camera};
pub use crate::point3::{Point3, unit_vector};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

fn main() {
    // World

    let mut world: HittableList = HittableList{objects: Vec::new()};
    world.add(Sphere{center: Point3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5});
    world.add(Sphere{center: Point3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0});

    let cam: Camera = create_camera(16.0f64/9.0f64, 400u32);
    cam.render(world);
}
