pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod perlin;
pub mod point3;
pub mod ray;
pub mod tests;
pub mod texture;

use std::sync::Arc;
use std::time::Instant;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::bvh::BVHNode;
use crate::camera::{Camera, CameraPosition, ImageQuality, ThinLens};
use crate::hittable::hittable_list::HittableSlice;
use crate::hittable::load_obj::load_model;
use crate::hittable::quadric::{Quadric, quadric_sphere};
use crate::hittable::triangle::Triangle;
use crate::hittable::{
    constant_medium::ConstantMedium,
    hittable_list::HittableList,
    parallelogram::{Parallelogram, create_box},
    quadric::y_cylinder,
    sphere::Sphere,
    {RotateY, Translate},
};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, dielectric, metal};
use crate::perlin::create_perlin_noise;
use crate::point3::color::Color;
use crate::point3::{Point3, random_vector};
use crate::texture::{CheckerTexture, ImageTexture, PerlinNoiseTexture, Texture};

// To do: once new() is implemented for hittables, materials and textures standarize the creation of objects in main
// To do: better way to handle creating scenes
fn many_spheres() {
    // World
    let mut world: HittableList = HittableList::default();

    let checker: Arc<CheckerTexture> =
        CheckerTexture::from_colors(3.1, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    // let ground_material = Lambertian{texture: SolidColor::new(Point3::new( 0.5, 0.5, 0.5 ))};
    let ground_material: Lambertian = Lambertian { texture: checker };
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        Arc::new(ground_material),
    ));

    const N: i32 = 11;

    for a in -N..N {
        for b in -N..N {
            let choose_mat: f64 = rand::random_range(0.0..1.0);
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * rand::random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rand::random_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length_squared() > 0.0 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Point3 = random_vector(0.0, 1.0) * random_vector(0.0, 1.0);
                    world.add(Sphere::new(center, 0.2, Lambertian::from_color(albedo)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Point3 = random_vector(0.0, 1.0) * random_vector(0.0, 1.0);
                    let fuzz: f64 = rand::random_range(0.0..0.5);
                    let sphere_material: Metal = Metal { albedo, fuzz };
                    world.add(Sphere::new(center, 0.2, Arc::new(sphere_material)));
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric {
                        refraction_index: 1.5,
                    };
                    world.add(Sphere::new(center, 0.2, Arc::new(sphere_material)));
                }
            }
        }
    }

    let material1: Dielectric = Dielectric {
        refraction_index: 1.5,
    };
    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material1),
    ));

    let material2: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3: Metal = Metal {
        albedo: Point3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material3),
    ));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1200; // 1200
    let samples_per_pixel: u32 = 10; // 500
    let max_depth: u32 = 100;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.6;
    let focus_distance: f64 = 10.0;

    let thin_lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        thin_lens,
        camera_position,
        Color::blue(),
    );

    // To do: Make this a parameter that can be passed in the console
    // If you want to compare without the bvh
    // cam.render(&HittableSlice::from_hittable_list(world));

    let bvh_world: BVHNode = BVHNode::from_hittable_list(world);
    cam.render(&bvh_world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker: Arc<CheckerTexture> =
        CheckerTexture::from_colors(0.10, Point3::new(0.2, 0.3, 0.1), Point3::new(0.9, 0.9, 0.9));
    let material: Arc<Lambertian> = Lambertian::from_texture(checker);

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, material));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        Color::new(0.7, 0.8, 1.0),
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn earth() {
    let mut world: HittableList = HittableList::default();

    let earth_texture: Arc<dyn Texture> = ImageTexture::new_or_fallback("textures/earthmap.jpg");
    let earth_material: Arc<Lambertian> = Arc::new(Lambertian {
        texture: earth_texture,
    });

    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_material));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(0.0, 0.0, 12.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        Point3::new(0.7, 0.8, 1.0),
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::default();

    let perlin_texture: Arc<PerlinNoiseTexture> = Arc::new(PerlinNoiseTexture {
        perlin_noise: create_perlin_noise(),
        scale: 2.0,
    });
    let perlin_material: Arc<Lambertian> = Lambertian::from_texture(perlin_texture);

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    ));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        Color::new(0.7, 0.8, 1.0),
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn para() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let left_red: Arc<Lambertian> = Lambertian::from_color(Color::new(1.0, 0.2, 0.2));
    let back_green: Arc<Lambertian> = Lambertian::from_color(Color::new(0.2, 1.0, 0.2));
    let right_blue: Arc<Lambertian> = Lambertian::from_color(Color::new(0.2, 0.2, 1.0));
    let upper_orange: Arc<Lambertian> = Lambertian::from_color(Color::new(1.0, 0.5, 0.0));
    let lower_teal: Arc<Lambertian> = Lambertian::from_color(Color::new(0.2, 0.8, 0.8));

    world.add(Parallelogram::new(
        Point3::new(-3.0, -2.0, 5.0),
        Point3::new(0.0, 0.0, -4.0),
        Point3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Parallelogram::new(
        Point3::new(-2.0, -2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
        Point3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Parallelogram::new(
        Point3::new(3.0, -2.0, 1.0),
        Point3::new(0.0, 0.0, 4.0),
        Point3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Parallelogram::new(
        Point3::new(-2.0, 3.0, 1.0),
        Point3::new(4.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Parallelogram::new(
        Point3::new(-2.0, -3.0, 5.0),
        Point3::new(4.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let vfov: f64 = 80.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(0.0, 0.0, 9.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        Color::new(0.7, 0.8, 1.0),
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn simple_light() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let perlin_texture: Arc<PerlinNoiseTexture> = Arc::new(PerlinNoiseTexture {
        perlin_noise: create_perlin_noise(),
        scale: 2.0,
    });
    let perlin_material: Arc<Lambertian> = Lambertian::from_texture(perlin_texture);

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    ));

    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    world.add(Parallelogram::new(
        Point3::new(3.0, 1.0, -2.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        diffuse_light,
    ));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(26.0, 3.0, 6.0);
    let look_at: Point3 = Point3::new(0.0, 2.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

/// Creates an empty cornell box, returning the HittableList with the quadrilaterals and light
fn create_empty_cornell_box() -> HittableList {
    let mut world: HittableList = HittableList::default();

    let red: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.65, 0.05, 0.05));
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));
    let green: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.12, 0.45, 0.15));
    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3::new(15.0, 15.0, 15.0));

    world.add(Parallelogram::new(
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Parallelogram::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        red,
    ));
    // Smaller light
    // world.add(Parallelogram::new(
    //     Point3::new(343.0, 554.0, 332.0),
    //     Point3::new(-130.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, -105.0),
    //     diffuse_light,
    // ));
    // Bigger light
    world.add(Parallelogram::new(
        Point3::new(113.0, 554.0, 127.0),
        Point3::new(330.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 305.0),
        diffuse_light,
    ));
    world.add(Parallelogram::new(
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));
    world
}

fn cornell_box() {
    let mut world: HittableList = create_empty_cornell_box();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    let box1: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    let box1_rotated: Arc<RotateY> = Arc::new(RotateY::new(box1, 15.0));
    let box1_trans: Translate = Translate::new(box1_rotated, Point3::new(265.0, 0.0, 295.0));

    world.add(box1_trans);

    let box2: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    )));
    let box2_rotated: Arc<RotateY> = Arc::new(RotateY::new(box2, -18.0));
    let box2_trans: Translate = Translate::new(box2_rotated, Point3::new(130.0, 0.0, 65.0));

    world.add(box2_trans);

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 20;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(278.0, 278.0, -800.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn cornell_smoke() {
    let mut world: HittableList = create_empty_cornell_box();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    let box1: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    let box1_rotated: Arc<RotateY> = Arc::new(RotateY::new(box1, 15.0));
    let box1_trans: Translate = Translate::new(box1_rotated, Point3::new(265.0, 0.0, 295.0));

    let box2: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    )));
    let box2_rotated: Arc<RotateY> = Arc::new(RotateY::new(box2, -18.0));
    let box2_trans: Translate = Translate::new(box2_rotated, Point3::new(130.0, 0.0, 65.0));

    world.add(ConstantMedium::from_color(
        Arc::new(box1_trans),
        0.01,
        Point3::new(0.0, 0.0, 0.0),
    ));
    world.add(ConstantMedium::from_color(
        Arc::new(box2_trans),
        0.01,
        Point3::new(1.0, 1.0, 1.0),
    ));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 20;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(278.0, 278.0, -800.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    // Randomized boxes for the ground
    let mut boxes1: HittableList = HittableList::default();
    let ground: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.48, 0.83, 0.53));

    // In total 400 boxes, 2400 parallelograms
    let boxes_per_side: u32 = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w: f64 = 100.0;
            let x0: f64 = -1000.0 + (i as f64) * w;
            let z0: f64 = -1000.0 + (j as f64) * w;
            let y0: f64 = 0.0;
            let x1: f64 = x0 + w;
            let y1: f64 = rand::random_range(1.0..101.0);
            let z1: f64 = z0 + w;

            boxes1.add(
                create_box(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground.clone(),
                )
                .to_hittable_slice(),
            );
        }
    }

    let mut world: HittableList = HittableList::default();

    world.add(BVHNode::from_hittable_list(boxes1));

    let light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3::new(7.0, 7.0, 7.0));
    world.add(Parallelogram::new(
        Point3::new(123.0, 554.0, 147.0),
        Point3::new(300.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 265.0),
        light,
    ));

    // Moving sphere that does not move
    let center: Point3 = Point3::new(400.0, 400.0, 200.0);
    let sphere_material: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.7, 0.3, 0.1));
    world.add(Sphere::new(center, 50.0, sphere_material));

    // Fuzzy metal and glass spheres
    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        dielectric(1.5),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        metal(Point3::new(0.8, 0.8, 0.9), 1.0),
    ));

    // Blue sphere with subsurface scattering (volume inside a dielectric)
    let boundary: Arc<Sphere> = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        dielectric(1.5),
    ));
    world.add_pointer(boundary.clone());
    world.add(ConstantMedium::from_color(
        boundary.clone(),
        0.2,
        Point3::new(0.2, 0.4, 0.9),
    ));

    // Big mist covering everything
    let boundary2: Arc<Sphere> = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        dielectric(1.5),
    ));
    world.add(ConstantMedium::from_color(
        boundary2,
        0.0001,
        Point3::new(1.0, 1.0, 1.0),
    ));

    // Earth texture
    let emat: Arc<Lambertian> =
        Lambertian::from_texture(ImageTexture::new_or_fallback("textures/earthmap.jpg"));
    world.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));

    // Perlin sphere
    let perlin_texture: Arc<PerlinNoiseTexture> = Arc::new(PerlinNoiseTexture {
        perlin_noise: create_perlin_noise(),
        scale: 0.2,
    });
    let perlin_material: Arc<Lambertian> = Lambertian::from_texture(perlin_texture);
    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    ));

    // Group of spheres
    let mut spheres: HittableList = HittableList::default();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    let number_of_spheres: u32 = 1000;
    for _ in 0..number_of_spheres {
        spheres.add(Sphere::new(random_vector(0.0, 165.0), 10.0, white.clone()));
    }
    // Translate and rotate them at the same time
    world.add_pointer(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::from_hittable_list(spheres)),
            15.0,
        )),
        Point3::new(-100.0, 270.0, 395.0),
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = image_width;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(478.0, 278.0, -600.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );
    // cam.render(&HittableSlice::from_hittable_list(world));
    cam.thrender(&HittableSlice::from_hittable_list(world));
}

