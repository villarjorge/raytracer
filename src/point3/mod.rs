use std::ops::{Add, Sub, Mul, Div};

// ToDo: Right now you can only multiply point*f32 (with the float on the left). There has to be a more general way to implement multiplication

// Name is point3 to avoid warning with similarly named vec
#[derive(Clone, Copy)]
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

impl Point3 {
    // Length squared
    fn length_squared(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    // Length
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    // Dot product
    fn dot(&self, other: Self) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    // Cross product
    fn cross(&self, other: Self) -> Point3 {
        Point3{x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x
        }
    }
}

// Unit vector
pub fn  unit_vector(u: Point3) -> Point3 {
    u*(1.0/u.length())
}

pub mod color;