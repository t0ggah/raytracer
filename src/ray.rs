use crate::hittable::Hittable;
use crate::vector::{unit_vector, Color, Vec3};

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

    pub fn color(&mut self, world: &impl Hittable, max_depth: u8) -> Color {
        let mut factor = 1.0;
        let mut depth = max_depth;
        loop {
            match (depth, world.hit(&self, 0.001, INFINITY)) {
                (depth, _) if depth <= 0 => {
                    break Color::new(0.0, 0.0, 0.0);
                }
                (_, Some(rec)) => {
                    let target = rec.p() + rec.normal() + Vec3::random_unit_vector();
                    self.origin = rec.p();
                    self.direction = target - rec.p();
                    factor *= 0.5;
                    depth -= 1;
                    continue;
                }
                (_, None) => {
                    let unit_direction = unit_vector(self.direction);

                    let t = 0.5 * (unit_direction.y() + 1.0);

                    break Color::from_vec3(
                        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t,
                    ) * factor;
                }
            }
        }
    }
}
