use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList, Sphere};

use rand::Rng;
use crate::material::{Dielectric, Lambertian, Metal};

mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;

const NX: i32 = 200 * 5;
const NY: i32 = 100 * 5;
const NS: i32 = 100 * 5;
const IMG_PATH: &str = "images/14-final-image.ppm";

const MAX_DEPTH:i32 = 50;

fn print_header(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;

    Ok(())
}

fn color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    return match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            if depth > MAX_DEPTH {
                return Vec3::new(0.0, 0.0, 0.0);
            }

            match hit_record.material.scatter(ray, &hit_record) {
                Some((scatter, attenuation)) => {
                    attenuation * color(&scatter, world, depth + 1)
                }
                None => {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            }
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * unit_direction.y + 1.0;
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut list: Vec<Box<dyn Hittable>> = vec![];
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(a as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..1.0));
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = rng.gen_range(0.0..1.0);
                if choose_mat < 0.8 { // Diffuse
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian::new(Vec3::new(rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                                                                                                                                   rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                                                                                                                                   rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0)))))));
                }
                else if choose_mat < 0.95 { // Metal
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Metal::new(Vec3::new(0.5 * (1.0 + rng.gen_range(0.0..1.0)),
                                                                                                                            0.5 * (1.0 + rng.gen_range(0.0..1.0)),
                                                                                                                            0.5 * (1.0 + rng.gen_range(0.0..1.0))),
                                                                                                                0.5 * rng.gen_range(0.0..1.0))))));
                }
                else { // Glass
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5)))));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

    HittableList::new(list)
}

fn original_scene() -> HittableList {
    let list: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                             0.5,
                             Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                             100.0,
                             Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0),
                             0.5,
                             Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2),
                                                 0.3)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                             0.5,
                             Box::new(Dielectric::new(1.5)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                             -0.45,
                             Box::new(Dielectric::new(1.5)))),

    ];
    HittableList::new(list)
}

fn main() -> std::io::Result<()> {
    let mut file = File::create(IMG_PATH)?;
    print_header(&mut file)?;

    let world = random_scene();
    let look_from = Vec3::new(16.0, 2.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(look_from,
                                        look_at,
                                        Vec3::new(0.0, 1.0, 0.0),
                                            15.0,
                                            NX as f64 / NY as f64,
                                            0.2,
                             (look_from - look_at).length());

    let mut rng = rand::thread_rng();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / NX as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / NY as f64;

                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }

            col /= NS as f64;
            col = col.sqrt();
            col *= 255.99;

            col.print_as_int(&mut file)?;
        }
    }

    Ok(())
}
