use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;
use crate::hittable::{Hittable, HittableList, Sphere};

mod vec3;
mod ray;
mod hittable;

const NX: i32 = 200;
const NY: i32 = 100;
const IMG_PATH: &str = "images/04-hello_world.ppm";

fn print_header(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;

    Ok(())
}

fn color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    return match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            (Vec3::new(hit_record.normal.x, hit_record.normal.y, hit_record.normal.z) + 1.0) / 2.0
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * unit_direction.y + 1.0;
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::create(IMG_PATH)?;
    print_header(&mut file)?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let list: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
                                            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))];
    let world = HittableList::new(list);

    for j in (0..NY).rev() {
        for i in 0..NX {

            let u = i as f64 / NX as f64;
            let v = j as f64 / NY as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&ray, &world) * 255.99;
            col.print_as_int(&mut file)?;
        }
    }

    Ok(())
}
