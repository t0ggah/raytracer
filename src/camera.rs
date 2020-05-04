use crate::ray::Ray;
use crate::vector::{cross, unit_vector, Vec3};
use std::f32::consts::PI;

#[derive(Default, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let origin = lookfrom;
        let lens_radius = aperture / 2.0;

        let theta = degrees_to_radians(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * 2.0 * half_width * focus_dist;
        let vertical = v * 2.0 * half_height * focus_dist;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            v,
            u,
            _w: w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}
