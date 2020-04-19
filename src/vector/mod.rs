mod color;
mod vec3;

pub use color::Color;
pub use vec3::Vec3;

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
