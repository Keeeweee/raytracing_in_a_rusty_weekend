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

fn color(ray: &Ray) {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    print_header();
    
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col: Vec3 = Vec3::new(i as f64 / NX as f64, j as f64 / NY as f64, 0.2 as f64);
            col *= 255.99;
            col.print_as_int();
        }
    }
}
