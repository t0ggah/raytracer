pub fn random() -> f32 {
    rand::random()
}

pub fn random_min_max(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}
