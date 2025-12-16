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
use crate::hittable::triangle::{Triangle, load_model};
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
use crate::point3::{Point3, point_from_array, random_vector};
use crate::texture::{CheckerTexture, ImageTexture, PerlinNoiseTexture, SolidColor, Texture};

// To do: once new() is implemented for hittables, materials and textures standarize the creation of objects in main

fn many_spheres() {
    // World
    let mut world: HittableList = HittableList::default();

    let checker: Arc<CheckerTexture> = CheckerTexture::from_colors(
        3.1,
        Color {
            x: 0.2,
            y: 0.3,
            z: 0.1,
        },
        Color {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        },
    );
    // let ground_material = Lambertian{texture: SolidColor::new(Point3 { x: 0.5, y: 0.5, z: 0.5 })};
    let ground_material: Lambertian = Lambertian { texture: checker };
    world.add(Sphere::new(
        Point3 {
            x: 0.0,
            y: -1000.0,
            z: -1.0,
        },
        1000.0,
        Arc::new(ground_material),
    ));

    const N: i32 = 11;

    for a in -N..N {
        for b in -N..N {
            let choose_mat: f64 = rand::random_range(0.0..1.0);
            let center: Point3 = Point3 {
                x: a as f64 + 0.9 * rand::random_range(0.0..1.0),
                y: 0.2,
                z: b as f64 + 0.9 * rand::random_range(0.0..1.0),
            };

            if (center
                - Point3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .length_squared()
                > 0.0
            {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Point3 = random_vector(0.0, 1.0) * random_vector(0.0, 1.0);
                    let sphere_material: Lambertian = Lambertian {
                        texture: SolidColor::new(albedo),
                    };
                    world.add(Sphere::new(center, 0.2, Arc::new(sphere_material)));
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
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        Arc::new(material1),
    ));

    let material2: Lambertian = Lambertian {
        texture: SolidColor::new(Point3 {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        }),
    };
    world.add(Sphere::new(
        Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        Arc::new(material2),
    ));

    let material3: Metal = Metal {
        albedo: Point3 {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    };
    world.add(Sphere::new(
        Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
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

    let look_from: Point3 = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
        Point3 {
            x: 0.7,
            y: 0.8,
            z: 1.0,
        },
    );

    // To do: Make this a parameter that can be passed in the console
    // If you want to compare without the bvh
    // cam.render(&HittableSlice::from_hittable_list(world));

    let bvh_world: BVHNode = BVHNode::from_hittable_list(world);
    cam.render(&bvh_world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker: Arc<CheckerTexture> = CheckerTexture::from_colors(
        0.10,
        Point3 {
            x: 0.2,
            y: 0.3,
            z: 0.1,
        },
        Point3 {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        },
    );
    let material: Arc<Lambertian> = Lambertian::from_texture(checker);

    world.add(Sphere::new(
        Point3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        10.0,
        material.clone(),
    ));
    world.add(Sphere::new(
        Point3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        10.0,
        material,
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

    let look_from: Point3 = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
        Point3 {
            x: 0.7,
            y: 0.8,
            z: 1.0,
        },
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn earth() {
    let mut world: HittableList = HittableList::default();

    let earth_texture: Arc<dyn Texture> = ImageTexture::new_or_fallback("textures/earthmap.jpg");
    let earth_material: Arc<Lambertian> = Arc::new(Lambertian {
        texture: earth_texture,
    });

    world.add(Sphere::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        2.0,
        earth_material,
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

    let look_from: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 12.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
        Point3 {
            x: 0.7,
            y: 0.8,
            z: 1.0,
        },
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
        Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        perlin_material.clone(),
    ));
    world.add(Sphere::new(
        Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
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

    let look_from: Point3 = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
        Point3 {
            x: 0.7,
            y: 0.8,
            z: 1.0,
        },
    );

    cam.render(&HittableSlice::from_hittable_list(world));
}

fn para() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let left_red: Arc<Lambertian> = Lambertian::from_color(Color {
        x: 1.0,
        y: 0.2,
        z: 0.2,
    });
    let back_green: Arc<Lambertian> = Lambertian::from_color(Color {
        x: 0.2,
        y: 1.0,
        z: 0.2,
    });
    let right_blue: Arc<Lambertian> = Lambertian::from_color(Color {
        x: 0.2,
        y: 0.2,
        z: 1.0,
    });
    let upper_orange: Arc<Lambertian> = Lambertian::from_color(Color {
        x: 1.0,
        y: 0.5,
        z: 0.0,
    });
    let lower_teal: Arc<Lambertian> = Lambertian::from_color(Color {
        x: 0.2,
        y: 0.8,
        z: 0.8,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: -3.0,
            y: -2.0,
            z: 5.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -4.0,
        },
        Point3 {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        },
        left_red,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: -2.0,
            y: -2.0,
            z: 0.0,
        },
        Point3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        },
        back_green,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 3.0,
            y: -2.0,
            z: 1.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 4.0,
        },
        Point3 {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        },
        right_blue,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: -2.0,
            y: 3.0,
            z: 1.0,
        },
        Point3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 4.0,
        },
        upper_orange,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: -2.0,
            y: -3.0,
            z: 5.0,
        },
        Point3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -4.0,
        },
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

    let look_from: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 9.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
        Point3 {
            x: 0.7,
            y: 0.8,
            z: 1.0,
        },
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
        Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        perlin_material.clone(),
    ));
    world.add(Sphere::new(
        Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        2.0,
        perlin_material,
    ));

    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 4.0,
        y: 4.0,
        z: 4.0,
    });
    world.add(Parallelogram::new(
        Point3 {
            x: 3.0,
            y: 1.0,
            z: -2.0,
        },
        Point3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
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

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 20.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 26.0,
        y: 3.0,
        z: 6.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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

fn cornell_box() {
    let mut world: HittableList = HittableList::default();

    let red: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    });
    let white: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });
    let green: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    });
    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 343.0,
            y: 554.0,
            z: 332.0,
        },
        Point3 {
            x: -130.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -105.0,
        },
        diffuse_light,
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 555.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 555.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 555.0, 0.0]),
        white.clone(),
    ));

    let box1: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([165.0, 330.0, 165.0]),
        white.clone(),
    )));
    let box1_rotated: Arc<RotateY> = Arc::new(RotateY::new(box1, 15.0));
    let box1_trans: Translate = Translate::new(box1_rotated, point_from_array([265.0, 0.0, 295.0]));

    world.add(box1_trans);

    let box2: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([165.0, 165.0, 165.0]),
        white,
    )));
    let box2_rotated: Arc<RotateY> = Arc::new(RotateY::new(box2, -18.0));
    let box2_trans: Translate = Translate::new(box2_rotated, point_from_array([130.0, 0.0, 65.0]));

    world.add(box2_trans);

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 20;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: -800.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
    let mut world: HittableList = HittableList::default();

    let red: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    });
    let white: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });
    let green: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    });
    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 7.0,
        y: 7.0,
        z: 7.0,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 113.0,
            y: 554.0,
            z: 127.0,
        },
        Point3 {
            x: 330.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 305.0,
        },
        diffuse_light,
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 555.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 555.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 555.0, 0.0]),
        white.clone(),
    ));

    let box1: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([165.0, 330.0, 165.0]),
        white.clone(),
    )));
    let box1_rotated: Arc<RotateY> = Arc::new(RotateY::new(box1, 15.0));
    let box1_trans: Translate = Translate::new(box1_rotated, point_from_array([265.0, 0.0, 295.0]));

    let box2: Arc<HittableSlice> = Arc::new(HittableSlice::from_hittable_list(create_box(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([165.0, 165.0, 165.0]),
        white,
    )));
    let box2_rotated: Arc<RotateY> = Arc::new(RotateY::new(box2, -18.0));
    let box2_trans: Translate = Translate::new(box2_rotated, point_from_array([130.0, 0.0, 65.0]));

    world.add(ConstantMedium::from_color(
        Arc::new(box1_trans),
        0.01,
        point_from_array([0.0, 0.0, 0.0]),
    ));
    world.add(ConstantMedium::from_color(
        Arc::new(box2_trans),
        0.01,
        point_from_array([1.0, 1.0, 1.0]),
    ));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 20;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: -800.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
    let ground: Arc<Lambertian> = Lambertian::from_color(point_from_array([0.48, 0.83, 0.53]));

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
                    point_from_array([x0, y0, z0]),
                    point_from_array([x1, y1, z1]),
                    ground.clone(),
                )
                .to_hittable_slice(),
            );
        }
    }

    let mut world: HittableList = HittableList::default();

    world.add(BVHNode::from_hittable_list(boxes1));

    let light: Arc<DiffuseLight> = DiffuseLight::from_color(point_from_array([7.0, 7.0, 7.0]));
    world.add(Parallelogram::new(
        point_from_array([123.0, 554.0, 147.0]),
        point_from_array([300.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 265.0]),
        light,
    ));

    // Moving sphere that does not move
    let center: Point3 = point_from_array([400.0, 400.0, 200.0]);
    let sphere_material: Arc<Lambertian> =
        Lambertian::from_color(point_from_array([0.7, 0.3, 0.1]));
    world.add(Sphere::new(center, 50.0, sphere_material));

    // Fuzzy metal and glass spheres
    world.add(Sphere::new(
        point_from_array([260.0, 150.0, 45.0]),
        50.0,
        dielectric(1.5),
    ));
    world.add(Sphere::new(
        point_from_array([0.0, 150.0, 145.0]),
        50.0,
        metal(point_from_array([0.8, 0.8, 0.9]), 1.0),
    ));

    // Blue sphere with subsurface scattering (volume inside a dielectric)
    let boundary: Arc<Sphere> = Arc::new(Sphere::new(
        point_from_array([360.0, 150.0, 145.0]),
        70.0,
        dielectric(1.5),
    ));
    world.add_pointer(boundary.clone());
    world.add(ConstantMedium::from_color(
        boundary.clone(),
        0.2,
        point_from_array([0.2, 0.4, 0.9]),
    ));

    // Big mist covering everything
    let boundary2: Arc<Sphere> = Arc::new(Sphere::new(
        point_from_array([0.0, 0.0, 0.0]),
        5000.0,
        dielectric(1.5),
    ));
    world.add(ConstantMedium::from_color(
        boundary2,
        0.0001,
        point_from_array([1.0, 1.0, 1.0]),
    ));

    // Earth texture
    let emat: Arc<Lambertian> =
        Lambertian::from_texture(ImageTexture::new_or_fallback("textures/earthmap.jpg"));
    world.add(Sphere::new(
        point_from_array([400.0, 200.0, 400.0]),
        100.0,
        emat,
    ));

    // Perlin sphere
    let perlin_texture: Arc<PerlinNoiseTexture> = Arc::new(PerlinNoiseTexture {
        perlin_noise: create_perlin_noise(),
        scale: 0.2,
    });
    let perlin_material: Arc<Lambertian> = Lambertian::from_texture(perlin_texture);
    world.add(Sphere::new(
        point_from_array([220.0, 280.0, 300.0]),
        80.0,
        perlin_material,
    ));

    // Group of spheres
    let mut spheres: HittableList = HittableList::default();
    let white: Arc<Lambertian> = Lambertian::from_color(point_from_array([0.73, 0.73, 0.73]));

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
        point_from_array([-100.0, 270.0, 395.0]),
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = image_width;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 478.0,
        y: 278.0,
        z: -600.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
    let mut world: HittableList = HittableList::default();

    let red: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    });
    let white: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });
    let green: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    });
    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 113.0,
            y: 554.0,
            z: 127.0,
        },
        Point3 {
            x: 330.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 305.0,
        },
        diffuse_light,
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 555.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 555.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 555.0, 0.0]),
        white.clone(),
    ));

    world.add(y_cylinder(
        Point3 {
            x: 150.0,
            y: 555.0 / 2.0,
            z: 175.0,
        },
        50.0,
        white.clone(),
    ));
    world.add(y_cylinder(
        Point3 {
            x: 400.0,
            y: 555.0 / 2.0,
            z: 555.0 / 2.0 + 50.0,
        },
        80.0,
        white.clone(),
    ));
    // world.add(quadric_Sphere::new(Point3 { x: 555.0/2.0, y: 555.0/2.0, z: 555.0/2.0 }, 100.0, white.clone()));
    // world.add(Sphere::new(Point3 { x: 555.0/2.0, y: 555.0/2.0, z: 555.0/2.0 }, 100.0, white.clone()));
    // world.add(y_cone(Point3 { x: 200.0, y: 555.0, z: 200.0 }, Point3 { x: 50.0, y: 50.0, z: 50.0 }, white.clone()));

    let aspect_ratio: f64 = 1.0;
    // let image_width: u32 = 300;
    // let image_quality: ImageQuality = ImageQuality::low_quality();
    let image_width: u32 = 600;
    let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: -800.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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

