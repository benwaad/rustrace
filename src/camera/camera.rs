#![allow(dead_code)]

use crate::hittable::*;
use crate::math::*;
use crate::ray::*;

use image::RgbImage;
use indicatif::ProgressBar;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,

    center: Vec3,
    pix_delta_u: Vec3,
    pix_delta_v: Vec3,
    pix00_loc: Vec3,
}

impl Camera {
    pub fn render<T: Hittable>(&self, world: &T) -> RgbImage {
        let mut img = RgbImage::new(self.width, self.height);
        let progress_bar = ProgressBar::new(self.width as u64);
        for i in 0..self.width {
            for j in 0..self.height {
                let pix_center =
                    self.pix00_loc + (self.pix_delta_u * i as f64) + (self.pix_delta_v * j as f64);
                let ray_dir = pix_center - self.center;
                let r = Ray::new(self.center, ray_dir);

                let c = Self::ray_color(&r, world);
                img.put_pixel(i, j, c.into());
            }
            progress_bar.inc(1);
        }
        progress_bar.finish();
        img
    }

    fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
        if let Some(hit_rec) = world.hit(ray, Interval::new(0.0, INFINITY)) {
            let normal = &hit_rec.normal;
            return 0.5 * (Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0));
        }

        let unit = ray.dir.unit();
        let a = 0.5 * (unit.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn default() -> Self {
        // Image
        const W: u32 = 1920;
        const H: u32 = 1080;
        const ASP_RAT: f64 = W as f64 / H as f64;

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
            center: CAMERA_CENTER,
            pix_delta_u,
            pix_delta_v,
            pix00_loc,
        }
    }
}
