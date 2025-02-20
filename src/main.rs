use std::{cmp, fs::File, io::{BufWriter, Write}};

pub mod point3;
pub mod ray;

pub use crate::point3::{Point3, unit_vector};
pub use crate::point3::color::write_color;
pub use crate::ray::Ray;

fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    let oc: Point3 = center - ray.origin;
    let a: f32 = ray.direction.dot(ray.direction);
    let b: f32 = -2.0 * oc.dot(ray.direction);
    let c: f32 = oc.dot(oc) - radius*radius;
    let discriminant: f32 = b*b - 4.0*a*c;
    
    if discriminant < 0.0 {
        return -1.0;
    } 
    return (-b - f32::sqrt(discriminant))/(2.0*a);
}

fn ray_color(given_ray: Ray) -> Point3 {
    let t: f32 =  hit_sphere(Point3{x:0.0, y:0.0, z:-1.0}, 0.5, &given_ray);
    if t > 0.0 {
        let n: Point3 = unit_vector(given_ray.at(t)) - Point3{x:0.0, y:0.0, z:1.0};
        return Point3{x:n.x+1.0, y:n.y+1.0, z:n.z+1.0}*0.5;
    }

    // Lerp between blue and white
    let unit_direction: Point3 = unit_vector(given_ray.direction);
    let a: f32 = 0.5*(unit_direction.y + 1.0);
    return Point3{x: 1.0, y: 1.0, z: 1.0}*(1.0 - a) + Point3{x: 0.5, y: 0.7, z: 1.0}*a;
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const IMAGE_WIDTH: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height: u32 = cmp::max(1, (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32);

    // Camera

    const FOCAL_LENGTH: f32 = 1.0;
    const VIEWPORT_HEIGHT: f32  = 2.0;
    let viewport_width: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32/image_height as f32);
    const CAMERA_CENTER: Point3 =Point3{x: 0.0, y: 0.0, z: 0.0};
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Point3 = Point3{x:viewport_width, y:0.0, z:0.0};
    let viewport_v: Point3 = Point3{x:0.0, y:-VIEWPORT_HEIGHT, z:0.0};

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Point3 = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v: Point3 = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Point3 = CAMERA_CENTER - Point3{x:0.0, y:0.0, z:FOCAL_LENGTH} - viewport_u*0.5f32 - viewport_v*0.5f32;
    let pixel00_loc: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v)*0.5f32;

    // Render
    let image: File = File::create("image.ppm").unwrap();
    let mut image_buffer: BufWriter<File> = BufWriter::new(image);

    image_buffer.write_all(&format!("P3\n{IMAGE_WIDTH} {image_height}\n255\n").as_bytes()).unwrap();

    for j in 0..image_height {
        // https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
        print!("\r                         ");
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center: Point3 = pixel00_loc + (pixel_delta_u * i as f32 ) + (pixel_delta_v * j as f32);
            let ray_direction: Point3 = pixel_center - CAMERA_CENTER;
            let current_ray: Ray = Ray{origin: CAMERA_CENTER, direction: ray_direction};

            let pixel_color: Point3 = ray_color(current_ray);

            write_color(&mut image_buffer, pixel_color);
        }
    }
    println!("\nDone!");

    image_buffer.flush().unwrap();
}
