use crate::vector::{unit_vector, Color, Vec3};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn color(&self) -> Color {
        let unit_direction = unit_vector(self.direction);

        let t = 0.5 * (unit_direction.y() + 1.0);

        Color::from_vec3(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t)
    }
}
