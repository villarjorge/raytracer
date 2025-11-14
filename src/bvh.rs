use std::cmp::Ordering;
use std::ops::Range;

use crate::aabb::join_aabbs;
use crate::hittable_list::HittableList;
use crate::{aabb::AABB, hittable::Hittable};
use crate::ray::Ray;
use crate::hittable::HitResult;

pub enum BVHNode {
    Leaf {
        // To do: change this into a list of objects. This will make the tree less deep and improve performance
        // Would it be possible to reuse HittableList for this?
        object: Box<dyn Hittable>,
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
        if !self.bounding_box().hit(ray, &ray_t) {
            return HitResult::DidNotHit;
        }
        
        // The right interval needs to be narrowed to prevent problems with occlusion
        // To do: refactor to remove nested match structure (add aditional function?)
        match self {
            BVHNode::Leaf { object, bounding_box: _ } => { return object.hit(ray, ray_t.clone()); },
            BVHNode::Internal { left, right, bounding_box: _ } => {
                match left.hit(ray, ray_t.clone()) {
                    HitResult::DidNotHit => { 
                        match right.hit(ray, ray_t) {
                            HitResult::DidNotHit => {return HitResult::DidNotHit;},
                            HitResult::HitRecord(hit_record) => {return HitResult::HitRecord(hit_record);}
                        }
                     },
                    HitResult::HitRecord(hit_record) => {
                        match right.hit(ray, ray_t.start..hit_record.t) {
                            HitResult::DidNotHit => {return HitResult::HitRecord(hit_record);},
                            HitResult::HitRecord(hit_record) => {return HitResult::HitRecord(hit_record);}
                        }
                    }
                }
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            BVHNode::Leaf { object: _, bounding_box } => bounding_box,
            BVHNode::Internal { left: _, right: _, bounding_box } => bounding_box,
        }
    }
}

pub fn create_bvh_node_from_hittable_list(list: HittableList) -> BVHNode {
    create_bvh_node(list.objects)
}

// To do: deal with a reference to a pointer
// To do: Is there a way to derive comparisons for bounding boxes?
fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: u64) -> Ordering {
    let a_axis_interval: &Range<f64> = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval: &Range<f64> = b.bounding_box().axis_interval(axis_index);

    return a_axis_interval.start.total_cmp(&b_axis_interval.start);
}

fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    return box_compare(a, b, 0);
}

fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    return box_compare(a, b, 1);
}

fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    return box_compare(a, b, 2);
}

pub fn create_bvh_node(mut objects: Vec<Box<dyn Hittable>>) -> BVHNode {
    let axis: u64 = rand::random_range(0..=2);
    
    let current_box_compare: fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering = {
        if axis == 0 { box_x_compare }
        else if axis == 1 { box_y_compare }
        else if axis == 2 { box_z_compare }
        else {panic!()}
    }; 

    let left: Box<dyn Hittable>;
    let right: Box<dyn Hittable>;

    if objects.len() == 1 {
        // To do: consider using a double end queue instead of a vector
        let object: Box<dyn Hittable> = objects.remove(0);
        let bounding_box: AABB = object.bounding_box().clone();
        return BVHNode::Leaf { object, bounding_box };
    } else if objects.len() == 2 {
        left = objects.remove(0);
        right = objects.remove(1);
    } else {
        objects.sort_by(|arg0: &Box<dyn Hittable + 'static>, arg1: &Box<dyn Hittable + 'static>| current_box_compare(arg0, arg1));

        let mid: usize = objects.len()/2;
        left = Box::new(create_bvh_node(objects.split_off(mid)));
        right = Box::new(create_bvh_node(objects));    
    }

    // To do: deal with taking a reference of a dereferenced pointer
    let bounding_box: AABB = join_aabbs(&*left.bounding_box(), &*right.bounding_box());

    BVHNode::Internal { left, right, bounding_box }
}