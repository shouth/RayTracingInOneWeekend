use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(Rc::new(material_ground.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Some(Rc::new(material_center.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(Rc::new(material_left.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Some(Rc::new(material_left.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(Rc::new(material_right.clone())),
    ));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Point3::new(0.0, 1.0, 0.0);
    camera.render(&world);
}
