#![feature(portable_simd)]

use cgmath::{point3, vec3, InnerSpace, MetricSpace};
use log::info;
use raytracer_core::geometry::f32_sphere::F32Sphere;
use std::time::SystemTime;

fn main() {
    pretty_env_logger::init();

    benchmark_ray_casting();
}

fn benchmark_ray_casting() {
    const ITER_SIZE: usize = 100_000_000;

    info!("Projecting...");

    let rng = fastrand::Rng::new();
    let test_sphere = F32Sphere::new(
        point3(rng.f32(), rng.f32(), rng.f32()),
        0.5 + (rng.f32() / 2.0),
    );

    // this next bit is just to convince the compiler we need the results
    // to keep it from 'optimizing' away the code under test
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    let start = SystemTime::now();

    // Naive impl: ~2800ms avg
    // SIMD f64x4: ~3800ms avg
    // SIMD f32x4: ~3400ms avg
    // Naive f32: ~12000ms avg

    for _ in 0..ITER_SIZE {
        let ray_origin = point3(rng.f32(), rng.f32(), rng.f32());
        let ray_direction = vec3(rng.f32(), rng.f32(), rng.f32());
        if let Some(collision) =
            test_sphere.will_intersect(ray_origin, ray_direction, 0.001, f32::INFINITY)
        {
            sum1 += collision.point.distance2(point3(0.0, 0.0, 0.0));
            sum2 += collision.normal.magnitude2();
        }
    }

    let end = SystemTime::now();

    info!("Finished projecting rays");
    println!("{}, {}", sum1, sum2);
    let end_print = SystemTime::now();
    info!("Finished streaming results");
    info!(
        "Timing: Total {} ms / Projection {} ms / Streaming {} ms",
        end_print.duration_since(start).unwrap().as_millis(),
        end.duration_since(start).unwrap().as_millis(),
        end_print.duration_since(end).unwrap().as_millis()
    );
}
