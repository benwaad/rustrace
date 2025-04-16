mod ray;
mod vecs;

use image::RgbImage;
use indicatif::ProgressBar;

use ray::*;
use vecs::*;

fn ray_color(ray: &Ray) -> Color {
    Color::new(0.0, 0.0, 0.0)
}

fn main() {
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
    let vp_upper_left = CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VP_U / 2.0 - VP_V / 2.0;
    let pix00_loc = vp_upper_left + (pix_delta_u + pix_delta_v) * 0.5;

    // Render
    let mut img = RgbImage::new(W, H);
    let progress_bar = ProgressBar::new(W as u64);
    for i in 0..W {
        for j in 0..H {
            let pix_center = pix00_loc + (pix_delta_u * i as f64) + (pix_delta_v * j as f64);
            let ray_dir = pix_center - CAMERA_CENTER;
            let r = Ray::new(CAMERA_CENTER, ray_dir);

            let c = ray_color(&r);
            img.put_pixel(i, j, c.into());
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();

    img.save("res/colors.png").unwrap();
}
