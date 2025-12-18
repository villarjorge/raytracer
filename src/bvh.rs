// use std::cmp::Ordering;
use std::ops::Range;
use std::sync::Arc;

use crate::aabb::join_aabbs;
use crate::hittable::HitRecord;
use crate::hittable::hittable_list::{HittableList, HittableSlice};
use crate::ray::Ray;
use crate::{aabb::AABB, hittable::Hittable};

// To do: consider if this can be replaced by rust's native implementation
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
pub enum BVHNode {
    Leaf {
        objects: HittableSlice,
        bounding_box: AABB,
    },
    Internal {
        // When multithreading is implemented, these Boxes will have to become Arc
        // Or will they? I don't think I need to modify data nor clone pointers on the fly, just access data, so Rc and box should be fine
        left: Arc<dyn Hittable>,
        right: Arc<dyn Hittable>,
        bounding_box: AABB,
    },
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        // This is the 3rd hottest part of the code, taking 25.5% of CPU time
        let mut reduced_ray_t: Range<f64> = ray_t.clone();

        if !self.bounding_box().hit(ray, &mut reduced_ray_t) {
            return false;
        }

        let ray_t: Range<f64> = reduced_ray_t;

        // The right interval needs to be narrowed to prevent problems with occlusion
        match self {
            BVHNode::Leaf {
                objects,
                bounding_box: _,
            } => objects.hit(ray, &ray_t, hit_record),
            BVHNode::Internal {
                left,
                right,
                bounding_box: _,
            } => {
                let hit_left: bool = left.hit(ray, &ray_t, hit_record);
                let hit_right: bool = right.hit(
                    ray,
                    &(ray_t.start..{ if hit_left { hit_record.t } else { ray_t.end } }),
                    hit_record,
                );

                hit_left || hit_right
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            BVHNode::Leaf {
                objects: _,
                bounding_box,
            } => bounding_box,
            BVHNode::Internal {
                left: _,
                right: _,
                bounding_box,
            } => bounding_box,
        }
    }
}

impl BVHNode {
    pub fn from_hittable_list(list: HittableList) -> BVHNode {
        BVHNode::new(list.objects)
    }

    pub fn from_vec(objects: Vec<Arc<dyn Hittable>>) -> BVHNode {
        BVHNode::new(objects)
    }

    // To do: Is there a way to derive comparisons for bounding boxes?
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>) -> BVHNode {
        let mut bounding_box: AABB = AABB::default();

        for object in &objects {
            bounding_box = join_aabbs(&bounding_box, object.bounding_box())
        }
        // choose the longest axis to split
        let axis: u8 = bounding_box.longest_axis();

        // To do: This threshold controls how many objects there are in the leaf nodes. Optimize for performance
        const THRESHOLD: usize = 16;

        if objects.len() <= THRESHOLD {
            let mut hittable_list: HittableList = HittableList::default();

            for element in objects.drain(0..THRESHOLD.min(objects.len())) {
                hittable_list.add_pointer(element);
            }
            // It is not necesary to use the function since thittable_list is not anonimized as dy Hittable
            bounding_box = hittable_list.bounding_box.clone();

            BVHNode::Leaf {
                objects: HittableSlice::from_hittable_list(hittable_list),
                bounding_box,
            }
        } else {
            // Use a clousure here: more idiomatic and much shorter
            // When I changed HittableList from Vec<Box<dyn Hittable>> to Vec<Rc<dyn Hittable> clippy stopped raising this as a borrowed box issue
            objects.sort_by(
                |a: &Arc<dyn Hittable + 'static>, b: &Arc<dyn Hittable + 'static>| {
                    let a_axis_interval: &Range<f64> = a.bounding_box().axis_interval(axis);
                    let b_axis_interval: &Range<f64> = b.bounding_box().axis_interval(axis);

                    a_axis_interval.start.total_cmp(&b_axis_interval.start)
                },
            );

            let mid: usize = objects.len() / 2;

            let left: Arc<dyn Hittable> = Arc::new(BVHNode::new(objects.split_off(mid)));
            let right: Arc<dyn Hittable> = Arc::new(BVHNode::new(objects));

            BVHNode::Internal {
                left,
                right,
                bounding_box,
            }
        }
    }
}
