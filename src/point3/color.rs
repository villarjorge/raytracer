
use super::Point3;
use std::{fs::File, io::{BufWriter, Write}};

pub fn write_color(out_buffer: &mut BufWriter<File>, pixel_color: Point3) {
    let r: f32 = pixel_color.x;
    let g: f32 = pixel_color.y;
    let b: f32 = pixel_color.z;

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte: u8 = (255.999 * r) as u8;
    let gbyte: u8 = (255.999 * g) as u8;
    let bbyte: u8 = (255.999 * b) as u8;

    // Write out the pixel color components.
    out_buffer.write_all(&format!("{rbyte} {gbyte} {bbyte}\n").as_bytes()).unwrap();
}