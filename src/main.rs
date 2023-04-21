use crate::vec3::Vec3;

mod vec3;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2 as f64);
            col *= 255.99;
            col.print_as_int();
        }
    }
}
