use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use rand;

// Lots of boiler plate here
// Should I implement all the traits again for references?

// Name is point3 to avoid warning with similarly named vec
#[derive(Clone, Copy)] // once I get the project going, remove this to sort out references etc
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Point3 {
    fn default() -> Self {
        // Defautl is {0.0, 0.0, 0.0}
        Point3 { x: 0.0f64, y: 0.0f64, z: 0.0f64 }
    }
}

impl Index<u8> for Point3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => { &self.x },
            1 => { &self.y },
            2 => { &self.z },
            _ => {panic!()}
        }
    }
}

// Addition of two vectors
impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
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

    fn sub(self, other: Self) -> Self::Output {
        Self{
            x: self.x - other.x,
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }
}
// Multiplication by scalar
impl Mul<f64> for Point3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Mul<Point3> for f64 {
    type Output = Point3;
    fn mul(self, other: Point3) -> Self::Output {
        Point3{
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

// Multiplication between vectors
impl Mul<Point3> for Point3 {
    type Output = Self;

    fn mul(self, other: Point3) -> Self::Output {
        Self{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Negation
impl Neg for Point3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z // This was -self.y !!
        }
    }
}

// Division by scalar
impl Div<f64> for Point3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        self * (1.0/other)
    }
}

// Geometrical functions: lenght, dot product and cross product

impl Point3 {
    // Length squared
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    // Length
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    // Dot product
    pub fn dot(&self, other: Self) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}
pub fn dot(u: &Point3, v: &Point3) -> f64 {
    // Dot product
    u.x*v.x + u.y*v.y + u.z*v.z
}
pub fn cross(u: &Point3, v: &Point3) -> Point3 {
    // Cross product
    Point3{x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
    }
}

impl Point3 {
    pub fn is_near_zero(&self) -> bool {
        /// Returns zero if the vector is close to zero in all dimensions
        const TOLERANCE: f64 = 1e-8;
        self.x.abs() < TOLERANCE && self.y.abs() < TOLERANCE && self.z.abs() < TOLERANCE
    } 
}

// Unit vector
pub fn unit_vector(u: Point3) -> Point3 {
    // Returns the unit vector of u
    u/u.length()
}

// Functions for random vectors
// To do: initialize a local rng handle for better performance

pub fn random_vector(a: f64, b: f64) -> Point3 {
    Point3 { x: rand::random_range(a..b), y: rand::random_range(a..b), z: rand::random_range(a..b) }
}

pub fn random_in_unit_sphere() -> Point3 {
    // Random unit vector bounded by the unit sphere
    loop {
        let p: Point3 = random_vector(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    };
}

pub fn random_unit_vector() -> Point3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: &Point3) -> Point3 {
    // A random unit vector in the same hemisphere as the given normal vector. 
    // The normal defines a plane. The generated vector will be on the same side of the plane as the normal
    let on_unit_sphere: Point3 = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    }
    -on_unit_sphere
}

pub fn random_in_unit_disk() -> Point3 {
    loop {
        let p: Point3 = Point3{ x: rand::random_range(-1.0..1.0), y: rand::random_range(-1.0..1.0), z: 0.0 };
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

// "Optical" vector operations

pub fn reflect(v: Point3, n: Point3) -> Point3{
    // Computes the reflection of v across the normal n
    v - 2.0*dot(&v, &n)*n
}

pub fn refract(uv: Point3, n: Point3, eta_inital_over_eta_final: f64) -> Point3 {
    // Calculates the refraction of uv given a normal n (both unit vectors) given the ratio of the refractive indexes 
    let cos_theta: f64 = dot(&(-uv), &n).min(1.0);
    let perpendicular_component: Point3 = (uv + n*cos_theta)*eta_inital_over_eta_final;
    let parallel_component: Point3 = (-n)*((1.0 - perpendicular_component.length_squared()).abs().sqrt());

    perpendicular_component + parallel_component
}

pub mod color;