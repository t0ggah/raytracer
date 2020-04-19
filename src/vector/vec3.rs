use std::ops;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: f32) -> Self::Output {
        let k = 1.0 / rhs;
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
        self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
        self
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
        self
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
        self
    }
}

#[cfg(test)]
mod test_vec3 {
    use super::*;

    #[test]
    fn test_dividing_vec3_with_factor() {
        let vec3 = Vec3::new(2.0, 4.0, 8.0);

        let expected = Vec3::new(1.0, 2.0, 4.0);
        let result = vec3 / 2.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplying_vec3_with_factor() {
        let vec3 = Vec3::new(2.0, 4.0, 8.0);

        let expected = Vec3::new(4.0, 8.0, 16.0);
        let result = vec3 * 2.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_adding_to_vec3s_together() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(5.0, 7.0, 9.0);
        let result = a + b;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtracting_vec3_from_vec3() {
        let a = Vec3::new(4.0, 5.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);

        let expected = Vec3::new(3.0, 3.0, 3.0);
        let result = a - b;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_params() {
        let vec3 = Vec3::new(2.0, 4.0, 8.0);

        assert_eq!(vec3.x(), 2.0);
        assert_eq!(vec3.y(), 4.0);
        assert_eq!(vec3.z(), 8.0);
    }
}
