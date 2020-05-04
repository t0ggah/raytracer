use raytracer::camera::Camera;
use raytracer::hittable::{Dielectric, HittableList, Lambertian, Metal, Sphere};
use raytracer::image::{ModifiableImage, PPM};
use raytracer::random::random;
use raytracer::vector::{Color, Vec3};

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut image = PPM::new(image_width, image_height, samples_per_pixel);
    build_image(&mut image, image_width, image_height, max_depth);

    println!("{}", image);
}

fn build_image<T>(image_creator: &mut T, image_width: usize, image_height: usize, max_depth: u8)
where
    T: ModifiableImage,
{
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let aspect_ratio = image_width as f32 / image_height as f32;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut world = HittableList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.8, 0.8, 0.0)),
    ));
    world.add(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Color::new(0.8, 0.6, 0.2), 0.3),
    ));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Dielectric::new(1.5),
    ));

    for (j, y) in (0..image_height).rev().enumerate() {
        for (i, x) in (0..image_width).enumerate() {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..100 {
                let u = (i as f32 + random()) / image_width as f32;
                let v = (j as f32 + random()) / image_height as f32;
                let mut ray = camera.get_ray(u, v);

                color += ray.color(&world, max_depth);
            }

            image_creator.add_pixel(x, y, color);
        }
    }
}
