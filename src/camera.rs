use std::{cmp, fs::File, io::{BufWriter, Write}};

use rand;

use crate::{material::ScatterResult, point3::{Point3, cross, random_in_unit_disk, unit_vector}};
use crate::point3::color::write_color;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitResult};

pub struct Camera {
    // Consider changing the u32 to u16 or even smaller
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32, 
    max_depth: u32,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
    camera_center: Point3,
    defocus_angle: f64,
    defocus_disk_u: Point3,
    defocus_disk_v: Point3
}

pub struct CameraPosition {
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Point3,
}

pub struct ThinLens {
    pub defocus_angle: f64,
    pub focus_distance: f64
}

// To do: improve the parameters in this function
pub fn create_camera(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64, thin_lens: ThinLens, camera_position: CameraPosition) -> Camera {
    // Calculate the image height, and ensure that it's at least 1.
    let image_height: u32 = cmp::max(1, (image_width as f64 / aspect_ratio) as u32);

    let camera_center: Point3 = camera_position.look_from;

    // Determine viewport dimensions
    let theta: f64 = vfov.to_radians();
    let h: f64 = (theta*0.5).tan();
    let viewport_height: f64  = 2.0*h*thin_lens.focus_distance;
    let viewport_width: f64 = viewport_height * (image_width as f64/image_height as f64);
    
    // Calculate the basis vectors for the camera coordinate frame
    let w: Point3 = unit_vector(camera_position.look_from - camera_position.look_at);
    let u: Point3 = unit_vector(cross(&camera_position.view_up, &w));
    let v: Point3 = unit_vector(cross(&w, &u));

    // Calcualte the vectors across the horizontal and down the vertical viewport edges
    let viewport_u: Point3 = u*viewport_width;
    let viewport_v: Point3 = v*(-viewport_height);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Point3 = viewport_u / image_width as f64;
    let pixel_delta_v: Point3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel (the 0, 0 pixel).
    let viewport_upper_left: Point3 = camera_center - w*thin_lens.focus_distance - viewport_u*0.5f64 - viewport_v*0.5f64;
    let pixel00_loc: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v)*0.5f64;

    // Calculate the camera defocus disk basis vectors
    let defocus_radius: f64 = thin_lens.focus_distance*( (thin_lens.defocus_angle*0.5).to_radians().tan() );
    let defocus_disk_u: Point3 = u * defocus_radius;
    let defocus_disk_v: Point3 = v * defocus_radius;

    // To do: I don't like to have this many parameters here. Maybe use ray to encapsulate two points? 
    Camera { image_width, image_height, samples_per_pixel, max_depth, pixel00_loc, pixel_delta_u, pixel_delta_v, camera_center, defocus_angle: thin_lens.defocus_angle, defocus_disk_u, defocus_disk_v }
}

// Public

impl Camera {
    pub fn render(&self, world: &dyn Hittable) {
        // Render
        let image: File = File::create("images/image.ppm").unwrap();
        let mut image_buffer: BufWriter<File> = BufWriter::new(image);

        image_buffer.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes()).unwrap();

        for j in 0..self.image_height {
            // https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
            // To do: better progress bar
            eprint!("\r----Scanlines remaining: {}/{}----", self.image_height - j, self.image_height); // eprint since this is the progress of the program
            for i in 0..self.image_width {
                let mut pixel_color: Point3 = Point3::default(); // to do: Accumulating step by step could lead to decreased accuracy
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + ray_color(&r, self.max_depth, world)
                }
                write_color(&mut image_buffer, pixel_color/(self.samples_per_pixel as f64));
            }
        }
        println!("\nRender done!");

        image_buffer.flush().unwrap();

    }
}

// Private

fn ray_color(given_ray: &Ray, depth: u32, world: &dyn Hittable) -> Point3 {
    if depth == 0 {
        return Point3{x: 0.0, y: 0.0, z: 0.0};
    }

    match world.hit(given_ray, 0.001..f64::INFINITY) {
        HitResult::DidNotHit => {},
        HitResult::HitRecord(hit_record) => {
            match hit_record.material.scatter(given_ray, &hit_record) {
                ScatterResult::DidNotScatter => return Point3{x: 0.0, y: 0.0, z: 0.0},
                ScatterResult::DidScatter(sca_att) => return sca_att.attenuation * ray_color(&sca_att.scattered_ray, depth-1, world)
            }
        }
    }

    // Lerp between blue and white
    let unit_direction: Point3 = unit_vector(given_ray.direction);
    let a: f64 = 0.5*(unit_direction.y + 1.0);
    
    Point3{x: 1.0, y: 1.0, z: 1.0}*(1.0 - a) + Point3{x: 0.5, y: 0.7, z: 1.0}*a
}

impl Camera {
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset: Point3 = sample_square();
        let pixel_sample: Point3 = self.pixel00_loc + (self.pixel_delta_u * (i as f64 + offset.x)) + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin: Point3 = if self.defocus_angle <= 0.0 { self.camera_center } else { self.defocus_disk_sample() };
        let ray_direction: Point3 = pixel_sample - ray_origin;

        Ray{origin:ray_origin, direction:ray_direction}
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p: Point3 = random_in_unit_disk();
        self.camera_center + p.x*self.defocus_disk_u + p.y*self.defocus_disk_v
    }
}

fn sample_square() -> Point3 {
    // Returns a vector to a random point in the x, y € [-0.5, 0.5] square
    Point3 { x: rand::random_range(-0.5..0.5), y: rand::random_range(-0.5..0.5), z: 0.0f64 }
}

