use std::ops::{Index, Range};

use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Clone)]
pub struct AABB {
    x: Range<f64>, 
    y: Range<f64>, 
    z: Range<f64>, 
}

impl Default for AABB {
    fn default() -> Self {
        create_aabb_from_points(Point3::default(), Point3::default())
    }
}

// https://doc.rust-lang.org/std/ops/trait.Index.html
impl Index<u8> for AABB {
    type Output = Range<f64>;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => { &self.x },
            1 => { &self.y },
            2 => { &self.z },
            _ => {panic!()}
        }
    }
}

pub fn create_aabb_from_points(a: Point3, b: Point3) -> AABB {
    let x: Range<f64> = if a.x <= b.x {a.x..b.x} else {b.x..a.x};
    let y: Range<f64> = if a.y <= b.y {a.y..b.y} else {b.y..a.y};
    let z: Range<f64> = if a.z <= b.z {a.z..b.z} else {b.z..a.z};

    AABB {x, y, z}
}

fn unite_ranges(r1: &Range<f64>, r2: &Range<f64>) -> Range<f64> {
    r1.start.min(r2.start)..r1.end.max(r2.end)
}

pub fn join_aabbs(bounding_box0: &AABB, bounding_box1: &AABB) -> AABB {
    // Joins together two bounding boxes to create a new box that encompases the two
    let x: Range<f64> = unite_ranges(&bounding_box0.x, &bounding_box1.x);
    let y: Range<f64> = unite_ranges(&bounding_box0.y, &bounding_box1.y);
    let z: Range<f64> = unite_ranges(&bounding_box0.z, &bounding_box1.z);

    AABB {x, y, z}
}

// fn check_axis(ray_t: &Range<f64>, axis: &Range<f64>, inverse_coord: f64, origin_coord: f64) -> bool {
//     let t0: f64 = (axis.start - origin_coord)*inverse_coord;
//     let t1: f64 = (axis.end - origin_coord)*inverse_coord;

//     let mut start: f64 = ray_t.start;
//     let mut end: f64 = ray_t.end;

//     if t0 < t1 {
//         if t0 > ray_t.start { start = t0; }
//         if t1 < ray_t.end { end = t1; }
//     } else {
//         if t1 > ray_t.start { start = t1; }
//         if t0 < ray_t.end { end = t0; }
//     }
//     end <= start
// }

impl AABB {
    pub fn hit(&self, ray: &Ray, ray_t: &mut Range<f64>) -> bool {
        // A bounding box is simpler than an object, we only care if the bounding box is hit or not
        // This 2nd hottest part of the code, taking 31.6% of CPU time
        let ray_origin: Point3 = ray.origin;
        let ray_direction: Point3 = ray.direction;

        for axis_index in 0_u8..3 {
            let axis: &Range<f64> = &self[axis_index];
            let inverse_coord: f64 = 1.0/ray_direction[axis_index];
            // let origin_coord: f64 = ray_origin[axis_index];

            let t0: f64 = (axis.start - ray_origin[axis_index])*inverse_coord;
            let t1: f64 = (axis.end - ray_origin[axis_index])*inverse_coord;

            if t0 < t1 {
                if t0 > ray_t.start { ray_t.start = t0; }
                if t1 < ray_t.end { ray_t.end = t1; }
            } else {
                if t1 > ray_t.start { ray_t.start = t1; }
                if t0 < ray_t.end { ray_t.end = t0; }
            }
            if ray_t.end <= ray_t.start { return false; }
        }

        true
    }

    // To do: change this axis interval to a impl of the Index trait https://doc.rust-lang.org/std/ops/trait.Index.html
    pub fn axis_interval(&self, n: u8) -> &Range<f64> {
        if n == 0 {
            &self.x
        } else if n == 1 {
            &self.y
        } else {
            &self.z
        }
    }

    pub fn longest_axis(&self) -> u8 {
        // Returns the idex of the longest axis of the bounding box
        let x_size: f64 = self.x.end - self.x.start;
        let y_size: f64 = self.y.end - self.y.start;
        let z_size: f64 = self.z.end - self.z.start;

        if x_size > y_size && x_size > z_size { 0}
        else if y_size > x_size && y_size > z_size { 1}
        else { 2}
    }
}

