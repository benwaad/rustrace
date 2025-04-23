use crate::vecs;
use crate::{ray::Ray, vecs::Vec3, Color, HitRecord};
use rand;
use std::option::Option;

pub trait Material {
    // Always imprint a color, sometimes emit a reflected ray
    fn scatter<T: rand::Rng>(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut T,
    ) -> (&Color, Option<Ray>);
}

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

struct Dielectric {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter<T: rand::Rng>(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &mut T,
    ) -> (&Color, Option<Ray>) {
        let mut scatter_dir = rec.normal + Vec3::random_unit(rng);
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_dir);
        (&self.albedo, Some(scattered))
    }
}

impl Material for Metal {
    fn scatter<T: rand::Rng>(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _rng: &mut T,
    ) -> (&Color, Option<Ray>) {
        let scatter_dir = r_in.dir - 2.0 * vecs::dot(&r_in.dir, &rec.normal) * rec.normal;
        let scattered = Ray::new(rec.p, scatter_dir);
        (&self.albedo, Some(scattered))
    }
}

pub enum MaterialKind {
    Lambertian(Lambertian),
    Metal(Metal),
    // Dielectric(Dielectric),
}

impl MaterialKind {
    pub fn lambertian(r: f64, g: f64, b: f64) -> Self {
        MaterialKind::Lambertian(Lambertian {
            albedo: Color::new(r, g, b),
        })
    }

    pub fn metal(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        MaterialKind::Metal(Metal {
            albedo: Color::new(r, g, b),
            fuzz,
        })
    }
}

impl Material for MaterialKind {
    fn scatter<T: rand::Rng>(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut T,
    ) -> (&Color, Option<Ray>) {
        match self {
            MaterialKind::Lambertian(mat) => mat.scatter(r_in, rec, rng),
            MaterialKind::Metal(mat) => mat.scatter(r_in, rec, rng),
            // MaterialKind::Dielectric(mat) => mat.scatter(r_in, rec, rng),
        }
    }
}
