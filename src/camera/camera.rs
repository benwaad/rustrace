#![allow(dead_code)]

use crate::hittable::*;
use crate::material::*;
use crate::math::*;
use crate::ray::*;

use image::RgbImage;
use indicatif::ProgressBar;
use rand;
use rand::SeedableRng;
use rayon::prelude::*;
use std::time::Instant;

use std::io::{stdout, Write};

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_ray_bounces: u32,

    center: Vec3,
    pix_delta_u: Vec3,
    pix_delta_v: Vec3,
    pix00_loc: Vec3,
}

impl Camera {
    pub fn render<T: Hittable + Sync>(&self, world: &T) -> RgbImage {
        let start = Instant::now();
        let mut img = RgbImage::new(self.width, self.height);

        let progress_bar = ProgressBar::new(self.height as u64);
        let row_iters: Vec<_> = img.rows_mut().collect();
        row_iters
            .into_par_iter()
            .enumerate()
            .for_each(|(j, rowit)| {
                let mut rng = rand::rngs::SmallRng::seed_from_u64(j as u64);
                for (i, pix) in rowit.enumerate() {
                    let mut c = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i as u32, j as u32, &mut rng);
                        c += Self::ray_color(&ray, world, self.max_ray_bounces, &mut rng);
                    }

                    c = (1.0 / self.samples_per_pixel as f64) * c;

                    const INTENSITY: Interval = Interval {
                        min: 0.0,
                        max: 0.999,
                    };
                    c.r = lin_to_gamma(INTENSITY.clamp(c.r));
                    c.g = lin_to_gamma(INTENSITY.clamp(c.g));
                    c.b = lin_to_gamma(INTENSITY.clamp(c.b));

                    *pix = c.into();
                }
                progress_bar.inc(1);
            });
        progress_bar.finish();

        println!("Render finished in {:.2} s", start.elapsed().as_secs_f64());
        img
    }

    fn ray_color<T, R>(ray: &Ray, world: &T, depth: u32, rng: &mut R) -> Color
    where
        T: Hittable,
        R: rand::Rng,
    {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            if let (attenuation, Some(scattered)) = hit_rec.material.scatter(ray, &hit_rec, rng) {
                return attenuation * &Camera::ray_color(&scattered, world, depth - 1, rng);
            }
        }

        let unit = ray.dir.unit();
        let a = 0.5 * (unit.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray<T: rand::Rng>(&self, i: u32, j: u32, rng: &mut T) -> Ray {
        let offset = math::sample_square(rng);
        let (x, y) = (offset.0 + i as f64, offset.1 + j as f64);
        let pix_center = self.pix00_loc + (self.pix_delta_u * x) + (self.pix_delta_v * y);
        let ray_dir = pix_center - self.center;
        Ray::new(self.center, ray_dir)
    }

    pub fn default() -> Self {
        // Image
        const W: u32 = 1920;
        const H: u32 = 1080;
        const ASP_RAT: f64 = W as f64 / H as f64;
        const SAMPS_PER_PIX: u32 = 100;
        const MAX_RAY_BOUNCES: u32 = 10;

        // Camera
        const FOCAL_LENGTH: f64 = 1.0;
        const CAMERA_CENTER: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        const VP_H: f64 = 2.0;
        const VP_W: f64 = ASP_RAT * VP_H;
        const VP_U: Vec3 = Vec3::new(VP_W, 0.0, 0.0);
        const VP_V: Vec3 = Vec3::new(0.0, -VP_H, 0.0);

        // Viewport stuff
        let pix_delta_u = VP_U / W as f64;
        let pix_delta_v = VP_V / H as f64;
        let vp_upper_left =
            CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VP_U / 2.0 - VP_V / 2.0;
        let pix00_loc = vp_upper_left + (pix_delta_u + pix_delta_v) * 0.5;

        Camera {
            width: W,
            height: H,
            aspect_ratio: ASP_RAT,
            samples_per_pixel: SAMPS_PER_PIX,
            max_ray_bounces: MAX_RAY_BOUNCES,
            center: CAMERA_CENTER,
            pix_delta_u,
            pix_delta_v,
            pix00_loc,
        }
    }
}
