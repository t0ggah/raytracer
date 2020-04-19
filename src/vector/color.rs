use crate::vector::Vec3;
use std::fmt;

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
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ir = 255.999 * self.get_r();
        let ig = 255.999 * self.get_g();
        let ib = 255.999 * self.get_b();
        write!(f, "{} {} {}", ir as u32, ig as u32, ib as u32)
    }
}
