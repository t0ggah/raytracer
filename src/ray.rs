use crate::vector::{dot, unit_vector, Color, Vec3};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self) -> Color {
        if let Some(t) = self.hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5) {
            let n = unit_vector(self.at(t) - Vec3::new(0.0, 0.0, -1.0));
            let vec = Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
            return Color::from_vec3(vec);
        }

        let unit_direction = unit_vector(self.direction);

        let t = 0.5 * (unit_direction.y() + 1.0);

        Color::from_vec3(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t)
    }

    fn hit_sphere(&self, center: &Vec3, radius: f32) -> Option<f32> {
        let oc = self.origin - *center;
        let a = self.direction.length_squared();
        let half_b = dot(&oc, &self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            return Some((-half_b - discriminant.sqrt()) / a);
        }
    }
}
