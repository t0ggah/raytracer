use raytracer::image::PPM;

fn main() {
    let image_width = 200;
    let image_height = 100;
    let mut image = PPM::new(image_width, image_height);

    for (j, y) in (0..image_height).rev().enumerate() {
        for (i, x) in (0..image_width).enumerate() {
            let r = i as f32 / image_width as f32;
            let g = j as f32 / image_height as f32;
            let b = 0.2 as f32;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            image.add_pixel(x, y, ir, ig, ib);
        }
    }

    println!("{}", image);
}
