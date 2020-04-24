use crate::hittable::Hittable;
use crate::vector::{unit_vector, Color, Vec3};

const INFINITY: f32 = std::f32::MAX;
const PI: f32 = 3.1415926535897932385;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self, world: &impl Hittable, depth: u8) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(&self, 0.0, INFINITY) {
            let target = rec.p() + rec.normal() + Vec3::random_in_unit_sphere();
            let ray = Ray::new(rec.p(), target - rec.p());
            return ray.color(world, depth - 1) * 0.5;
        }

        let unit_direction = unit_vector(self.direction);

        let t = 0.5 * (unit_direction.y() + 1.0);

        Color::from_vec3(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t)
    }
}
