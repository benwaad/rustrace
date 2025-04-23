use crate::material::*;
use crate::math::*;
use crate::ray::*;

use std::option::Option;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a MaterialKind,
}

impl<'a> HitRecord<'a> {
    fn set_face_normal(&mut self, dir: &Vec3, outward_normal: &Vec3) {
        self.front_face = dot(dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn new(
        t: f64,
        p: &Vec3,
        dir: &Vec3,
        outward_normal: &Vec3,
        material: &'a MaterialKind,
    ) -> Self {
        let mut rec = HitRecord {
            p: *p,
            t,
            normal: *outward_normal,
            front_face: true,
            material,
        };
        rec.set_face_normal(dir, outward_normal);
        rec
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
