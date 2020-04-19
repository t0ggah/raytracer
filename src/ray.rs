use crate::vector::{dot, unit_vector, Color, Vec3};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn color(&self) -> Color {
        let sphere = Vec3::new(0.0, 0.0, -1.0);
        if self.hit_sphere(&sphere, 0.5) {
            return Color::new(1.0, 0.0, 0.0);
        }
        let unit_direction = unit_vector(self.direction);

        let t = 0.5 * (unit_direction.y() + 1.0);

        Color::from_vec3(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t)
    }

    fn hit_sphere(&self, center: &Vec3, radius: f32) -> bool {
        let oc = self.origin - *center;
        let a = dot(&self.direction, &self.direction);
        let b = dot(&oc, &self.direction) * 2.0;
        let c = dot(&oc, &oc) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}
