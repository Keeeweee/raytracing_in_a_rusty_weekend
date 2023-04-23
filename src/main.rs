use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList, Sphere};

use rand::Rng;

mod vec3;
mod ray;
mod hittable;
mod camera;

const NX: i32 = 200;
const NY: i32 = 100;
const NS: i32 = 100;
const IMG_PATH: &str = "images/07-sphere-with-difusion-gamma-corrected.ppm";

fn print_header(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;

    Ok(())
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
                        - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
              - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    return match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
            color(&Ray::new(hit_record.p, target - hit_record.p), world) * 0.5
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

    let list: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
                                            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))];
    let world = HittableList::new(list);
    let camera = Camera::default();

    let mut rng = rand::thread_rng();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / NX as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / NY as f64;

                let ray = camera.get_ray(u, v);
                col += color(&ray, &world);
            }

            col /= NS as f64;
            col = col.sqrt();
            col *= 255.99;

            col.print_as_int(&mut file)?;
        }
    }

    Ok(())
}
