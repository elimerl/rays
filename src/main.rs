#![allow(dead_code)]

use std::sync::atomic::AtomicU64;

use glam::{Vec3, Vec4, Vec4Swizzles};
use log::{debug, info};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use shapes::{Shape, Sphere};

use crate::shapes::{Light, Material};
mod img;
mod math;
mod ray;
mod shapes;
mod world;

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let args = std::env::args().collect::<Vec<String>>();

    let mut img = img::Img::new(1024, 1024);
    info!(
        "Rendering image with {} threads at {}x{}",
        rayon::current_num_threads(),
        img.width,
        img.height
    );

    let ray_origin = Vec4::new(0.0, 0.0, -5.0, 1.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / img.width as f32;
    let half = wall_size / 2.0;

    let color = Vec3::new(1.0, 0.2, 1.0);
    let shape = Sphere::new(
        Vec4::new(0.0, 0.0, 0.0, 1.0),
        1.0,
        Material::new(color, 0.1, 0.9, 0.9, 200.0),
    );
    let light = Light::point(Vec4::new(-10.0, 10.0, -10.0, 1.0), Vec3::new(1.0, 1.0, 1.0));
    let pixels_count = AtomicU64::new(0);

    let start = std::time::Instant::now();
    img.pixels.par_iter_mut().enumerate().for_each(|(i, p)| {
        let x = i % img.width;
        let y = i / img.width;

        let world_y = half - pixel_size * y as f32;
        let world_x = -half + pixel_size * x as f32;

        let position = Vec4::new(world_x, world_y, wall_z, 1.0);

        let direction = (position - ray_origin).normalize();
        let ray = ray::Ray::new(ray_origin, direction);
        let xs = shape.intersect(&ray);

        if xs.len() > 0 {
            let point = ray.at(xs[0].t);

            let normal = xs[0]
                .object
                .transform()
                .inverse()
                .transform_vector3(xs[0].object.normal_at(point).xyz());

            let eye = -direction;

            let color = xs[0]
                .object
                .material()
                .lighting(&light, point, eye, normal.extend(0.0));

            *p = color;
        }

        pixels_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let px = pixels_count.load(std::sync::atomic::Ordering::Relaxed);
        if px % 300000 == 1 {
            debug!(
                "{:.1}% of image rendered",
                px as f32 / (img.width * img.height) as f32 * 100.0
            );
        }
    });
    info!(
        "Finished render in {:.2} seconds",
        start.elapsed().as_secs_f32()
    );

    let out_path = if args.len() == 2 {
        args[1].as_str()
    } else {
        "out.png"
    };

    info!("Saving image to {}", out_path);
    img.save(out_path).unwrap();
    info!("Finished with all tasks");
}
