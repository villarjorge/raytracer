use std::{
    cmp,
    fs::File,
    io::{BufWriter, Write},
};

use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressIterator};
use rand;
use rayon::prelude::*;

use crate::hittable::{Hittable};
use crate::point3::color::write_color;
use crate::point3::{Point3, Vector3, cross, random_in_unit_disk, unit_vector};
use crate::ray::Ray;
use crate::{
    hittable::{HitRecord, SurfaceCoordinate},
    material::{Lambertian, ScatteredRayAndAttenuation},
    point3::color::{Color, proccess_color},
};

pub struct Camera {
    // To do: Consider changing the u32 to u16 or even smaller
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    camera_center: Point3,
    defocus_angle: f64,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
    background_color: Point3,
}

// Create a few structs to group similar arguments together and reduce the arguments to pass to create_camera
pub struct CameraPosition {
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vector3,
}

pub struct ThinLens {
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

pub struct ImageQuality {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl ImageQuality {
    /// A low quality preset for debuging and prototyping. It has 20 samples per pixel and rays stop after 4 bounces
    pub fn low_quality() -> ImageQuality {
        ImageQuality {
            samples_per_pixel: 20,
            max_depth: 4,
        }
    }
    /// A medium quality preset. It has 20 samples per pixel and rays stop after 50 bounces
    pub fn medium_quality() -> ImageQuality {
        ImageQuality {
            samples_per_pixel: 200,
            max_depth: 50,
        }
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        image_quality: ImageQuality,
        vfov: f64,
        thin_lens: ThinLens,
        camera_position: CameraPosition,
        background_color: Point3,
    ) -> Camera {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height: u32 = cmp::max(1, (image_width as f64 / aspect_ratio) as u32);

        let camera_center: Point3 = camera_position.look_from;

        // Determine viewport dimensions
        let theta: f64 = vfov.to_radians();
        let h: f64 = (theta * 0.5).tan();
        let viewport_height: f64 = 2.0 * h * thin_lens.focus_distance;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the basis vectors for the camera coordinate frame
        let w: Vector3 = unit_vector(camera_position.look_from - camera_position.look_at);
        let u: Vector3 = unit_vector(cross(&camera_position.view_up, &w));
        let v: Vector3 = unit_vector(cross(&w, &u));

        // Calcualte the vectors across the horizontal and down the vertical viewport edges
        let viewport_u: Vector3 = u * viewport_width;
        let viewport_v: Vector3 = v * (-viewport_height);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: Point3 = viewport_u / image_width as f64;
        let pixel_delta_v: Point3 = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel (the 0, 0 pixel).
        let viewport_upper_left: Point3 = camera_center
            - w * thin_lens.focus_distance
            - viewport_u * 0.5f64
            - viewport_v * 0.5f64;
        let pixel00_loc: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5f64;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius: f64 =
            thin_lens.focus_distance * ((thin_lens.defocus_angle * 0.5).to_radians().tan());
        let defocus_disk_u: Point3 = u * defocus_radius;
        let defocus_disk_v: Point3 = v * defocus_radius;

        // To do: I don't like to have this many parameters here. Maybe use ray to encapsulate two points?
        let samples_per_pixel: u32 = image_quality.samples_per_pixel;
        let max_depth: u32 = image_quality.max_depth;
        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            camera_center,
            defocus_angle: thin_lens.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background_color,
        }
    }
}

// Public

impl Camera {
    fn create_image_and_buffer(&self, path: &str) -> BufWriter<File> {
        let image: File = File::create(path).unwrap();
        let mut image_buffer: BufWriter<File> = BufWriter::new(image);

        image_buffer
            .write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .unwrap();
        image_buffer
    }

    pub fn render_ppm(&self, world: &dyn Hittable) {
        // Render a ppm image
        let mut image_buffer: BufWriter<File> = self.create_image_and_buffer("images/image.ppm");

        println!("Scan lines progress:");
        for j in (0..self.image_height).progress() {
            // https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
            // eprint!("\r----Scanlines remaining: {}/{}----", self.image_height - j, self.image_height); // eprint since this is the progress of the program
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::default(); // To do: Accumulating step by step could lead to decreased accuracy
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    // Instead of making ray color a method of Camera, do it like this.
                    // To do: make background color a texture
                    pixel_color =
                        pixel_color + ray_color(&r, self.max_depth, world, self.background_color);
                }
                write_color(
                    &mut image_buffer,
                    pixel_color / (self.samples_per_pixel as f64),
                );
            }
        }
        println!("\nRender done!");

