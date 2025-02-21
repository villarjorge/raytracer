use crate::Point3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction*t
    }
}