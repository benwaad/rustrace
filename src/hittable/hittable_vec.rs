use super::*;
use crate::math::Interval;
use crate::ray::*;
use std::rc::Rc;
use std::vec::Vec;

pub struct HittableVec<T: Hittable> {
    objects: Vec<Rc<T>>,
}

impl<T: Hittable> HittableVec<T> {
    pub fn new() -> Self {
        HittableVec {
            objects: Vec::new(),
        }
    }
    pub fn with_one(object: T) -> Self {
        HittableVec {
            objects: vec![Rc::new(object)],
        }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(Rc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T: Hittable> Hittable for HittableVec<T> {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let (tmin, tmax) = (interval.min, interval.max);
        let mut hit_rec: Option<HitRecord> = None;
        let mut closest_so_far = tmax;

        for elem in &self.objects {
            if let Some(elem_hit) = elem.hit(ray, Interval::new(tmin, closest_so_far)) {
                closest_so_far = elem_hit.t;
                hit_rec = Some(elem_hit);
            }
        }

        hit_rec
    }
}
