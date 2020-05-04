use crate::hittable::{HitRecord, Hittable, Material};
use crate::ray::Ray;
use crate::vector::{dot, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: impl Material + 'static + Send + Sync) -> Self {
        Self {
            center,
            radius,
            material: Arc::new(material),
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let mut hit_record = HitRecord::default();
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.at(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(ray, &outward_normal);
                hit_record.material = Some(self.material.clone());
                return Some(hit_record);
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.at(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;

                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(ray, &outward_normal);
                hit_record.material = Some(self.material.clone());
                return Some(hit_record);
            }
        }

        None
    }
}
