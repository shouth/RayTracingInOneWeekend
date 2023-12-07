use std::ops::{Deref, DerefMut};

use crate::{vec3::Vec3, inteval::Interval};

pub type Color = Vec3;

impl Color {
    fn linear_to_gamma(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn color_str(&self, samples_per_pixel: i32) -> String {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        r = Self::linear_to_gamma(r);
        g = Self::linear_to_gamma(g);
        b = Self::linear_to_gamma(b);

        const INTENSITY: Interval = Interval::new(0.0, 0.999);

        format!("{} {} {}",
            (INTENSITY.clamp(r) * 256.0) as i32,
            (INTENSITY.clamp(g) * 256.0) as i32,
            (INTENSITY.clamp(b) * 256.0) as i32,
        )
    }
}
