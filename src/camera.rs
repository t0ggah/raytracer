use crate::ray::Ray;
use crate::vector::{cross, unit_vector, Vec3};
use std::f32::consts::PI;

#[derive(Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let origin = lookfrom;

        let theta = degrees_to_radians(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let lower_left_corner = origin - u * half_width - v * half_height - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}
