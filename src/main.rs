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
pub mod perlin;
pub mod parallelogram;

use std::rc::Rc;

use crate::bvh::{BVHNode, create_bvh_node_from_hittable_list};
use crate::camera::{create_camera, Camera, CameraPosition, ThinLens};
use crate::parallelogram::create_parallelogram;
use crate::perlin::create_perlin_noise;
use crate::point3::random_vector;
use crate::point3::{Point3, unit_vector};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, create_diffuse_light_from_color};
use crate::sphere::{create_sphere};
use crate::hittable_list::HittableList;
use crate::texture::{CheckerTexture, PerlinNoiseTexture, create_checker_texture_from_colors, create_image_texture, create_solid_color};

fn many_spheres() {
    // World

    let mut world: HittableList = HittableList::default();

    let checker: Rc<CheckerTexture> = create_checker_texture_from_colors(3.1, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});
    // let ground_material = Lambertian{texture: create_solid_color(Point3 { x: 0.5, y: 0.5, z: 0.5 })};
    let ground_material: Lambertian = Lambertian{texture: checker};
    world.add(create_sphere(Point3{x: 0.0, y: -1000.0, z: -1.0}, 1000.0, Rc::new(ground_material)));

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
                    world.add(create_sphere(center, 0.2, Rc::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Point3 = random_vector(0.0, 1.0)*random_vector(0.0, 1.0);
                    let fuzz: f64 = rand::random_range(0.0..0.5);
                    let sphere_material: Metal = Metal{albedo, fuzz};
                    world.add(create_sphere(center, 0.2, Rc::new(sphere_material)));
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric { refraction_index: 1.5 };
                    world.add(create_sphere(center, 0.2, Rc::new(sphere_material)));
                }
            }
        }
    }

    let material1: Dielectric = Dielectric { refraction_index: 1.5 };
    world.add(create_sphere(Point3 { x: 0.0, y: 1.0, z: 0.0 }, 1.0, Rc::new(material1)));

    let material2: Lambertian = Lambertian { texture: create_solid_color(Point3 { x: 0.4, y: 0.2, z: 0.1 }) };
    world.add(create_sphere(Point3 { x: -4.0, y: 1.0, z: 0.0 }, 1.0, Rc::new(material2)));

    let material3: Metal = Metal { albedo: Point3 { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 };
    world.add(create_sphere(Point3 { x: 4.0, y: 1.0, z: 0.0 },  1.0,  Rc::new(material3)));

    // let material3: BlackBody = BlackBody {  };
    // world.add(Sphere{center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: Rc::new(material3)});

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

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, thin_lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    // To do: Make this a parameter that can be passed in the console
    // If you want to compare without the bvh
    // cam.render(&world);

    let bvh_world: BVHNode = create_bvh_node_from_hittable_list(world);
    cam.render(&bvh_world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker: Rc<CheckerTexture> = create_checker_texture_from_colors(0.10, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});
    let material: Rc<Lambertian> = Rc::new(Lambertian{texture: checker});

    world.add(create_sphere(Point3{x: 0.0, y: -10.0, z: 0.0}, 10.0, material.clone()));
    world.add(create_sphere(Point3{x: 0.0, y: 10.0, z: 0.0}, 10.0, material));

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

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn earth() {
    let mut world: HittableList = HittableList::default();

    let earth_texture: Rc<texture::ImageTexture> = create_image_texture("textures/earthmap.jpg");
    let earth_material: Rc<Lambertian> = Rc::new(Lambertian{texture: earth_texture});

    world.add(create_sphere(Point3{x: 0.0, y: 0.0, z: 0.0}, 2.0, earth_material));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 0.0, y: 0.0, z: 12.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::default();

    let perlin_texture: Rc<PerlinNoiseTexture>  = Rc::new(PerlinNoiseTexture { perlin_noise: create_perlin_noise(), scale: 2.0});
    let perlin_material: Rc<Lambertian> = Rc::new(Lambertian{ texture: perlin_texture });

    world.add(create_sphere(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material.clone()));
    world.add(create_sphere(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material));

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

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn para() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let left_red: Rc<Lambertian> = Rc::new(Lambertian{texture: create_solid_color(Point3 { x: 1.0, y: 0.2, z: 0.2 })});
    let back_green: Rc<Lambertian> = Rc::new(Lambertian{texture: create_solid_color(Point3 { x: 0.2, y: 1.0, z: 0.2 })});
    let right_blue: Rc<Lambertian> = Rc::new(Lambertian{texture: create_solid_color(Point3 { x: 0.2, y: 0.2, z: 1.0 })});
    let upper_orange: Rc<Lambertian> = Rc::new(Lambertian{texture: create_solid_color(Point3 { x: 1.0, y: 0.5, z: 0.0 })});
    let lower_teal: Rc<Lambertian> = Rc::new(Lambertian{texture: create_solid_color(Point3 { x: 0.2, y: 0.8, z: 0.8})});

    world.add(create_parallelogram(Point3{x: -3.0, y: -2.0, z:5.0}, Point3{x: 0.0, y: 0.0, z:-4.0}, Point3{x: 0.0, y:4.0, z:0.0}, left_red));
    world.add(create_parallelogram(Point3{x: -2.0, y: -2.0, z:0.0}, Point3{x: 4.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:4.0, z:0.0}, back_green));
    world.add(create_parallelogram(Point3{x:  3.0, y: -2.0, z:1.0}, Point3{x: 0.0, y: 0.0, z: 4.0}, Point3{x: 0.0, y:4.0, z:0.0}, right_blue));
    world.add(create_parallelogram(Point3{x: -2.0, y:  3.0, z:1.0}, Point3{x: 4.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:4.0}, upper_orange));
    world.add(create_parallelogram(Point3{x: -2.0, y: -3.0, z:5.0}, Point3{x: 4.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:-4.0}, lower_teal));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let vfov: f64 = 80.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 0.0, y: 0.0, z: 9.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn simple_light() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let perlin_texture: Rc<PerlinNoiseTexture>  = Rc::new(PerlinNoiseTexture { perlin_noise: create_perlin_noise(), scale: 2.0});
    let perlin_material: Rc<Lambertian> = Rc::new(Lambertian{ texture: perlin_texture });

    world.add(create_sphere(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material.clone()));
    world.add(create_sphere(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material));

    let diffuse_light: Rc<DiffuseLight> = create_diffuse_light_from_color(Point3 { x: 4.0, y: 4.0, z: 4.0 });
    world.add(create_parallelogram(Point3{x: 3.0, y: 1.0, z:-2.0}, Point3{x: 2.0, y: 0.0, z:0.0}, Point3{x: 0.0, y:2.0, z:0.0}, diffuse_light));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 26.0, y: 3.0, z: 6.0};
    let look_at: Point3 = Point3{x: 0.0, y: 2.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn cornell_box() {
    let mut world: HittableList = HittableList::default();

    let red: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.65, y: 0.05, z: 0.05 }) });
    let white: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.73, y: 0.73, z: 0.73 }) });
    let green: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.12, y: 0.45, z: 0.15 }) });
    let diffuse_light: Rc<DiffuseLight> = create_diffuse_light_from_color(Point3 { x: 15.0, y: 15.0, z: 15.0 });

    world.add(create_parallelogram(Point3{x: 555.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z:0.0}, Point3{x: 0.0, y:0.0, z:555.0}, green));
    world.add(create_parallelogram(Point3{x: 0.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:555.0}, red));
    world.add(create_parallelogram(Point3{x:  343.0, y: 554.0, z: 332.0}, Point3{x: -130.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:-105.0}, diffuse_light));
    world.add(create_parallelogram(Point3{x: 555.0, y: 555.0, z: 555.0}, Point3{x: -555.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:-555.0}, white.clone()));
    world.add(create_parallelogram(Point3{x: 0.0, y: 0.0, z:555.0}, Point3{x: 555.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:555.0, z:0.0}, white));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 40.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 278.0, y: 278.0, z: -800.0};
    let look_at: Point3 = Point3{x: 278.0, y: 278.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn main() {
    let scene_number: u32 = 6;

    match scene_number {
        0 => many_spheres(),
        1 => checkered_spheres(),
        2 => earth(),
        3 => perlin_spheres(),
        4 => para(),
        5 => simple_light(),
        6 => cornell_box(),
        _ => panic!()
    }
}
