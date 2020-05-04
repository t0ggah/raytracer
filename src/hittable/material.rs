use crate::hittable::HitRecord;
use crate::random::random;
use crate::ray::Ray;
use crate::vector::{dot, reflect, refract, unit_vector, Color, Vec3};

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color,
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(unit_vector(ray_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some(Scatter {
                scattered,
                attenuation,
            });
        }

        None
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}

#[derive(Debug)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Self {
        Self { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some(Scatter {
                scattered,
                attenuation,
            });
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random() < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some(Scatter {
                scattered,
                attenuation,
            });
        }

        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted);
        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}
