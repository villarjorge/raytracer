
use super::Point3;
use std::{fs::File, io::{BufWriter, Write}};

// To do: Find and import rgb function. It is integrated in vscode and it lets you easily see the color
//const RED = rgb(243, 27, 11);

// To do: Make colors physical. Transfrom colors to frequencies for calculations and then transform them back to rgb for printing
// Not trivial to do since the sum of two waves does not have a singular frequency. https://www.tandfonline.com/doi/abs/10.1080/10867651.1999.10487511

// Define this alias
pub type Color = Point3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

/// Process the color from linear space rgb to u8 rgb by tranforming from linear to gamma, clamping it to the 0..1 range and mutipling it by 256. 
pub fn proccess_color(pixel_color: Point3) -> [u8; 3] {
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
    [rbyte, gbyte, bbyte]
}

pub fn write_color(out_buffer: &mut BufWriter<File>, pixel_color: Color) {
    let arr: [u8; 3] = proccess_color(pixel_color);
    let rbyte: u8 = arr[0];
    let gbyte: u8 = arr[1]; 
    let bbyte: u8 = arr[2]; 

    // Write out the pixel color components.
    out_buffer.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes()).unwrap();
}

