mod color;
mod vec3;

pub use color::Color;
pub use vec3::Vec3;

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(&v, &n)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&(-uv), &n);
    let r_out_parallel = (uv + n * cos_theta) * etai_over_etat;
    let r_out_perp = n * -(1.0f32 - r_out_parallel.length_squared()).sqrt();
    return r_out_parallel + r_out_perp;
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    );
}
