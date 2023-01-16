use cgmath::{vec3, Vector3, point3, InnerSpace};

use crate::{image::buffer::ImageBuffer, geometry::{sphere::Sphere, RayCollidable, ray::Ray}};


fn ray_color(ray: &Ray, scene: &Vec<Box<dyn RayCollidable>>) -> Vector3<f64> {
    for object in scene {
        match &object.will_intersect(&ray, 0.0, 3.0) {
            Option::None => {
                // do nothing
            },
            Option::Some(collision) => {
                let unit_normal = collision.normal.normalize();
                return 0.5 * vec3(unit_normal.x + 1.0, unit_normal.y + 1.0, unit_normal.z + 1.0);
            }
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
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    eprintln!("Rendering parameters");
    eprintln!("  Output: {} x {} @ {}", WIDTH, HEIGHT, ASPECT_RATIO);
    eprintln!("  Viewport: {} x {}", VIEWPORT_WIDTH, VIEWPORT_HEIGHT);

    let origin = point3(0.0, 0.0, 0.0);
    let horizontal = vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = vec3(0.0, -VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, FOCAL_LENGTH);

    let scene: Vec<Box<dyn RayCollidable>> = vec![
        Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(point3(0.0, -100.5, -1.0), 100.0))
    ];

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;

    for j in (0..(height - 1)).rev() {
        eprintln!("{} Scanlines remaining", j);
        for i in 0..width {
            let u: f64 = i as f64 / (width - 1) as f64;
            let v: f64 = j as f64 / (height - 1) as f64;
            let ray_direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, ray_direction);
            // choose color
            let color = ray_color(&ray, &scene);
            let idx = (j * width + i) * 3;
            buf.data[idx + 0] = (255.0 * color[0]).round() as u8;
            buf.data[idx + 1] = (255.0 * color[1]).round() as u8;
            buf.data[idx + 2] = (255.0 * color[2]).round() as u8;
        }
    }

    buf
}