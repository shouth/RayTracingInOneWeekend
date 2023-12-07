use crate::{hittable::Hittable, ray::Ray, color::Color, interval::Interval, vec3::{Point3, Vec3}};

use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub sample_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            sample_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:3}", (self.image_height - j));
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                println!("{}", pixel_color.color_str(self.sample_per_pixel));
            }
        }

        eprint!("\rDone.                    \n");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.center = Point3::default();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + self.pixel_delta_u * i as f64 + self.pixel_delta_v * j as f64;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = rng.gen_range(-0.5..=0.5);
        let py = rng.gen_range(-0.5..=0.5);
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        match world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            Some(record) => {
                match record.mat.as_ref().and_then(|mat| mat.scatter(r, &record)) {
                    Some((attenuation, scattered)) => attenuation * self.ray_color(&scattered, depth - 1, world),
                    None => Color::default(),
                }
            },
            None => {
                let unit_direction = r.direction().unit();
                let t = 0.5 * (unit_direction.y() + 1.0);
                let white = Color::new(1.0, 1.0, 1.0);
                let blue = Color::new(0.5, 0.7, 1.0);
                (1.0 - t) * white + t * blue
            }
        }
    }
}
