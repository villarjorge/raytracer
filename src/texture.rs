use std::rc::Rc;

use image::{ImageBuffer, Rgb, open};

use crate::{hittable::SurfaceCoordinate, perlin::PerlinNoise, point3::{Point3, color::Color}};

pub trait Texture {
    fn value(&self, surface_coords: SurfaceCoordinate, p: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    pub fn new(color: Point3) -> Rc<SolidColor> {
        Rc::new(SolidColor { albedo: color })
    }
}

impl Texture for SolidColor {
    fn value(&self, _surface_coords: SurfaceCoordinate, _p: &Point3) -> Point3 {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inverse_scale: f64,
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn from_pointers(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Rc<CheckerTexture> {
        Rc::new(CheckerTexture { inverse_scale: 1.0/scale, even, odd })
    }

    pub fn from_colors(scale: f64, even: Point3, odd: Point3) -> Rc<CheckerTexture> {
        Rc::new(CheckerTexture { inverse_scale: 1.0/scale, even: SolidColor::new(even), odd: SolidColor::new(odd) })
    }
}

impl Texture for CheckerTexture {
    fn value(&self, surface_coords: SurfaceCoordinate, p: &Point3) -> Point3 {
        let x_integer: i64 = (self.inverse_scale * p.x).floor() as i64;
        let y_integer: i64 = (self.inverse_scale * p.y).floor() as i64;
        let z_integer: i64 = (self.inverse_scale * p.z).floor() as i64;

        let is_even: bool = (x_integer + y_integer + z_integer) % 2 == 0;

        // To do: A texture needs to know around what surface it is warped in order to map properly
        // To properly map into spheres
        // let u_integer: i64 = (self.inverse_scale * surface_coords.u).floor() as i64;
        // let v_integer: i64 = (self.inverse_scale * surface_coords.v).floor() as i64;

        // let is_even: bool = (u_integer + v_integer) % 2 == 0;

        if is_even {
            self.even.value(surface_coords, p)
        } else {
            self.odd.value(surface_coords, p)
        }
    }
}

/// An image texture, build on the image crate. Create with create_image_texture(). 
/// If the path gives an error, load an error texture that is easy to see
pub struct ImageTexture {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl ImageTexture {
    pub fn new(path: &str) -> Rc<dyn Texture> {
        match open(path) {
            Ok(image) => Rc::new(ImageTexture { image: image.into_rgb8() }),
            Err(image_error) => {
                eprintln!("Could not load the image texture. Falling back to default. Error:");
                eprintln!("{}", image_error);

                CheckerTexture::from_colors(
                    2.0,
                    Point3 { x: 1.0, y: 0.0, z: 0.862745098039 },
                    Point3 { x: 0.00392156862745, y: 0.0, z: 0.00392156862745 }
                )
            },
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, surface_coords: SurfaceCoordinate, _p: &Point3) -> Point3 {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let clamped_surface_coords: SurfaceCoordinate = SurfaceCoordinate {
            u: surface_coords.u.clamp(0.0, 1.0),
            v: 1.0 - surface_coords.v.clamp(0.0, 1.0) // Flip v to image coordinates
        };

        let u_integer: u32 = (self.image.width() as f64 * clamped_surface_coords.u) as u32;
        let v_integer: u32 = (self.image.height() as f64 * clamped_surface_coords.v) as u32;
        // https://docs.rs/image/0.25.9/image/struct.ImageBuffer.html#method.get_pixel
        let texture_pixel: &Rgb<u8> = self.image.get_pixel(u_integer, v_integer);

        let color_scale: f64 = 1.0/255.0;
        Point3 { x: (texture_pixel.0[0] as f64)*color_scale, y: (texture_pixel.0[1] as f64)*color_scale, z: (texture_pixel.0[2] as f64)*color_scale }
    }
}

pub struct PerlinNoiseTexture {
    pub perlin_noise: PerlinNoise,
    pub scale: f64
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _surface_coords: SurfaceCoordinate, p: &Point3) -> Point3 {
        // To do: this point is dereferenced so it can be multiplied. Improve?
        // Basic perlin noise
        // let p: Point3 = *p*self.scale;
        // Point3 { x: 1.0, y: 1.0, z: 1.0 } * 0.5 * (1.0 + self.perlin_noise.noise(&p))
        // Turbulent perlin noise
        // Point3 { x: 1.0, y: 1.0, z: 1.0 } * self.perlin_noise.turbulence(&p, 7)
        // Turbulent perlin noise modulated with a sine
        Point3 { x: 0.5, y: 0.5, z: 0.5 } * (1.0 + (self.scale * p.z + 10.0 * self.perlin_noise.turbulence(p, 7)).sin())
    }
}