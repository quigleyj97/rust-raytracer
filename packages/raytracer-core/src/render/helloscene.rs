use cgmath::{vec3, Vector3, Point3, point3};

use crate::image::buffer::ImageBuffer;

fn norm(vec: Vector3<f64>) -> f64 {
    return f64::sqrt(cgmath::dot(vec, vec));
} 

fn point_along_ray(origin: Point3<f64>, ray: Vector3<f64>, t: f64) -> Point3<f64> {
    origin + (t * ray)
}

fn ray_color(origin: Point3<f64>, ray: Vector3<f64>) -> Vector3<f64> {
    let sphere_center = point3(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let sphere_intersection = hit_sphere(origin, ray, sphere_center, sphere_radius);
    if sphere_intersection > 0.0 {
        let normal = point_along_ray(origin, ray, sphere_intersection) - sphere_center;
        let unit_normal = normal / norm(normal);
        return 0.5 * vec3(unit_normal.x + 1.0, unit_normal.y + 1.0, unit_normal.z + 1.0);

    }
    let unit_direction = ray / norm(ray);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

fn hit_sphere(ray_origin: Point3<f64>, ray: Vector3<f64>, center: Point3<f64>, radius: f64) -> f64 {
    let oc_segment = ray_origin - center;
    let a = cgmath::dot(ray, ray);
    let b = 2.0 * cgmath::dot(oc_segment, ray);
    let c = cgmath::dot(oc_segment, oc_segment) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
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
    let vertical = vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, FOCAL_LENGTH);

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;

    for j in (0..(height - 1)).rev() {
        eprintln!("{} Scanlines remaining", j);
        for i in 0..width {
            let u: f64 = i as f64 / (width - 1) as f64;
            let v: f64 = j as f64 / (height - 1) as f64;
            let ray_origin = origin;
            let ray_direction = lower_left_corner + u * horizontal + v * vertical - origin;
            // choose color
            let color = ray_color(ray_origin, ray_direction);
            let idx = (j * width + i) * 3;
            buf.data[idx + 0] = (255.0 * color[0]).round() as u8;
            buf.data[idx + 1] = (255.0 * color[1]).round() as u8;
            buf.data[idx + 2] = (255.0 * color[2]).round() as u8;
        }
    }

    buf
}