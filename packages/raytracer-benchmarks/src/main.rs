use cgmath::{point3, vec3, InnerSpace, MetricSpace};
use log::info;
use raytracer_core::render::camera::Camera;
use std::time::SystemTime;

fn main() {
    pretty_env_logger::init();

    benchmark_ray_casting();
}

fn benchmark_ray_casting() {
    const ITER_SIZE: usize = 1_000_000_000;
    let camera = Camera::new(
        point3(2.0, 3.0, 4.0),
        point3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        1.0,
        cgmath::Deg(45.0),
        22.0,
        3.0,
    );

    info!("Projecting...");

    let start = SystemTime::now();

    let rng = fastrand::Rng::new();

    // this next bit is just to convince the compiler we need the results
    // to keep it from 'optimizing' away the code under test
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    for _ in 0..ITER_SIZE {
        let ray = camera.project_ray(rng.f64(), rng.f64());
        sum1 += ray.origin.distance2(point3(0.0, 0.0, 0.0));
        sum2 += ray.direction.magnitude2();
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
