use crate::vector::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub fn from_vec3(v: Vec3) -> Self {
        Color(v)
    }

    pub fn get_r(&self) -> f32 {
        self.0.e[0]
    }

    pub fn get_g(&self) -> f32 {
        self.0.e[1]
    }

    pub fn get_b(&self) -> f32 {
        self.0.e[2]
    }

    pub fn write(&self, samples_per_pixel: u8) -> String {
        let scale = 1.0 / samples_per_pixel as f32;
        let r = (scale * self.get_r()).sqrt();
        let g = (scale * self.get_g()).sqrt();
        let b = (scale * self.get_b()).sqrt();
        let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
        format!("{} {} {}", ir, ig, ib)
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };

    x
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0.e[0] += rhs.0.e[0];
        self.0.e[1] += rhs.0.e[1];
        self.0.e[2] += rhs.0.e[2];
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.0 = self.0 * rhs;
        self
    }
}
