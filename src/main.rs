use raytracer::camera::Camera;
use raytracer::hittable::{Dielectric, HittableList, Lambertian, Metal, Sphere};
use raytracer::image::{ModifiableImage, PPM};
use raytracer::random::{random, random_min_max};
use raytracer::vector::{Color, Vec3};
use std::sync::{Arc, Mutex};

fn main() {
    let image_width = 800;
    let image_height = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut image = PPM::new(image_width, image_height, samples_per_pixel);
    build_image(
        &mut image,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    );

    println!("{}", image);
}

fn random_scene() -> HittableList<Sphere> {
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::from_vec3(Vec3::random() * Vec3::random());
                    world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::from_vec3(Vec3::random_min_max(0.5, 1.0));
                    let fuzz = random_min_max(0.0, 0.5);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // glass
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));

    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));

    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

fn build_image<T>(
    image_creator: &mut T,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: u8,
    max_depth: u8,
) where
    T: ModifiableImage + Send,
{
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let aspect_ratio = image_width as f32 / image_height as f32;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world = Arc::new(random_scene());
    let image_creator = Arc::new(Mutex::new(image_creator));

    rayon::scope(|s| {
        for (j, y) in (0..image_height).rev().enumerate() {
            let image_creator = image_creator.clone();
            let world = world.clone();
            s.spawn(move |_| {
                for (i, x) in (0..image_width).enumerate() {
                    let mut color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + random()) / image_width as f32;
                        let v = (j as f32 + random()) / image_height as f32;
                        let mut ray = camera.get_ray(u, v);

                        color += ray.color(world.clone(), max_depth);
                    }

                    image_creator.lock().unwrap().add_pixel(x, y, color);
                }
            })
        }
    })
}
