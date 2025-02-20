use std::ops::{Add, Sub, Mul, Div};

// Name is point3 to avoid warning with similarly named vec
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
// Addition of two vectors
impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
// Subtraction of two vectors
impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{x: self.x - other.x,
                y: self.y - other.y, 
                z: self.z - other.z
        }
    }
}
// Multiplication by scalar
impl Mul<f32> for Point3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self{x: self.x * other,
                y: self.y * other,
                z: self.z * other,
        }
    }
}
// Division by scalar
impl Div<f32> for Point3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        self * (1.0/other)
    }
}

// Geometry things

// Length squared
impl Point3 {
    fn length_squared(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}
// Length
impl Point3 {
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}
// Dot product
fn dot(u: Point3, v: Point3) -> f32 {
    u.x*v.x + u.y*v.y + u.z*v.z
}
// Cross product
fn cross(u: Point3, v: Point3) -> Point3 {
    Point3{x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
    }
}

pub mod color;