use std::{fs::File, io::Write};

fn main() {
    // Image
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    // Render
    let mut file: File = File::create("image.ppm").unwrap();

    file.write_all(&format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes()).unwrap();

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            // r, g and b must be floats between 0 and 1
            let r: f32 = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let g: f32 = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let b: f32 = 0.0;

            // Translate the [0,1] component values to the byte range [0,255].
            let rbyte: u8 = (255.999 * r) as u8;
            let gbyte: u8 = (255.999 * g) as u8;
            let bbyte: u8 = (255.999 * b) as u8;

            file.write_all(&format!("{rbyte} {gbyte} {bbyte}\n").as_bytes()).unwrap();
        }
    }
}
