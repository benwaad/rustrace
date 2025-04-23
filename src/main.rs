mod camera;
mod hittable;
mod material;
mod math;
mod ray;

use camera::*;
use hittable::*;
use material::*;
use math::*;

fn main() {
    // Image
    let camera = Camera::default();

    let material_ground = MaterialKind::lambertian(0.8, 0.8, 0.0);
    let material_center = MaterialKind::lambertian(0.1, 0.2, 0.5);
    let material_left = MaterialKind::metal(0.8, 0.8, 0.8);
    let material_right = MaterialKind::metal(0.8, 0.6, 0.2);

    // World
    let mut world = HittableVec::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Render
    let img = camera.render(&world);
    img.save("res/scene.png").unwrap();
}
