use super::HitRecord;
use super::Hittable;
use crate::math::*;
use crate::ray::*;
use std::option::Option;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        // let (tmin, tmax) = (interval.min, interval.max);
        let oc = self.center - ray.orig;
        let a = ray.dir.norm();
        let h = dot(ray.dir, oc);
        let c = oc.norm() - self.radius * self.radius;
        let disc = h * h - a * c;

        if disc < 0.0 {
            return None;
        }

        let sqrtd = disc.sqrt();
        let root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let hitpoint = ray.at(root);
        let outward_normal = (hitpoint - self.center) / self.radius;
        let rec = HitRecord::from_ray(root, &hitpoint, &ray.dir, &outward_normal);
        Some(rec)
    }
}
