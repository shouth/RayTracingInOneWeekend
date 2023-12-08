use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;
use sphere::Sphere;
use vec3::{Point3, Vec3};

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
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Rc::new(ground_material.clone())),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let mut rng = rand::thread_rng();
            let choose_mat = rng.gen_range(0.0..=1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..=1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::new(
                        rng.gen_range(0.0..=1.0),
                        rng.gen_range(0.0..=1.0),
                        rng.gen_range(0.0..=1.0),
                    );
                    let albedo = albedo * albedo;
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, Some(Rc::new(sphere_material))));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(
                        rng.gen_range(0.5..=1.0),
                        rng.gen_range(0.5..=1.0),
                        rng.gen_range(0.5..=1.0),
                    );
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, Some(Rc::new(sphere_material))));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, Some(Rc::new(sphere_material))));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Some(Rc::new(material1)),
    ));
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(Rc::new(material2)),
    ));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Some(Rc::new(material3)),
    ));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.sample_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;
    camera.render(&world);
}
