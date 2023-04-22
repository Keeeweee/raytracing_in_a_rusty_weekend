use crate::ray::Ray;
use crate::vec3::Vec3;

mod vec3;
mod ray;

const NX: i32 = 200;
const NY: i32 = 100;

fn print_header() {
    println!("P3");
    println!("{} {}", NX, NY);
    println!("255");
}

fn color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    print_header();
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..NY).rev() {
        for i in 0..NX {

            let u = i as f64 / NX as f64;
            let v = j as f64 / NY as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let mut col = color(ray);
            col *= 255.99;
            col.print_as_int();
        }
    }
}