        image_buffer.flush().unwrap();
    }

    pub fn render_iterators(&self, world: &dyn Hittable) {
        // The same render function but with iterators instead of loops
        let mut image_buffer: BufWriter<File> = self.create_image_and_buffer("images/image.ppm");

        println!("Scan lines progress:");
        // To use rayon: Import its prelude, transform range -> (range).into_par_iter()
        // A problem is that writting to disk will become non secuential
        // Maybe switch from .ppm to other format and handle it with image crate. See creating a fractal: https://github.com/image-rs/image/blob/main/README.md
        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let pixel_color: Color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let r: Ray = self.get_ray(i, j);
                        ray_color(&r, self.max_depth, world, self.background_color)
                    })
                    .sum();

                write_color(
                    &mut image_buffer,
                    pixel_color / (self.samples_per_pixel as f64),
                );
            }
        }

        println!("\nRender done!");

        image_buffer.flush().unwrap();
    }

    pub fn thrender(&self, world: &dyn Hittable) {
        let mut image_buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            RgbImage::new(self.image_width, self.image_height);

        println!("Scan lines progress:");
        // for (i, j, pixel) in image_buffer.enumerate_pixels_mut().progress() {
        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let pixel_color: Color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let r: Ray = self.get_ray(i, j);
                        ray_color(&r, self.max_depth, world, self.background_color)
                    })
                    .sum();
                let pixel: &mut image::Rgb<u8> = image_buffer.get_pixel_mut(i, j);
                *pixel = image::Rgb(proccess_color(
                    pixel_color / (self.samples_per_pixel as f64),
                ));
            }
        }
        println!("\nRender done!");
        image_buffer.save("images/image.png").unwrap();
    }

    // https://stackoverflow.com/questions/25649423/sending-trait-objects-between-threads-in-rust
    // Add a constraint to the type (the + Sync + Send part)
    // Very easy to convert into parallel code once you know that par_enumerate_pixels_mut exists and you manage to sort out its dependencies
    // For a while it said that image_buffer.par_enumerate_pixels_mut() was not an iterator
    pub fn thrender2(&self, world: &(dyn Hittable + Sync + Send)) {
        let mut image_buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = RgbImage::new(self.image_width, self.image_height);

        println!("Scan lines progress:");
        image_buffer.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
            let pixel_color: Color = (0..self.samples_per_pixel)
                .map(|_| {
                    let r: Ray = self.get_ray(i, j);
                    ray_color2(&r, self.max_depth, world, self.background_color)
                })
                .sum();
            *pixel = image::Rgb(proccess_color(
                pixel_color / (self.samples_per_pixel as f64),
            ));
        });
        println!("\nRender done!");
        image_buffer.save("images/image.png").unwrap();
    }
}

// Private

fn ray_color(given_ray: &Ray, depth: u32, world: &dyn Hittable, background_color: Color) -> Color {
    if depth == 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let mut hit_record: HitRecord = HitRecord {
        p: Point3::default(),
        normal: Point3::default(),
        material: Lambertian::from_color(Point3::default()),
        t: 0.0,
        surface_coords: SurfaceCoordinate { u: 0.0, v: 0.0 },
        front_face: false,
    };

    if !world.hit(given_ray, &(0.001..f64::INFINITY), &mut hit_record) {
        // If the ray hits nothing return the background color
        return background_color;
    }

    let mut sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation {
        scattered_ray: Ray {
            origin: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            inverse_direction: Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        },
        attenuation: Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,},
    };

    let color_from_emission: Color = hit_record
        .material
        .emitted(hit_record.surface_coords, &hit_record.p);

    if !hit_record
        .material
        .scatter(given_ray, &hit_record, &mut sca_att)
    {
        return color_from_emission;
    }

    let color_from_scatter: Color =
        sca_att.attenuation * ray_color(&sca_att.scattered_ray, depth - 1, world, background_color);

    color_from_emission + color_from_scatter
}

fn ray_color2(given_ray: &Ray, depth: u32, world: &(dyn Hittable + Sync + Send), background_color: Color) -> Color {
    if depth == 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let mut hit_record: HitRecord = HitRecord {
        p: Point3::default(),
        normal: Point3::default(),
        material: Lambertian::from_color(Point3::default()),
        t: 0.0,
        surface_coords: SurfaceCoordinate { u: 0.0, v: 0.0 },
        front_face: false,
    };

    if !world.hit(given_ray, &(0.001..f64::INFINITY), &mut hit_record) {
        // If the ray hits nothing return the background color
        return background_color;
    }

    let mut sca_att: ScatteredRayAndAttenuation = ScatteredRayAndAttenuation {
        scattered_ray: Ray {
            origin: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            inverse_direction: Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        },
        attenuation: Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,},
    };

    let color_from_emission: Color = hit_record
        .material
        .emitted(hit_record.surface_coords, &hit_record.p);

    if !hit_record
        .material
        .scatter(given_ray, &hit_record, &mut sca_att)
    {
        return color_from_emission;
    }

    let color_from_scatter: Color =
        sca_att.attenuation * ray_color(&sca_att.scattered_ray, depth - 1, world, background_color);

    color_from_emission + color_from_scatter
}

impl Camera {
    /// Construct a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset: Point3 = sample_square();
        let pixel_sample: Point3 = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin: Point3 = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction: Point3 = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let p: Point3 = random_in_unit_disk();
        self.camera_center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}

/// Returns a vector to a random point in the x, y € [-0.5, 0.5] square
fn sample_square() -> Vector3 {
    Vector3 {
        x: rand::random_range(-0.5..0.5),
        y: rand::random_range(-0.5..0.5),
        z: 0.0f64,
    }
}