fn debug_quadric() {
    let mut world: HittableList = HittableList::default();

    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        },
        Point3 {
            x: 10.0,
            y: 0.0,
            z: 10.0,
        },
        Point3 {
            x: 0.0,
            y: 10.0,
            z: 10.0,
        },
        diffuse_light,
    ));

    let white: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });

    world.add(Sphere::new(
        Point3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        1.0,
        white.clone(),
    ));
    world.add(y_cylinder(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        1.0,
        white.clone(),
    ));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 50;
    let max_depth: u32 = 50;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 12.0,
    };
    let look_at: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
    let mut world: HittableList = HittableList::default();

    let red: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    });
    let white: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });
    let green: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    });
    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 113.0,
            y: 554.0,
            z: 127.0,
        },
        Point3 {
            x: 330.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 305.0,
        },
        diffuse_light,
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 555.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 555.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 555.0, 0.0]),
        white.clone(),
    ));

    world.add(Triangle::new(
        Point3 {
            x: 555.0 / 2.0,
            y: 555.0 / 2.0,
            z: 555.0 / 2.0,
        },
        Point3 {
            x: 100.0,
            y: 100.0,
            z: 10.0,
        },
        Point3 {
            x: 100.0,
            y: 0.0,
            z: 100.0,
        },
        white.clone(),
    ));
    // world.add(Parallelogram::new(Point3 { x: 555.0/2.0, y: 555.0/2.0, z: 555.0/2.0 }, Point3 { x: 100.0, y: 100.0, z: 10.0 }, Point3 { x: 100.0, y: 0.0, z: 100.0 }, white.clone()));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 300;
    let image_quality: ImageQuality = ImageQuality::low_quality();
    // let image_width: u32 = 600;
    // let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: -800.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
    // Similar to the final scene but with some of the random elements removed to asses performance
    // boxes for the ground
    let mut boxes1: HittableList = HittableList::default();
    let ground: Arc<Lambertian> = Lambertian::from_color(point_from_array([0.48, 0.83, 0.53]));

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
                    point_from_array([x0, y0, z0]),
                    point_from_array([x1, y1, z1]),
                    ground.clone(),
                )
                .to_hittable_slice(),
            );
        }
    }

    let mut world: HittableList = HittableList::default();

    world.add(BVHNode::from_hittable_list(boxes1));

    let light: Arc<DiffuseLight> = DiffuseLight::from_color(point_from_array([7.0, 7.0, 7.0]));
    world.add(Parallelogram::new(
        point_from_array([123.0, 554.0, 147.0]),
        point_from_array([300.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 265.0]),
        light,
    ));

    // Moving sphere that does not move
    let center: Point3 = point_from_array([400.0, 400.0, 200.0]);
    let sphere_material: Arc<Lambertian> =
        Lambertian::from_color(point_from_array([0.7, 0.3, 0.1]));
    world.add(Sphere::new(center, 50.0, sphere_material));

    // Fuzzy metal and glass spheres
    world.add(Sphere::new(
        point_from_array([260.0, 150.0, 45.0]),
        50.0,
        dielectric(1.5),
    ));
    world.add(Sphere::new(
        point_from_array([0.0, 150.0, 145.0]),
        50.0,
        metal(point_from_array([0.8, 0.8, 0.9]), 1.0),
    ));

    // Blue sphere with subsurface scattering (volume inside a dielectric)
    let boundary: Arc<Sphere> = Arc::new(Sphere::new(
        point_from_array([360.0, 150.0, 145.0]),
        70.0,
        dielectric(1.5),
    ));
    world.add_pointer(boundary.clone());
    world.add(ConstantMedium::from_color(
        boundary.clone(),
        0.2,
        point_from_array([0.2, 0.4, 0.9]),
    ));

    // Earth texture
    let emat: Arc<Lambertian> =
        Lambertian::from_texture(ImageTexture::new_or_fallback("textures/earthmap.jpg"));
    world.add(Sphere::new(
        point_from_array([400.0, 200.0, 400.0]),
        100.0,
        emat,
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
        point_from_array([-100.0, 270.0, 395.0]),
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = image_width;
    let samples_per_pixel: u32 = samples_per_pixel;
    let max_depth: u32 = max_depth;
    let image_quality: ImageQuality = ImageQuality {
        samples_per_pixel,
        max_depth,
    };

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 478.0,
        y: 278.0,
        z: -600.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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
    let mut world: HittableList = HittableList::default();

    let red: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    });
    let white: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });
    let green: Arc<Lambertian> = Lambertian::from_color(Point3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    });
    let diffuse_light: Arc<DiffuseLight> = DiffuseLight::from_color(Point3 {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    });

    world.add(Parallelogram::new(
        Point3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    ));
    world.add(Parallelogram::new(
        Point3 {
            x: 113.0,
            y: 554.0,
            z: 127.0,
        },
        Point3 {
            x: 330.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 305.0,
        },
        diffuse_light,
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 555.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 0.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 0.0, 555.0]),
        white.clone(),
    ));
    world.add(Parallelogram::new(
        point_from_array([0.0, 0.0, 555.0]),
        point_from_array([555.0, 0.0, 0.0]),
        point_from_array([0.0, 555.0, 0.0]),
        white.clone(),
    ));

    let model: BVHNode = load_model("models/pawn.txt", 750.0, white.clone());

    world.add(Translate::new(
        Arc::new(model),
        Point3::new(300.0, 300.0, 300.0),
    ));

    let aspect_ratio: f64 = 1.0;
    // let image_width: u32 = 300;
    let image_width: u32 = 600;
    let image_quality: ImageQuality = ImageQuality::low_quality();
    // let image_quality: ImageQuality = ImageQuality::medium_quality();

    let background_color: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vfov: f64 = 40.0;
    let defocus_angle: f64 = 0.0;
    let focus_distance: f64 = 10.0;

    let lens: ThinLens = ThinLens {
        defocus_angle,
        focus_distance,
    };

    let look_from: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: -800.0,
    };
    let look_at: Point3 = Point3 {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };
    let view_up: Point3 = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

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

fn main() {
    let now: Instant = Instant::now();
    let scene_number: u32 = 13;

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
        _ => final_scene(400, 20, 4),
    }

    println!("Time: {:.2?}", now.elapsed());
}
