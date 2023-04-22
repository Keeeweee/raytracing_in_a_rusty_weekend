use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod ray;

const NX: i32 = 200;
const NY: i32 = 100;
const IMG_PATH: &str = "images/03-sphere_hit_with_normals.ppm";

fn print_header(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;

    Ok(())
}

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 { - 1.0 } else { (-b - discriminant.sqrt()) / (2.0 * a) }
}

fn color(ray: Ray) -> Vec3 {
    let t =  hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = ray.point_at_parameter(t).unit() - Vec3::new(0.0, 0.0, -1.0);
        return (Vec3::new(normal.x, normal.y, normal.z) + 1.0) / 2.0;
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let mut file = File::create(IMG_PATH)?;
    print_header(&mut file)?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..NY).rev() {
        for i in 0..NX {

            let u = i as f64 / NX as f64;
            let v = j as f64 / NY as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let mut col = color(ray) * 255.99;
            col.print_as_int(&mut file)?;
        }
    }

    Ok(())
}
