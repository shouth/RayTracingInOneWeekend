use crate::interval::Interval;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
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

            let mut record = HitRecord::default();
            record.t = t;
            record.p = r.at(t);
            let outward_normal = (record.p - self.center) / self.radius;
            record.set_face_normal(r, outward_normal);

            Some(record)
        }
    }
}
