use crate::{Point3, point3::Vector3};

/// Ray is a geometrical line: \vec{A} + \vec{B}t
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction*t
    }
}