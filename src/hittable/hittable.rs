use crate::math::*;
use crate::ray::*;

use std::option::Option;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, dir: &Vec3, outward_normal: &Vec3) {
        self.front_face = dot(*dir, *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn from_ray(t: f64, p: &Vec3, dir: &Vec3, outward_normal: &Vec3) -> Self {
        let mut rec = HitRecord {
            p: *p,
            t,
            normal: *outward_normal,
            front_face: true,
        };
        rec.set_face_normal(dir, outward_normal);
        rec
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
