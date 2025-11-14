use std::cmp::Ordering;
use std::ops::Range;

use crate::aabb::join_aabbs;
use crate::hittable_list::{HittableList};
use crate::{aabb::AABB, hittable::Hittable};
use crate::ray::Ray;
use crate::hittable::{HitResult};

pub enum BVHNode {
    Leaf {
        // To do: change this into a list of objects. This will make the tree less deep and improve performance
        // Would it be possible to reuse HittableList for this?
        objects: HittableList,
        bounding_box: AABB
    },
    Internal {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        bounding_box: AABB
    }
}

impl Hittable for BVHNode {
    fn hit(&'_  self, ray: &Ray, ray_t: Range<f64>) -> HitResult<'_> {
        // This is the 3rd hottest part of the code, taking 25.5% of CPU time
        let mut reduced_ray_t: Range<f64> = ray_t.clone();

        if !self.bounding_box().hit(ray, &mut reduced_ray_t) {
            return HitResult::DidNotHit;
        }

        let ray_t: Range<f64> = reduced_ray_t;
        
        // The right interval needs to be narrowed to prevent problems with occlusion
        // To do: refactor to remove nested match structure (add aditional function?)
        match self {
            BVHNode::Leaf { objects, bounding_box: _ } => { objects.hit(ray, ray_t.clone()) },
            BVHNode::Internal { left, right, bounding_box: _ } => {
                match left.hit(ray, ray_t.clone()) {
                    HitResult::DidNotHit => { 
                        match right.hit(ray, ray_t) {
                            HitResult::DidNotHit => {HitResult::DidNotHit},
                            HitResult::HitRecord(hit_record) => {HitResult::HitRecord(hit_record)}
                        }
                     },
                    HitResult::HitRecord(hit_record) => {
                        match right.hit(ray, ray_t.start..hit_record.t) {
                            HitResult::DidNotHit => {HitResult::HitRecord(hit_record)},
                            HitResult::HitRecord(hit_record) => {HitResult::HitRecord(hit_record)}
                        }
                    }
                }
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            BVHNode::Leaf { objects: _, bounding_box } => bounding_box,
            BVHNode::Internal { left: _, right: _, bounding_box } => bounding_box,
        }
    }
}

pub fn create_bvh_node_from_hittable_list(list: HittableList) -> BVHNode {
    create_bvh_node(list.objects)
}

// To do: deal with a reference to a pointer
// To do: Is there a way to derive comparisons for bounding boxes?
fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: u8) -> Ordering {
    let a_axis_interval: &Range<f64> = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval: &Range<f64> = b.bounding_box().axis_interval(axis_index);

    a_axis_interval.start.total_cmp(&b_axis_interval.start)
}

fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

pub fn create_bvh_node(mut objects: Vec<Box<dyn Hittable>>) -> BVHNode {
    let mut bounding_box: AABB = AABB::default();

    for object in &objects {
        bounding_box = join_aabbs(&bounding_box, object.bounding_box())
    }

    let axis = bounding_box.longest_axis();

    let current_box_compare: fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering = {
        if axis == 0 { box_x_compare }
        else if axis == 1 { box_y_compare }
        else if axis == 2 { box_z_compare }
        else {panic!()}
    };

    // To do: optimize this threshold for performance
    const THRESHOLD: usize = 4;
    
    // To do: consider using a double end queue instead of a vector
    if objects.len() <= THRESHOLD {
        let mut hittable_list: HittableList = HittableList::default();

        for element in objects.drain(0..THRESHOLD.min(objects.len())) {
            hittable_list.add_pointer(element);
        }

        bounding_box = hittable_list.bounding_box().clone();

        return BVHNode::Leaf { objects: hittable_list, bounding_box};
    } else {
        objects.sort_by(|arg0: &Box<dyn Hittable + 'static>, arg1: &Box<dyn Hittable + 'static>| current_box_compare(arg0, arg1));

        let mid: usize = objects.len()/2;

        let left: Box<dyn Hittable> = Box::new(create_bvh_node(objects.split_off(mid)));
        let right: Box<dyn Hittable> = Box::new(create_bvh_node(objects));

        return BVHNode::Internal { left, right, bounding_box };
    }    
}