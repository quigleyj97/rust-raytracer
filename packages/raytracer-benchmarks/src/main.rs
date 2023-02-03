use cgmath::{point3, vec3, InnerSpace, MetricSpace};
use log::info;
use raytracer_core::geometry::{sphere::Sphere, Ray, RayCollidable};
use std::time::SystemTime;

fn main() {
    pretty_env_logger::init();

    benchmark_ray_casting();
}

fn benchmark_ray_casting() {
    const ITER_SIZE: usize = 100_000_000;

    info!("Projecting...");

    let rng = fastrand::Rng::new();
    let test_sphere = Sphere::new(
        point3(rng.f64(), rng.f64(), rng.f64()),
        0.5 + (rng.f64() / 2.0),
    );

    // this next bit is just to convince the compiler we need the results
    // to keep it from 'optimizing' away the code under test
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    let start = SystemTime::now();

    for _ in 0..ITER_SIZE {
        let ray = Ray::new(
            point3(rng.f64(), rng.f64(), rng.f64()),
            vec3(rng.f64(), rng.f64(), rng.f64()),
        );
        if let Some(collision) = test_sphere.will_intersect(&ray, 0.001, f64::INFINITY) {
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
