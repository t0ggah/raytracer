use raytracer::hittable::{HittableList, Sphere};
use raytracer::image::{ModifiableImage, PPM};
use raytracer::ray::Ray;
use raytracer::vector::Vec3;

fn main() {
    let image_width = 200;
    let image_height = 100;
    let mut image = PPM::new(image_width, image_height);
    build_image(&mut image, image_width, image_height);

    println!("{}", image);
}

fn build_image<T>(image_creator: &mut T, image_width: usize, image_height: usize)
where
    T: ModifiableImage,
{
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for (j, y) in (0..image_height).rev().enumerate() {
        for (i, x) in (0..image_width).enumerate() {
            let u = i as f32 / image_width as f32;
            let v = j as f32 / image_height as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let color = ray.color(&world);
            image_creator.add_pixel(x, y, color);
        }
    }
}