fn cornell_quadric() {
    let mut world: HittableList = create_empty_cornell_box();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    world.add(y_cylinder(
        Point3::new(150.0, 555.0 / 2.0, 175.0),
        50.0,
        white.clone(),
    ));
    world.add(y_cylinder(
        Point3::new(400.0, 555.0 / 2.0, 555.0 / 2.0 + 50.0),
        80.0,
        white.clone(),
    ));
    // world.add(quadric_Sphere::new(Point3::new( x: 555.0/2.0, y: 555.0/2.0, z: 555.0/2.0 }, 100.0, white.clone()));
    // world.add(Sphere::new(Point3::new( x: 555.0/2.0, y: 555.0/2.0, z: 555.0/2.0 }, 100.0, white.clone()));
    // world.add(y_cone(Point3::new( x: 200.0, y: 555.0, z: 200.0 }, Point3::new( x: 50.0, y: 50.0, z: 50.0 }, white.clone()));

    let aspect_ratio: f64 = 1.0;
    // let image_width: u32 = 300;
    // let image_quality: ImageQuality = ImageQuality::low_quality();
    let image_width: u32 = 600;
    let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(278.0, 278.0, -800.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.thrender(&HittableSlice::from_hittable_list(world));
}

fn debug_quadric() {
    let mut world: HittableList = HittableList::default();

    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3::new(15.0, 15.0, 15.0));

    world.add(Parallelogram::new(
        Point3::new(10.0, 10.0, 10.0),
        Point3::new(10.0, 0.0, 10.0),
        Point3::new(0.0, 10.0, 10.0),
        diffuse_light,
    ));

    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    world.add(Sphere::new(Point3::new(3.0, 0.0, 0.0), 1.0, white.clone()));
    world.add(y_cylinder(Point3::new(0.0, 0.0, 0.0), 1.0, white.clone()));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 50;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(0.0, 0.0, 12.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn cornell_triangle() {
    let mut world: HittableList = create_empty_cornell_box();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    world.add(Triangle::new(
        Point3::new(555.0 / 2.0, 555.0 / 2.0, 555.0 / 2.0),
        Point3::new(100.0, 100.0, 10.0),
        Point3::new(100.0, 0.0, 100.0),
        white.clone(),
    ));
    // world.add(Parallelogram::new(Point3::new( x: 555.0/2.0, y: 555.0/2.0, z: 555.0/2.0 }, Point3::new( x: 100.0, y: 100.0, z: 10.0 }, Point3::new( x: 100.0, y: 0.0, z: 100.0 }, white.clone()));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 300;
    let image_quality: ImageQuality = ImageQuality::low_quality();
    // let image_width: u32 = 600;
    // let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(278.0, 278.0, -800.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn profiler_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    // Similar to the final scene but with some of the random elements removed to assess performance
    // boxes for the ground
    let mut boxes1: HittableList = HittableList::default();
    let ground: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.48, 0.83, 0.53));

    let mut rng: SmallRng = SmallRng::seed_from_u64(42_u64);
    // In total 400 boxes, 2400 parallelograms
    let boxes_per_side: u32 = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w: f64 = 100.0;
            let x0: f64 = -1000.0 + (i as f64) * w;
            let z0: f64 = -1000.0 + (j as f64) * w;
            let y0: f64 = 0.0;
            let x1: f64 = x0 + w;
            let y1: f64 = rng.random_range(1.0..101.0);
            let z1: f64 = z0 + w;

            boxes1.add(
                create_box(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground.clone(),
                )
                .to_hittable_slice(),
            );
        }
    }

    let mut world: HittableList = HittableList::default();

    world.add(BVHNode::from_hittable_list(boxes1));

    let light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3::new(7.0, 7.0, 7.0));
    world.add(Parallelogram::new(
        Point3::new(123.0, 554.0, 147.0),
        Point3::new(300.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 265.0),
        light,
    ));

    // Moving sphere that does not move
    let center: Point3 = Point3::new(400.0, 400.0, 200.0);
    let sphere_material: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.7, 0.3, 0.1));
    world.add(Sphere::new(center, 50.0, sphere_material));

    // Fuzzy metal and glass spheres
    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        dielectric(1.5),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        metal(Point3::new(0.8, 0.8, 0.9), 1.0),
    ));

    // Blue sphere with subsurface scattering (volume inside a dielectric)
    let boundary: Arc<Sphere> = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        dielectric(1.5),
    ));
    world.add_pointer(boundary.clone());
    world.add(ConstantMedium::from_color(
        boundary.clone(),
        0.2,
        Point3::new(0.2, 0.4, 0.9),
    ));

    // Earth texture
    let emat: Arc<Lambertian> =
        Lambertian::from_texture(ImageTexture::new_or_fallback("textures/earthmap.jpg"));
    world.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));

    // Group of spheres
    let mut spheres: HittableList = HittableList::default();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    let number_of_spheres: u32 = 1000;
    for _ in 0..number_of_spheres {
        spheres.add(Sphere::new(random_vector(0.0, 165.0), 10.0, white.clone()));
    }
    // Translate and rotate them at the same time
    world.add_pointer(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::from_hittable_list(spheres)),
            15.0,
        )),
        Point3::new(-100.0, 270.0, 395.0),
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = image_width;
    let samples_per_pixel: u32 = samples_per_pixel;
    let max_depth: u32 = max_depth;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(478.0, 278.0, -600.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn cornell_model() {
    let mut world: HittableList = create_empty_cornell_box();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    // let model: BVHNode = load_model("models/Pawn/CHAHIN_PAWN.obj", 750.0, white.clone());
    let model: BVHNode = load_model("models/teapot.obj", 75.0, white.clone());

    world.add(Translate::new(
        Arc::new(model),
        Point3::new(300.0, 150.0, 300.0),
    ));

    let aspect_ratio: f64 = 1.0;
    // let image_width: u32 = 300;
    let image_width: u32 = 600;
    let image_quality: ImageQuality = ImageQuality::low_quality();
    // let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Point3 = Point3::new(0.0, 0.0, 0.0);

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(278.0, 278.0, -800.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.thrender(&HittableSlice::from_hittable_list(world));
}

fn spherical_mirror() {
    let mut world: HittableList = HittableList::default();

    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3::new(15.0, 15.0, 15.0));
    let sphere_light: Sphere = Sphere::new(Point3::new(0.0, 300.0, 0.0), 100.0, diffuse_light);
    world.add(sphere_light);

    let metal: Arc<Metal> = metal(Color::new(0.8, 0.8, 0.8), 0.0);
    let mirror_sphere_small: Quadric =
        quadric_sphere(Point3::new(0.0, 0.0, 0.0), 10.0, metal.clone());
    world.add(mirror_sphere_small);

    // Group of spheres
    let mut spheres: HittableList = HittableList::default();
    let white: Arc<Lambertian> = Lambertian::from_color(Point3::new(0.73, 0.73, 0.73));

    let number_of_spheres: u32 = 100;
    for _ in 0..number_of_spheres {
        spheres.add(Sphere::new(random_vector(-25.0, 25.0), 1.0, white.clone()));
    }

    world.add(spheres.to_bvh_node());

    let blue: Arc<Lambertian> =
        Lambertian::from_color(Color::new(0.0, 48.0 / 255.0, 143.0 / 255.0));

    world.add(Parallelogram::new(
        Point3::new(-30.0, -30.0, -30.0),
        Point3::new(60.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 60.0),
        blue.clone(),
    ));
    // To do: being inside a sphere does not seem to work
    // let mirror_sphere_big: Quadric = quadric_sphere(Point3::new(0.0, 0.0, 0.0), 600.0, metal.clone());
    // world.add(mirror_sphere_big);

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Color = Color::black();

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3::new(-100.0, 0.0, 0.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let view_up: Point3 = Point3::new(0.0, 1.0, 0.0);

    let camera_position: CameraPosition = CameraPosition {
        look_from,
        look_at,
        view_up,
    };

    let cam: Camera = Camera::new(
        aspect_ratio,
        image_width,
        image_quality,
        vfov,
        lens,
        camera_position,
        background_color,
    );

    cam.thrender(&world.to_hittable_slice());
}

fn main() {
    let now: Instant = Instant::now();
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
        11 => cornell_triangle(),
        12 => profiler_scene(400, 20, 4),
        13 => final_scene(800, 200, 50),
        14 => cornell_model(),
        15 => spherical_mirror(),
        _ => final_scene(400, 20, 4),
    }

    println!("Image rendered in: {:.2?}", now.elapsed());
}
