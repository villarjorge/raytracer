use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use rand;

// Lots of boiler plate here
// Should I implement all the traits again for references?

// Name is point3 to avoid warning with similarly named vec
// To do: Remove this derive to sort out references and borrows
#[derive(Clone, Copy, PartialEq, Debug)]

/// A struct representing a point in three dimensional flat space with orthonormal cartesian coordinates. 
/// There are two aliases for these struct: Vector3 and Color. Point3 supports addition, subtraction, negation, indexing, 
/// multiplication by scalar (left and right), division by scalar and coordinate-wise multiplication. 
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Alias for a three dimensional vector
pub type Vector3 = Point3;

/// Returns a new point from an array of three f64
pub fn point_from_array(array: [f64; 3]) -> Point3 {
    Point3 { x: array[0], y: array[1], z: array[2] }
}

impl Default for Point3 {
    /// The default is {0.0, 0.0, 0.0}
    fn default() -> Self {
        Point3 { x: 0.0f64, y: 0.0f64, z: 0.0f64 }
    }
}

impl Index<u8> for Point3 {
    /// The indices (0, 1, 2) correspond to (x, y, z)
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

impl Neg for Point3 {
    /// Negation
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z // This was -self.y !!
        }
    }
}

impl Add for Point3 {
    /// Addition of two vectors
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point3 {
    /// Subtraction of two vectors
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self{
            x: self.x - other.x,
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }
}

impl Mul<f64> for Point3 {
    /// Multiplication of Point3 by f64 scalar: Point3*f64
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Point3 {
    /// Division by scalar
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        self * (1.0/other)
    }
}

impl Mul<Point3> for f64 {
    /// Multiplication of f64 by Point: f64*Point3
    type Output = Point3;

    fn mul(self, other: Point3) -> Self::Output {
        Point3{
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<Point3> for Point3 {
    /// Coordinate-wise multiplication of points
    type Output = Self;

    fn mul(self, other: Point3) -> Self::Output {
        Self{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

/// Geometrical functions: length, dot product and cross product
impl Point3 {
    /// Length squared
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    /// Length
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    /// Dot product as a method point
    pub fn dot(&self, other: Self) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

/// Returns the dot product of to points
pub fn dot(u: &Point3, v: &Point3) -> f64 {
    u.x*v.x + u.y*v.y + u.z*v.z
}

/// Returns the cross product of two points
pub fn cross(u: &Point3, v: &Point3) -> Point3 {
    Point3{
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x
    }
}

/// Returns a rotation of p on the y axis given the sine and cosine of the angle
pub fn rotate_y(p: &Point3, cos_theta: f64, sin_theta: f64) -> Point3 {
    Point3 { x: cos_theta*p.x - sin_theta*p.z, y: p.y, z: sin_theta*p.x + cos_theta*p.z }
}

impl Point3 {
    /// Returns true if the vector is close to zero in all dimensions false otherwise. 
    /// The function does a tolerance check with a tolerance of 1e-8 
    pub fn is_near_zero(&self) -> bool {
        const TOLERANCE: f64 = 1e-8;
        self.x.abs() < TOLERANCE && self.y.abs() < TOLERANCE && self.z.abs() < TOLERANCE
    } 
}

/// Returns the unit vector of u
pub fn unit_vector(u: Point3) -> Point3 {
    u/u.length()
}

// Functions for random vectors
// To do: initialize a local rng handle for better performance

/// Returns a random vector whose entries are between a and b
pub fn random_vector(a: f64, b: f64) -> Point3 {
    Point3 { x: rand::random_range(a..b), y: rand::random_range(a..b), z: rand::random_range(a..b) }
}

/// Returns a random unit vector bounded by the unit sphere. This function uses a rejection method to find such vector, which uses looping
pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let p: Point3 = random_vector(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    };
}

/// Returns a random unit vector
pub fn random_unit_vector() -> Point3 {
    unit_vector(random_in_unit_sphere())
}

/// Returns a random unit vector in the same hemisphere as the given normal vector. 
/// The normal defines a plane. The generated vector will be on the same side of the plane as the normal
pub fn random_on_hemisphere(normal: &Point3) -> Point3 {
    let on_unit_sphere: Point3 = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    }
    -on_unit_sphere
}

/// Returns a random vector bounded by a unit disk in the x-y plane. This function uses a rejection method to find such vector.
pub fn random_in_unit_disk() -> Point3 {
    loop {
        let p: Point3 = Point3{ x: rand::random_range(-1.0..1.0), y: rand::random_range(-1.0..1.0), z: 0.0 };
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

// "Optical" vector operations

/// Computes the reflection of v across the normal n
pub fn reflect(v: Point3, n: Point3) -> Point3{
    v - 2.0*dot(&v, &n)*n
}

/// Calculates the refraction of uv given a normal n  and the ratio of the refractive indexes. Both uv and n must be unit vectors
pub fn refract(uv: Point3, n: Point3, eta_inital_over_eta_final: f64) -> Point3 {
    let cos_theta: f64 = dot(&(-uv), &n).min(1.0);
    let perpendicular_component: Point3 = (uv + n*cos_theta)*eta_inital_over_eta_final;
    let parallel_component: Point3 = (-n)*((1.0 - perpendicular_component.length_squared()).abs().sqrt());

    perpendicular_component + parallel_component
}

pub mod color;