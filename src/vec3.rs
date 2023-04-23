use std::fs::File;
use std::io::prelude::*;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(self.y * v.z - self.z * v.y,
                  self.z * v.x - self.x * v.z,
                  self.x - v.y - self.y * v.x)
    }

    pub fn squared_length(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn print_as_int(self, file: &mut File) -> std::io::Result<()> {
        writeln!(*file, "{} {} {}", self.x as i32, self.y as i32, self.z as i32)?;
        Ok(())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, f: f64) -> Vec3 {
        Vec3::new(self.x + f, self.y + f, self.z + f)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self + v.x, self + v.y, self + v.z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, f: f64) -> Vec3 {
        Vec3::new(self.x / f, self.y / f, self.z / f)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, f: f64) {
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}