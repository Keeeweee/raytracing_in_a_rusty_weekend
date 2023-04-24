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

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 { Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt()) } else { None }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3{
    v - 2.0 * v.dot(n) * n
}

pub fn schlick (cosine: f64, ref_index: f64) -> f64 {
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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

pub struct Dielectric {
    ref_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Dielectric {
        Dielectric { ref_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray.direction.unit(), hit_record.normal);
        let outward_normal: Vec3;
        let ni_over_nt: f64;
        let cosine: f64;
        let dot = ray.direction.dot(hit_record.normal);
        if dot > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_index;
            cosine = self.ref_index * dot / ray.direction.length();
        }
        else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0/self.ref_index;
            cosine = -dot / ray.direction.length();
        }

        return match refract(ray.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0.0..1.0)  < schlick(cosine, self.ref_index) {
                    Some((Ray::new(hit_record.p, reflected), Vec3::new(1.0, 1.0, 1.0)))
                }
                else {
                    Some((Ray::new(hit_record.p, refracted), Vec3::new(1.0, 1.0, 1.0)))
                }
            }
            None => {
                Some((Ray::new(hit_record.p, reflected), Vec3::new(1.0, 1.0, 1.0)))
            }
        }
    }
}