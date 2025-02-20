use std::{fs::File, io::{BufWriter, Write}};

pub mod point3;

pub use crate::point3::Point3;
pub use crate::point3::color::write_color;

fn main() {
    // Image
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    // Render
    let image: File = File::create("image.ppm").unwrap();
    let mut image_buffer: BufWriter<File> = BufWriter::new(image);

    image_buffer.write_all(&format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes()).unwrap();

    for j in 0..IMAGE_HEIGHT {
        // https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
        print!("\r                         ");
        print!("\rScanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            // r, g and b must be floats between 0 and 1
            let r: f32 = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let g: f32 = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let b: f32 = 0.0;

            let pixel_color: Point3 = Point3{x:r, y:g, z:b};

            write_color(&mut image_buffer, pixel_color);
        }
    }

    image_buffer.flush().unwrap();
}
