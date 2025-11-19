use std::rc::Rc;

use crate::point3::{Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Point3;
}

pub struct SolidColor {
    albedo: Point3
}

pub fn create_solid_color(color: Point3) -> Rc<SolidColor> {
    return Rc::new(SolidColor { albedo: color });
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Point3 {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inverse_scale: f64,
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>
}

pub fn create_checker_texture_from_pointers(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Rc<CheckerTexture> {
    Rc::new(CheckerTexture { inverse_scale: 1.0/scale, even, odd })
}

pub fn create_checker_texture_from_colors(scale: f64, even: Point3, odd: Point3) -> Rc<CheckerTexture> {
    Rc::new(CheckerTexture { inverse_scale: 1.0/scale, even: create_solid_color(even), odd: create_solid_color(odd) })
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Point3 {
        let x_integer: i64 = (self.inverse_scale * p.x).floor() as i64;
        let y_integer: i64 = (self.inverse_scale * p.y).floor() as i64;
        let z_integer: i64 = (self.inverse_scale * p.z).floor() as i64;

        let is_even: bool = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            return self.even.value(u, v, p);
        } else {
            return self.odd.value(u, v, p);
        }
    }
}