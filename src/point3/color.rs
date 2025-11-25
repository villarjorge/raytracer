
use super::Point3;
use std::{fs::File, io::{BufWriter, Write}};

// Define this alias
pub type Color = Point3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(out_buffer: &mut BufWriter<File>, pixel_color: Color) {
    let r: f64 = pixel_color.x;
    let g: f64 = pixel_color.y;
    let b: f64 = pixel_color.z;

    // Apply a linear to gamma transform for gamma 2
    let g: f64 = linear_to_gamma(g);
    let b: f64 = linear_to_gamma(b);
    let r: f64 = linear_to_gamma(r);

    // Translate the [0,1] component values to the byte range [0,255].

    let min: f64 = 0.000;
    let max: f64 = 0.999;
    let rbyte: u8 = (256.00 * r.clamp(min, max)) as u8;
    let gbyte: u8 = (256.00 * g.clamp(min, max)) as u8;
    let bbyte: u8 = (256.00 * b.clamp(min, max)) as u8;

    // Write out the pixel color components.
    out_buffer.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes()).unwrap();
}