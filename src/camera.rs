use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;
use rand::Rng;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w : Vec3,
    pub lens_radius: f64,
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0)
        - Vec3::new(1.0, 1.0, 0.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0)
            - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

impl Camera {

    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        let origin = look_from;
        let lower_left_corner = origin - focus_dist * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * half_width * focus_dist *u;
        let vertical = 2.0 * half_height * focus_dist * v;
        Camera { lower_left_corner, horizontal, vertical, origin, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}