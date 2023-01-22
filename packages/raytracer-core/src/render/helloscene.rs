use cgmath::{vec3, Vector3, InnerSpace};
use log::debug;

use crate::{image::buffer::ImageBuffer, geometry::{RayCollidable, Ray}, scene::new_test_world, render::camera::Camera};


fn ray_color<T: RayCollidable>(ray: &Ray, scene: T) -> Vector3<f64> {
    match &scene.will_intersect(&ray, 0.0, f64::INFINITY) {
        Option::None => {
            // do nothing
        },
        Option::Some(collision) => {
            let unit_normal = collision.normal.normalize();
            return 0.5 * vec3(unit_normal.x + 1.0, unit_normal.y + 1.0, unit_normal.z + 1.0);
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
    const SAMPLES_PER_PIXEL: i64 = 100;

    let camera = Camera::new(
        ASPECT_RATIO
    );

    debug!("Output dimensions: {} x {} @ {}", WIDTH, HEIGHT, ASPECT_RATIO);

    let scene = new_test_world();

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;

    for j in (0..(height - 1)).rev() {
        debug!("{} Scanlines remaining", j);
        for i in 0..width {
            let mut color = vec3(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = ((i as f64) + rand::random::<f64>()) / (width - 1) as f64;
                let v: f64 = ((j as f64) + rand::random::<f64>()) / (height - 1) as f64;
                let ray = camera.project_ray(u, v);
                color += ray_color(&ray, &scene);
            }
            color /= SAMPLES_PER_PIXEL as f64;
            let idx = (j * width + i) * 3;
            buf.data[idx + 0] = (255.0 * color[0]).round() as u8;
            buf.data[idx + 1] = (255.0 * color[1]).round() as u8;
            buf.data[idx + 2] = (255.0 * color[2]).round() as u8;
        }
    }

    buf
}