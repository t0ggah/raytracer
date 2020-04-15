use std::fmt;

#[derive(Debug)]
pub struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<[u32; 3]>,
}

impl PPM {
    pub fn new(width: usize, height: usize) -> Self {
        PPM {
            width,
            height,
            pixels: vec![[0, 0, 0]; width * height],
        }
    }

    pub fn add_pixel(&mut self, x: usize, y: usize, r: u32, g: u32, b: u32) {
        self.pixels[x + (self.width * y)][0] = r;
        self.pixels[x + (self.width * y)][1] = g;
        self.pixels[x + (self.width * y)][2] = b;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u32; 3] {
        self.pixels[x + (self.width * y)]
    }
}

impl fmt::Display for PPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = (0..self.height)
            .flat_map(|y| {
                (0..self.width).map(move |x| {
                    let pixel = self.get_pixel(x, y);
                    format!("{} {} {}\n", pixel[0], pixel[1], pixel[2])
                })
            })
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

        ppm.add_pixel(0, 0, 1, 1, 1);
        ppm.add_pixel(1, 0, 2, 2, 2);
        ppm.add_pixel(0, 1, 3, 3, 3);
        ppm.add_pixel(1, 1, 4, 4, 4);

        let expexted = "P3\n2 2\n255\n1 1 1\n2 2 2\n3 3 3\n4 4 4\n";
        let result = format!("{}", ppm);

        assert_eq!(result, expexted);
    }
}
