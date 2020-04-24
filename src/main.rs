use raytracer::camera::Camera;
use raytracer::hittable::{HittableList, Sphere};
use raytracer::image::{ModifiableImage, PPM};
use raytracer::random::random;
use raytracer::vector::{Color, Vec3};

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let mut image = PPM::new(image_width, image_height, samples_per_pixel);
    build_image(&mut image, image_width, image_height);

    println!("{}", image);
}

fn build_image<T>(image_creator: &mut T, image_width: usize, image_height: usize)
where
    T: ModifiableImage,
{
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    for (j, y) in (0..image_height).rev().enumerate() {
        for (i, x) in (0..image_width).enumerate() {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..100 {
                let u = (i as f32 + random()) / image_width as f32;
                let v = (j as f32 + random()) / image_height as f32;
                let ray = camera.get_ray(u, v);

                color += ray.color(&world);
            }

            image_creator.add_pixel(x, y, color);
        }
    }
}
