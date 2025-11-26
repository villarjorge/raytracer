pub mod point3;
pub mod hittable;
pub mod ray;
pub mod camera;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod perlin;
pub mod constant_medium;
pub mod tests;

use std::rc::Rc;

use crate::bvh::{BVHNode, bvh_node_from_hittable_list};
use crate::camera::{Camera, CameraPosition, ImageQuality, ThinLens, create_camera};
use crate::constant_medium::{constant_medium_from_color};
use crate::hittable::quadric::{y_cylinder};
use crate::hittable::{RotateY, Translate, create_rotate_y, create_translation};
use crate::hittable::parallelogram::{create_box, parallelogram};
use crate::perlin::create_perlin_noise;
use crate::point3::{point_from_array, random_vector};
use crate::point3::{Point3, unit_vector};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, dielectric, diffuse_light_from_color, lambertian, metal};
use crate::hittable::sphere::{Sphere, sphere};
use crate::hittable::hittable_list::HittableList;
use crate::texture::{CheckerTexture, PerlinNoiseTexture, Texture, checker_texture_from_colors, create_image_texture, create_solid_color};

fn many_spheres() {
    // World

    let mut world: HittableList = HittableList::default();

    let checker: Rc<CheckerTexture> = checker_texture_from_colors(3.1, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});
    // let ground_material = Lambertian{texture: create_solid_color(Point3 { x: 0.5, y: 0.5, z: 0.5 })};
    let ground_material: Lambertian = Lambertian{texture: checker};
    world.add(sphere(Point3{x: 0.0, y: -1000.0, z: -1.0}, 1000.0, Rc::new(ground_material)));

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
                    world.add(sphere(center, 0.2, Rc::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Point3 = random_vector(0.0, 1.0)*random_vector(0.0, 1.0);
                    let fuzz: f64 = rand::random_range(0.0..0.5);
                    let sphere_material: Metal = Metal{albedo, fuzz};
                    world.add(sphere(center, 0.2, Rc::new(sphere_material)));
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric { refraction_index: 1.5 };
                    world.add(sphere(center, 0.2, Rc::new(sphere_material)));
                }
            }
        }
    }

    let material1: Dielectric = Dielectric { refraction_index: 1.5 };
    world.add(sphere(Point3 { x: 0.0, y: 1.0, z: 0.0 }, 1.0, Rc::new(material1)));

    let material2: Lambertian = Lambertian { texture: create_solid_color(Point3 { x: 0.4, y: 0.2, z: 0.1 }) };
    world.add(sphere(Point3 { x: -4.0, y: 1.0, z: 0.0 }, 1.0, Rc::new(material2)));

    let material3: Metal = Metal { albedo: Point3 { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 };
    world.add(sphere(Point3 { x: 4.0, y: 1.0, z: 0.0 },  1.0,  Rc::new(material3)));

    // let material3: BlackBody = BlackBody {  };
    // world.add(Sphere{center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: Rc::new(material3)});

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 1200; // 1200
    let samples_per_pixel: u32 = 10; // 500
    let max_depth: u32 = 100;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.6;
    let focus_distance: f64 = 10.0;

    let thin_lens: ThinLens = ThinLens{defocus_angle, focus_distance};

    let look_from: Point3 = Point3 { x: 13.0, y: 2.0, z: 3.0 };
    let look_at: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };
    let view_up: Point3 = Point3 { x: 0.0, y: 1.0, z: 0.0 };

    let camera_position: CameraPosition = CameraPosition{look_from, look_at, view_up};

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, thin_lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    // To do: Make this a parameter that can be passed in the console
    // If you want to compare without the bvh
    // cam.render(&world);

    let bvh_world: BVHNode = bvh_node_from_hittable_list(world);
    cam.render(&bvh_world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker: Rc<CheckerTexture> = checker_texture_from_colors(0.10, Point3 {x: 0.2, y: 0.3, z: 0.1}, Point3 {x: 0.9, y: 0.9, z: 0.9});
    let material: Rc<Lambertian> = Rc::new(Lambertian{texture: checker});

    world.add(sphere(Point3{x: 0.0, y: -10.0, z: 0.0}, 10.0, material.clone()));
    world.add(sphere(Point3{x: 0.0, y: 10.0, z: 0.0}, 10.0, material));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 13.0, y: 2.0, z: 3.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn earth() {
    let mut world: HittableList = HittableList::default();

    let earth_texture: Rc<dyn Texture> = create_image_texture("textures/earthmap.jpg");
    let earth_material: Rc<Lambertian> = Rc::new(Lambertian{texture: earth_texture});

    world.add(sphere(Point3{x: 0.0, y: 0.0, z: 0.0}, 2.0, earth_material));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 0.0, y: 0.0, z: 12.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::default();

    let perlin_texture: Rc<PerlinNoiseTexture>  = Rc::new(PerlinNoiseTexture { perlin_noise: create_perlin_noise(), scale: 2.0});
    let perlin_material: Rc<Lambertian> = Rc::new(Lambertian{ texture: perlin_texture });

    world.add(sphere(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material.clone()));
    world.add(sphere(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 13.0, y: 2.0, z: 3.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

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

    world.add(parallelogram(Point3{x: -3.0, y: -2.0, z:5.0}, Point3{x: 0.0, y: 0.0, z:-4.0}, Point3{x: 0.0, y:4.0, z:0.0}, left_red));
    world.add(parallelogram(Point3{x: -2.0, y: -2.0, z:0.0}, Point3{x: 4.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:4.0, z:0.0}, back_green));
    world.add(parallelogram(Point3{x:  3.0, y: -2.0, z:1.0}, Point3{x: 0.0, y: 0.0, z: 4.0}, Point3{x: 0.0, y:4.0, z:0.0}, right_blue));
    world.add(parallelogram(Point3{x: -2.0, y:  3.0, z:1.0}, Point3{x: 4.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:4.0}, upper_orange));
    world.add(parallelogram(Point3{x: -2.0, y: -3.0, z:5.0}, Point3{x: 4.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:-4.0}, lower_teal));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let vfov: f64 = 80.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 0.0, y: 0.0, z: 9.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, Point3 { x: 0.7, y: 0.8, z: 1.0 });

    cam.render(&world);
}

fn simple_light() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let perlin_texture: Rc<PerlinNoiseTexture>  = Rc::new(PerlinNoiseTexture { perlin_noise: create_perlin_noise(), scale: 2.0});
    let perlin_material: Rc<Lambertian> = Rc::new(Lambertian{ texture: perlin_texture });

    world.add(sphere(Point3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, perlin_material.clone()));
    world.add(sphere(Point3{x: 0.0, y: 2.0, z: 0.0}, 2.0, perlin_material));

    let diffuse_light: Rc<DiffuseLight> = diffuse_light_from_color(Point3 { x: 4.0, y: 4.0, z: 4.0 });
    world.add(parallelogram(Point3{x: 3.0, y: 1.0, z:-2.0}, Point3{x: 2.0, y: 0.0, z:0.0}, Point3{x: 0.0, y:2.0, z:0.0}, diffuse_light));

    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 20.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 26.0, y: 3.0, z: 6.0};
    let look_at: Point3 = Point3{x: 0.0, y: 2.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn cornell_box() {
    let mut world: HittableList = HittableList::default();

    let red: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.65, y: 0.05, z: 0.05 }) });
    let white: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.73, y: 0.73, z: 0.73 }) });
    let green: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.12, y: 0.45, z: 0.15 }) });
    let diffuse_light: Rc<DiffuseLight> = diffuse_light_from_color(Point3 { x: 15.0, y: 15.0, z: 15.0 });

    world.add(parallelogram(Point3{x: 555.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z:0.0}, Point3{x: 0.0, y:0.0, z:555.0}, green));
    world.add(parallelogram(Point3{x: 0.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:555.0}, red));
    world.add(parallelogram(Point3{x:  343.0, y: 554.0, z: 332.0}, Point3{x: -130.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:-105.0}, diffuse_light));
    world.add(parallelogram(point_from_array([0.0, 555.0, 0.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 0.0, 555.0]), white.clone()));
    world.add(parallelogram(point_from_array([0.0, 0.0, 0.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 0.0, 555.0]), white.clone()));
    world.add(parallelogram(point_from_array([0.0, 0.0, 555.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 555.0, 0.0]), white.clone()));

    let box1: Rc<HittableList> = Rc::new(create_box(point_from_array([0.0, 0.0, 0.0]), point_from_array([165.0, 330.0, 165.0]), white.clone()));
    let box1_rotated: Rc<RotateY>  = Rc::new(create_rotate_y(box1, 15.0));
    let box1_trans: Translate = create_translation(box1_rotated, point_from_array([265.0, 0.0, 295.0]));

    world.add(box1_trans);

    let box2: Rc<HittableList> = Rc::new(create_box(point_from_array([0.0, 0.0, 0.0]), point_from_array([165.0, 165.0, 165.0]), white));
    let box2_rotated: Rc<RotateY>  = Rc::new(create_rotate_y(box2, -18.0));
    let box2_trans: Translate = create_translation(box2_rotated, point_from_array([130.0, 0.0, 65.0]));

    world.add(box2_trans);

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 40.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 278.0, y: 278.0, z: -800.0};
    let look_at: Point3 = Point3{x: 278.0, y: 278.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world: HittableList = HittableList::default();

    let red: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.65, y: 0.05, z: 0.05 }) });
    let white: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.73, y: 0.73, z: 0.73 }) });
    let green: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.12, y: 0.45, z: 0.15 }) });
    let diffuse_light: Rc<DiffuseLight> = diffuse_light_from_color(Point3 { x: 7.0, y: 7.0, z: 7.0 });

    world.add(parallelogram(Point3{x: 555.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z:0.0}, Point3{x: 0.0, y:0.0, z:555.0}, green));
    world.add(parallelogram(Point3{x: 0.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:555.0}, red));
    world.add(parallelogram(Point3{x:  113.0, y: 554.0, z: 127.0}, Point3{x: 330.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:305.0}, diffuse_light));
    world.add(parallelogram(point_from_array([0.0, 555.0, 0.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 0.0, 555.0]), white.clone()));
    world.add(parallelogram(point_from_array([0.0, 0.0, 0.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 0.0, 555.0]), white.clone()));
    world.add(parallelogram(point_from_array([0.0, 0.0, 555.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 555.0, 0.0]), white.clone()));

    let box1: Rc<HittableList> = Rc::new(create_box(point_from_array([0.0, 0.0, 0.0]), point_from_array([165.0, 330.0, 165.0]), white.clone()));
    let box1_rotated: Rc<RotateY>  = Rc::new(create_rotate_y(box1, 15.0));
    let box1_trans: Translate = create_translation(box1_rotated, point_from_array([265.0, 0.0, 295.0]));

    let box2: Rc<HittableList> = Rc::new(create_box(point_from_array([0.0, 0.0, 0.0]), point_from_array([165.0, 165.0, 165.0]), white));
    let box2_rotated: Rc<RotateY>  = Rc::new(create_rotate_y(box2, -18.0));
    let box2_trans: Translate = create_translation(box2_rotated, point_from_array([130.0, 0.0, 65.0]));

    world.add(constant_medium_from_color(Rc::new(box1_trans), 0.01, point_from_array([0.0, 0.0, 0.0])));
    world.add(constant_medium_from_color(Rc::new(box2_trans), 0.01, point_from_array([1.0, 1.0, 1.0])));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 40.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 278.0, y: 278.0, z: -800.0};
    let look_at: Point3 = Point3{x: 278.0, y: 278.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    // Randomized boxes for the ground
    let mut boxes1: HittableList = HittableList::default();
    let ground: Rc<Lambertian> = lambertian(point_from_array([0.48, 0.83, 0.53]));

    let boxes_per_side: u32 = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w: f64 = 100.0;
            let x0: f64 = -1000.0 + (i as f64)*w;
            let z0: f64 = -1000.0 + (j as f64)*w;
            let y0: f64 = 0.0;
            let x1: f64 = x0 + w;
            let y1: f64 = rand::random_range(1.0..101.0);
            let z1: f64 = z0 + w;

            boxes1.add(create_box(point_from_array([x0,y0,z0]), point_from_array([x1,y1,z1]), ground.clone()));
        }
    }

    let mut world: HittableList = HittableList::default();

    world.add(bvh_node_from_hittable_list(boxes1));

    let light: Rc<DiffuseLight> = diffuse_light_from_color(point_from_array([7.0, 7.0, 7.0]));
    world.add(parallelogram(point_from_array([123.0,554.0,147.0]), point_from_array([300.0, 0.0, 0.0]), point_from_array([0.0,0.0,265.0]), light));

    // Moving sphere that does not move
    let center: Point3 = point_from_array([400.0, 400.0, 200.0]);
    let sphere_material: Rc<Lambertian> = lambertian(point_from_array([0.7, 0.3, 0.1]));
    world.add(sphere(center, 50.0, sphere_material));

    // Fuzzy metal and glass spheres
    world.add(sphere(point_from_array([260.0, 150.0, 45.0]), 50.0, dielectric(1.5)));
    world.add(sphere(
        point_from_array([0.0, 150.0, 145.0]), 50.0, metal(point_from_array([0.8, 0.8, 0.9]), 1.0)
    ));

    // Blue sphere with subsurface scattering (volume inside a dielectric)
    let boundary: Rc<Sphere> = Rc::new(sphere(point_from_array([360.0, 150.0, 145.0]), 70.0, dielectric(1.5)));
    world.add_pointer(boundary.clone());
    world.add(constant_medium_from_color(boundary.clone(), 0.2, point_from_array([0.2, 0.4, 0.9])));

    // Big mist covering everything
    let boundary2: Rc<Sphere> = Rc::new(sphere(point_from_array([0.0, 0.0, 0.0]), 5000.0, dielectric(1.5)));
    world.add(constant_medium_from_color(boundary2, 0.0001, point_from_array([1.0, 1.0, 1.0])));

    // Earth texture
    let emat: Rc<Lambertian> = Rc::new(Lambertian{texture: create_image_texture("textures/earthmap.jpg")});
    world.add(sphere(point_from_array([400.0, 200.0, 400.0]), 100.0, emat));

    // Perlin sphere
    let perlin_texture: Rc<PerlinNoiseTexture>  = Rc::new(PerlinNoiseTexture { perlin_noise: create_perlin_noise(), scale: 0.2});
    let perlin_material: Rc<Lambertian> = Rc::new(Lambertian{ texture: perlin_texture });
    world.add(sphere(point_from_array([220.0, 280.0, 300.0]), 80.0, perlin_material));

    // Group of spheres
    let mut spheres: HittableList = HittableList::default();
    let white: Rc<Lambertian> = lambertian(point_from_array([0.73, 0.73, 0.73]));

    let ns: u32 = 1000;
    for _ in 0..ns {
        spheres.add(sphere(random_vector(0.0, 165.0), 10.0, white.clone()));
    }
    // Translate and rotate them at the same time
    world.add_pointer(Rc::new(create_translation(
            Rc::new(
                create_rotate_y(
                    Rc::new(bvh_node_from_hittable_list(spheres)),
                    15.0)
            ),
            point_from_array([-100.0, 270.0, 395.0])
            ))
    );

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = image_width;
    let samples_per_pixel: u32 = samples_per_pixel;
    let max_depth: u32 = max_depth;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 40.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 478.0, y: 278.0, z: -600.0};
    let look_at: Point3 = Point3{x: 278.0, y: 278.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn cornell_quadric() {
    let mut world: HittableList = HittableList::default();

    let red: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.65, y: 0.05, z: 0.05 }) });
    let white: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.73, y: 0.73, z: 0.73 }) });
    let green: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.12, y: 0.45, z: 0.15 }) });
    let diffuse_light: Rc<DiffuseLight> = diffuse_light_from_color(Point3 { x: 15.0, y: 15.0, z: 15.0 });

    world.add(parallelogram(Point3{x: 555.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z:0.0}, Point3{x: 0.0, y:0.0, z:555.0}, green));
    world.add(parallelogram(Point3{x: 0.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y: 555.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:555.0}, red));
    world.add(parallelogram(Point3{x:  343.0, y: 554.0, z: 332.0}, Point3{x: -130.0, y: 0.0, z: 0.0}, Point3{x: 0.0, y:0.0, z:-105.0}, diffuse_light));
    world.add(parallelogram(point_from_array([0.0, 555.0, 0.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 0.0, 555.0]), white.clone()));
    world.add(parallelogram(point_from_array([0.0, 0.0, 0.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 0.0, 555.0]), white.clone()));
    world.add(parallelogram(point_from_array([0.0, 0.0, 555.0]), point_from_array([555.0, 0.0, 0.0]), point_from_array([0.0, 555.0, 0.0]), white.clone()));

    world.add(y_cylinder(100.0, white.clone()));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 20;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 40.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 278.0, y: 278.0, z: -800.0};
    let look_at: Point3 = Point3{x: 278.0, y: 278.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn debug_quadric() {
    let mut world: HittableList = HittableList::default();

    let diffuse_light: Rc<DiffuseLight> = diffuse_light_from_color(Point3 { x: 15.0, y: 15.0, z: 15.0 });

    world.add(parallelogram(Point3{x:  10.0, y: 10.0, z: 10.0}, Point3{x: 10.0, y: 0.0, z: 10.0}, Point3{x: 0.0, y:10.0, z:10.0}, diffuse_light));

    let white: Rc<Lambertian> = Rc::new(Lambertian{ texture: create_solid_color(Point3 { x: 0.73, y: 0.73, z: 0.73 }) });

    world.add(sphere(Point3 { x: 0.0, y: 0.0, z: 0.0 }, 1.0, white.clone()));
    // world.add(y_cylinder(1.0, white.clone()));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 300;
    let samples_per_pixel: u32 = 20;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {samples_per_pixel, max_depth};

    let background_color: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let vfov: f64 = 40.0;
    let defocus_angle:f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens { defocus_angle, focus_distance };

    let look_from: Point3 = Point3{x: 0.0, y: 0.0, z: 12.0};
    let look_at: Point3 = Point3{x: 0.0, y: 0.0, z: 0.0};
    let view_up: Point3 = Point3{x: 0.0, y: 1.0, z: 0.0};

    let camera_position: CameraPosition = CameraPosition { look_from, look_at, view_up };

    let cam: Camera = create_camera(aspect_ratio, image_width, image_quality, vfov, lens, camera_position, background_color);

    cam.render(&world);
}

fn main() {
    let scene_number: u32 = 151;

    match scene_number {
        0 => many_spheres(),
        1 => checkered_spheres(),
        2 => earth(),
        3 => perlin_spheres(),
        4 => para(),
        5 => simple_light(),
        6 => cornell_box(),
        7 => cornell_smoke(),
        8 => final_scene(800, 10_000, 40),
        9 => cornell_quadric(),
        10 => debug_quadric(),
        _ => final_scene(400, 20, 4),
    }
}
