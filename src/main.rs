mod camera;
mod hittable;
mod math;
mod ray;

use camera::*;
use hittable::*;
use math::*;

fn main() {
    // Image
    let camera = Camera::default();

    // World
    let mut world = HittableVec::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Render
    let img = camera.render(&world);
    img.save("res/colors.png").unwrap();
}
