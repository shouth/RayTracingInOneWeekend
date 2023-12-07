use std::rc::Rc;

use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;
use camera::Camera;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

fn main() {
    let mut world = HittableList::default();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(Rc::new(material_ground))));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Some(Rc::new(material_center))));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(Rc::new(material_left))));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(Rc::new(material_right))));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;
    camera.render(&world);
}
