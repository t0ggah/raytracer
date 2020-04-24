mod hittable_list;
mod sphere;

pub use hittable_list::HittableList;
pub use sphere::Sphere;

use crate::ray::Ray;
use crate::vector::{dot, Vec3};

#[derive(Debug, Default)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
