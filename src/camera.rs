use std::{cmp, fs::File, io::{BufWriter, Write}};

use crate::point3::{Point3, unit_vector};
use crate::point3::color::write_color;
use crate::ray::Ray;
use crate::hittable::HitResult;
use crate::hittable_list::HittableList;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3
}

pub fn create_camera(aspect_ratio: f64, image_width: u32) -> Camera {
    // Calculate the image height, and ensure that it's at least 1.
    let image_height: u32 = cmp::max(1, (image_width as f64 / aspect_ratio) as u32);

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64  = 2.0;
    let viewport_width: f64 = VIEWPORT_HEIGHT * (image_width as f64/image_height as f64);
    const CAMERA_CENTER: Point3 =Point3{x: 0.0, y: 0.0, z: 0.0};
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Point3 = Point3{x:viewport_width, y:0.0, z:0.0};
    let viewport_v: Point3 = Point3{x:0.0, y:-VIEWPORT_HEIGHT, z:0.0};

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Point3 = viewport_u / image_width as f64;
    let pixel_delta_v: Point3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Point3 = CAMERA_CENTER - Point3{x:0.0, y:0.0, z:FOCAL_LENGTH} - viewport_u*0.5f64 - viewport_v*0.5f64;
    let pixel00_loc: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v)*0.5f64;
    // I don't like to have this many parameters here
    Camera { aspect_ratio: aspect_ratio, image_width: image_width, image_height: image_height, center: CAMERA_CENTER, pixel00_loc: pixel00_loc, pixel_delta_u: pixel_delta_u, pixel_delta_v: pixel_delta_v }
}

// Public

impl Camera {
    pub fn render(&self, world: HittableList) {
        // Render
        let image: File = File::create("images/image.ppm").unwrap();
        let mut image_buffer: BufWriter<File> = BufWriter::new(image);

        image_buffer.write_all(&format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes()).unwrap();

        for j in 0..self.image_height {
            // https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
            print!("\r                         ");
            print!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center: Point3 = self.pixel00_loc + (self.pixel_delta_u * i as f64 ) + (self.pixel_delta_v * j as f64);
                let ray_direction: Point3 = pixel_center - self.center;
                let current_ray: Ray = Ray{origin: self.center, direction: ray_direction};

                let pixel_color: Point3 = self.ray_color(&current_ray, &world);

                write_color(&mut image_buffer, pixel_color);
            }
        }
        println!("\nDone!");

        image_buffer.flush().unwrap();

    }
}

// Private

impl Camera {
    fn ray_color(&self, given_ray: &Ray, world: &HittableList) -> Point3 {
        match world.hit(given_ray, 0.001..f64::INFINITY) {
            HitResult::DidNotHit => {},
            HitResult::HitRecord(hit_record) => {
                return (hit_record.normal + Point3{x: 1.0, y: 1.0, z: 1.0})*0.5;
            }
        }

        // Lerp between blue and white
        let unit_direction: Point3 = unit_vector(given_ray.direction);
        let a: f64 = 0.5*(unit_direction.y + 1.0);
        return Point3{x: 1.0, y: 1.0, z: 1.0}*(1.0 - a) + Point3{x: 0.5, y: 0.7, z: 1.0}*a;
    }
}