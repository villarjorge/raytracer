pub mod point3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod texture;

use crate::bvh::{BVHNode, create_bvh_node_from_hittable_list};
use crate::camera::{create_camera, Camera, CameraPosition, ThinLens};
use crate::point3::random_vector;
use crate::point3::{Point3, unit_vector};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::{create_sphere};
use crate::hittable_list::HittableList;
use crate::texture::{CheckerTexture, create_checker_texture_from_colors, create_solid_color};

fn many_spheres() {
    // World

    let mut world: HittableList = HittableList::default();

    //let checker: CheckerTexture = create_checker_texture_from_colors(3.1, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});
    let ground_material = Lambertian{texture: create_solid_color(Point3 { x: 0.5, y: 0.5, z: 0.5 })};
    world.add(create_sphere(Point3{x: 0.0, y: -1000.0, z: -1.0}, 1000.0, Box::new(ground_material)));

    const N: i32 = 11;

    for a in -N..N {
        for b in -N..N {
            let choose_mat: f64 = rand::random_range(0.0..1.0);
            let center: Point3 = Point3{x: a as f64 + 0.9*rand::random_range(0.0..1.0), y: 0.2, z: b as f64 + 0.9*rand::random_range(0.0..1.0)};

            if (center - Point3{x: 4.0, y: 0.2, z: 0.0}).length_squared() > 0.0 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Point3 = random_vector(0.0, 1.0)*random_vector(0.0, 1.0);
                    let sphere_material: Lambertian = Lambertian{texture: create_solid_color(albedo)};
                    world.add(create_sphere(center, 0.2, Box::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Point3 = random_vector(0.0, 1.0)*random_vector(0.0, 1.0);
                    let fuzz: f64 = rand::random_range(0.0..0.5);
                    let sphere_material: Metal = Metal{albedo, fuzz};
                    world.add(create_sphere(center, 0.2, Box::new(sphere_material)));
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric { refraction_index: 1.5 };
                    world.add(create_sphere(center, 0.2, Box::new(sphere_material)));
                }
            }
        }
    }

    let material1: Dielectric = Dielectric { refraction_index: 1.5 };
    world.add(create_sphere(Point3 { x: 0.0, y: 1.0, z: 0.0 }, 1.0, Box::new(material1)));

    let material2: Lambertian = Lambertian { texture: create_solid_color(Point3 { x: 0.4, y: 0.2, z: 0.1 }) };
    world.add(create_sphere(Point3 { x: -4.0, y: 1.0, z: 0.0 }, 1.0, Box::new(material2)));

    let material3: Metal = Metal { albedo: Point3 { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 };
    world.add(create_sphere(Point3 { x: 4.0, y: 1.0, z: 0.0 },  1.0,  Box::new(material3)));

    // let material3: BlackBody = BlackBody {  };
    // world.add(Sphere{center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: Box::new(material3)});

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 1200; // 1200
    let samples_per_pixel: u32 = 10; // 500
    let max_depth: u32 = 100;

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.6;
    let focus_distance: f64 = 10.0;

    let thin_lens: ThinLens = ThinLens{defocus_angle, focus_distance};

    let look_from: Point3 = Point3 { x: 13.0, y: 2.0, z: 3.0 };
    let look_at: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };
    let view_up: Point3 = Point3 { x: 0.0, y: 1.0, z: 0.0 };

    let camera_position: CameraPosition = CameraPosition{look_from, look_at, view_up};

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, thin_lens, camera_position);

    // To do: Make this a parameter that can be passed in the console
    // If you want to compare without the bvh
    // cam.render(&world);

    let bvh_world: BVHNode = create_bvh_node_from_hittable_list(world);
    cam.render(&bvh_world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    // To do: make it so this texture can be reused
    let checker1: CheckerTexture = create_checker_texture_from_colors(0.32, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});
    let checker2: CheckerTexture = create_checker_texture_from_colors(0.32, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});

    world.add(create_sphere(Point3{x: 0.0, y: -10.0, z: 0.0}, 10.0, Box::new(Lambertian{texture: Box::new(checker1)})));
    world.add(create_sphere(Point3{x: 0.0, y: 10.0, z: 0.0}, 10.0, Box::new(Lambertian{texture: Box::new(checker2)})));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 13.0, y: 2.0, z: 3.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position);

    cam.render(&world);
}

fn main() {

    let scene_number: u32 = 1;

    match scene_number {
        0 => many_spheres(),
        1 => checkered_spheres(),
        _ => panic!()
    }
}
