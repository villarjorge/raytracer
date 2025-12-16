use std::ops::Range;
use std::sync::Arc;

use crate::aabb::{AABB, join_aabbs};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
/// A vector of pointers to objects that implement the hittable trait.
pub struct HittableList {
    // HittableList is a list of objects with the hittable trait.
    // The objects can be of diferent sizes, so it is necesary to use a reference or a pointer.
    // Using a pointer leads to dealing with less lifetimes, but to sort a type Vec<Box<dyn Hittable>> you need to
    // take references of Box<dyn Hittable>, which is discouraged. The ownership is more clear, since Vec owns Box which owns the Hittable
    // Using references leads to more lifetimes. It also leads to worse ownership, since the vec does not own anything.
    // I could not make it work with references
    // Idealy all objects would be stored contiguously on the heap to make performance better. One way to do this would be to
    // have vectors for each primitive, and then passing references to hittable list.
    // this does not need to be a vector, it is just like this to make initialization easier
    pub objects: Vec<Arc<dyn Hittable>>,
    pub bounding_box: AABB,
}

impl HittableList {
    pub fn clear(mut self) {
        self.objects.clear()
    }
    /// Extend the list's bounding box by the object's and add the object, creating a pointer first
    pub fn add<T: Hittable + 'static>(&mut self, to_add: T) {
        self.bounding_box = join_aabbs(&self.bounding_box, to_add.bounding_box());
        self.objects.push(Arc::new(to_add));
    }
    /// Extend the list's bounding box by the object's and add the pointer to the object
    pub fn add_pointer(&mut self, to_add: Arc<dyn Hittable>) {
        self.bounding_box = join_aabbs(&self.bounding_box, to_add.bounding_box());
        self.objects.push(to_add);
    }
    pub fn to_hittable_slice(self) -> HittableSlice {
        HittableSlice::from_hittable_list(self)
    }
}

// impl Hittable for HittableList {
//     /// Go through the objects on the vector and compute their hit functions. Keep track of the closest and return that
//     fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
//         let mut temp_record: HitRecord = hit_record.clone();
//         let mut hit_anything: bool = false;
//         let mut closest_so_far: f64 = ray_t.end; // max

//         for object in &self.objects {
//             if object.hit(ray, &(ray_t.start..closest_so_far), &mut temp_record) {
//                 hit_anything = true;
//                 closest_so_far = temp_record.t;
//                 *hit_record = temp_record.clone();
//             }
//         }

//         hit_anything
//     }
//     fn bounding_box(&self) -> &AABB {
//         &self.bounding_box
//     }
// }

/// Similar to HittableList, but with an Arc of a slice so that sync is implemented
pub struct HittableSlice {
    objects: Arc<[Arc<dyn Hittable>]>,
    bounding_box: AABB,
}

impl HittableSlice {
    pub fn from_hittable_list(hittable_list: HittableList) -> HittableSlice {
        HittableSlice {
            objects: Arc::from(hittable_list.objects.as_slice()),
            bounding_box: hittable_list.bounding_box,
        }
    }
}

impl Hittable for HittableSlice {
    /// Go through the objects on the vector and compute their hit functions. Keep track of the closest and return that
    fn hit(&self, ray: &Ray, ray_t: &Range<f64>, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = hit_record.clone();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.end; // max
        // Dereference the outer arc
        for object in &(*self.objects) {
            if object.hit(ray, &(ray_t.start..closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone();
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
