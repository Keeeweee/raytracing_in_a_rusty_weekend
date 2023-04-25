use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        let origin = look_from;
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        Camera { lower_left_corner, horizontal, vertical, origin }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}