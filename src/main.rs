use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;
use camera::Camera;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod inteval;
mod camera;

fn main() {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world = world;

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;
    camera.render(&world);
}
