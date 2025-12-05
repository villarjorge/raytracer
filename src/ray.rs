use crate::{Point3, point3::Vector3};

/// Ray is a geometrical line: \vec{A} + \vec{B}t
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub inverse_direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction*t
    }
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction, inverse_direction: Point3::new(1.0/direction.x, 1.0/direction.y, 1.0/direction.z) }
        // Ray { origin, direction }
    }
}