use crate::color::Color;
use std::fmt;

#[derive(Debug)]
pub struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl PPM {
    pub fn new(width: usize, height: usize) -> Self {
        PPM {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn add_pixel(&mut self, x: usize, y: usize, rgb: Color) {
        self.pixels[x + (self.width * y)] = rgb;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x + (self.width * y)]
    }
}

impl fmt::Display for PPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| format!("{}\n", self.get_pixel(x, y))))
            .collect::<String>();
        write!(f, "P3\n{} {}\n255\n{}", self.width, self.height, lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_and_printing_ppm_image() {
        let mut ppm = PPM::new(2, 2);

        ppm.add_pixel(0, 0, Color::new(1.0, 1.0, 1.0));
        ppm.add_pixel(1, 0, Color::new(2.0, 2.0, 2.0));
        ppm.add_pixel(0, 1, Color::new(3.0, 3.0, 3.0));
        ppm.add_pixel(1, 1, Color::new(4.0, 4.0, 4.0));

        let expexted = "P3\n2 2\n255\n1 1 1\n2 2 2\n3 3 3\n4 4 4\n";
        let result = format!("{}", ppm);

        assert_eq!(result, expexted);
    }
}
