use cgmath::{vec3, Vector3, InnerSpace};
use log::{debug, info};

use crate::{image::buffer::ImageBuffer, geometry::{RayCollidable, Ray}, scene::new_test_world, render::{camera::Camera, iter::{PixelIterator, Pixel, ChunkedPixelIterator}}};


fn ray_color<T: RayCollidable>(ray: &Ray, scene: &T, min_clip: f64, max_depth: i64) -> Vector3<f64> {
    if max_depth < 0 {
        return vec3(0.0, 0.0, 0.0);
    }
    match scene.will_intersect(&ray, min_clip, f64::INFINITY) {
        Option::None => {
            // do nothing
        },
        Option::Some(collision) => {
            return match collision.material.scatter(ray, &collision) {
                Option::None => vec3(0.0, 0.0, 0.0),
                Option::Some((attenuation, scatter_ray)) => {
                    return attenuation.zip(ray_color(&scatter_ray, scene, min_clip, max_depth - 1), |a, b| -> f64 {a * b});
                }
            };
        }
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

pub fn render_helloworld() -> ImageBuffer {
    const WIDTH: usize = 720;
    const HEIGHT: usize = 405;
    const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;
    const SAMPLES_PER_PIXEL: i64 = 4;
    const MAX_RAY_CASTS: i64 = 10;

    let camera = Camera::new(
        ASPECT_RATIO
    );

    debug!("Output dimensions: {} x {} @ {}", WIDTH, HEIGHT, ASPECT_RATIO);

    let scene = new_test_world();

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;
    let mut i_chunk = 0;

    for chunk in ChunkedPixelIterator::with_chunks(width, height, 10){
        info!("Rendering chunk {} of {}", i_chunk, 10);
        i_chunk += 1;
        for Pixel {x, y} in chunk {
            let i = x;
            let j = y;
            let mut color = vec3(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = ((i as f64) + rand::random::<f64>()) / (width - 1) as f64;
                let v: f64 = ((j as f64) + rand::random::<f64>()) / (height - 1) as f64;
                let ray = camera.project_ray(u, v);
                color += ray_color(&ray, &scene,  0.001, MAX_RAY_CASTS);
            }
            color /= SAMPLES_PER_PIXEL as f64;
            let idx = (j * width + i) * 3;
            buf.data[idx + 0] = (256.0 * color[0].sqrt()).round() as u8;
            buf.data[idx + 1] = (256.0 * color[1].sqrt()).round() as u8;
            buf.data[idx + 2] = (256.0 * color[2].sqrt()).round() as u8;
        }
    }

    buf
}