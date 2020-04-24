use crate::vector::Color;
use std::fmt;
use std::fmt::Display;

pub trait ModifiableImage {
    fn add_pixel(&mut self, x: usize, y: usize, color: Color);
}

#[derive(Debug)]
pub struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
    samples_per_pixel: u8,
}

impl PPM {
    pub fn new(width: usize, height: usize, samples_per_pixel: u8) -> Self {
        PPM {
            width,
            height,
            pixels: vec![Color::default(); width * height],
            samples_per_pixel,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x + (self.width * y)]
    }
}

impl Display for PPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = (0..self.height)
            .flat_map(|y| {
                (0..self.width).map(move |x| {
                    format!("{}\n", self.get_pixel(x, y).write(self.samples_per_pixel))
                })
            })
            .collect::<String>();
        write!(f, "P3\n{} {}\n255\n{}", self.width, self.height, lines)
    }
}

impl ModifiableImage for PPM {
    fn add_pixel(&mut self, x: usize, y: usize, rgb: Color) {
        self.pixels[x + (self.width * y)] = rgb;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_and_printing_ppm_image() {
        let mut ppm = PPM::new(2, 2, 1);

        ppm.add_pixel(0, 0, Color::new(0.25, 0.5, 0.75));
        ppm.add_pixel(1, 0, Color::new(1.0, 0.75, 0.5));
        ppm.add_pixel(0, 1, Color::new(0.0, 1.0, 0.0));
        ppm.add_pixel(1, 1, Color::new(0.0, 0.0, 0.0));

        let expexted = "P3\n2 2\n255\n64 128 192\n255 192 128\n0 255 0\n0 0 0\n";
        let result = format!("{}", ppm);

        assert_eq!(result, expexted);
    }
}
