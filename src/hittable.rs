use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Material>,
}

impl HitRecord<'_> {
    pub fn new(t: f64, p: Vec3, normal: Vec3, material: &Box<dyn Material>) -> HitRecord {
        HitRecord { t, p , normal, material }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius , material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let collision = ray.point_at_parameter(temp);
                return Some(HitRecord::new(temp, collision,  (collision - self.center) / self.radius, &self.material));
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let collision = ray.point_at_parameter(temp);
                return Some(HitRecord::new(temp, collision,  (collision - self.center) / self.radius, &self.material));
            }
        }
        None
    }
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;
        for hittable in self.list.iter() {
            if let Some(hit_record ) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                result = Some(hit_record);
            }
        }
        result
    }
}