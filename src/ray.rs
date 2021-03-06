use crate::hittable::Hittable;
use crate::vector::{unit_vector, Color, Vec3};
use std::sync::Arc;

const INFINITY: f32 = std::f32::MAX;

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

    pub fn color(&mut self, world: Arc<impl Hittable>, max_depth: u8) -> Color {
        let factor = 1.0;
        let depth = 0;
        if depth >= max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(&self, 0.001, INFINITY) {
            Some(rec) => {
                if let Some(material) = rec.material.clone() {
                    if let Some(mut scatter) = material.scatter(self, &rec) {
                        return scatter.attenuation * scatter.scattered.color(world, max_depth - 1);
                    }
                }

                Color::new(0., 0., 0.)
            }
            None => {
                let unit_direction = unit_vector(self.direction);

                let t = 0.5 * (unit_direction.y() + 1.0);

                Color::from_vec3(
                    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t,
                ) * factor
            }
        }
    }
}
