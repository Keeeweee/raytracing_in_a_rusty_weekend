use rand::Rng;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;


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

pub fn reflect(v: Vec3, n: Vec3) -> Vec3{
    v - 2.0 * v.dot(n) * n
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        Some((Ray::new(hit_record.p, target- hit_record.p), self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray.direction.unit(), hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.direction.dot(hit_record.normal) > 0.0 { Some((scattered, self.albedo)) } else { None }
    }
}