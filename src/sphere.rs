use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Option<Rc<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if !interval.surrounds(t) {
                t = (-half_b + root) / a;
                if !interval.surrounds(t) {
                    return None;
                }
            }

            let p = r.at(t);
            let normal = (p - self.center) / self.radius;
            Some(HitRecord::new(p, normal, t, r, self.material.clone()))
        }
    }
}
