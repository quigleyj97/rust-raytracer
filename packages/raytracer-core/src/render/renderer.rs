use cgmath::{vec3, ElementWise, InnerSpace, Vector3};

use crate::{
    geometry::{Ray, RayCollidable},
    image::buffer::ImageBuffer,
    scene::SceneGraph,
    shader::MaterialTrait,
};

use super::{
    camera::Camera,
    iter::{Pixel, PixelIterator},
};

pub struct Renderer {
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_ray_casts: i64,
    camera: Camera,
}

impl Renderer {
    pub fn new(
        width: usize,
        height: usize,
        samples_per_pixel: usize,
        max_ray_casts: i64,
        camera: Camera,
    ) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
            max_ray_casts,
            camera,
        }
    }

    pub fn new_from_defaults(width: usize, height: usize, camera: Camera) -> Self {
        Self::new(width, height, 16, 16, camera)
    }

    pub fn render_to_buffer(
        &self,
        scene: &SceneGraph,
        buf: &mut ImageBuffer,
        iterator: PixelIterator,
    ) {
        let rng = fastrand::Rng::new();
        for Pixel { x, y } in iterator {
            let i = x;
            let j = y;
            let mut color = vec3(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let u: f64 = ((i as f64) + rng.f64()) / (self.width - 1) as f64;
                let v: f64 = ((j as f64) + rng.f64()) / (self.height - 1) as f64;
                let ray = self.camera.project_ray(u, v);
                color += ray_color(&ray, scene, 0.001, self.max_ray_casts);
            }
            color /= self.samples_per_pixel as f64;
            let idx = (j * self.width + i) * 3;
            buf.data[idx + 0] = (256.0 * color[0].sqrt()).round() as u8;
            buf.data[idx + 1] = (256.0 * color[1].sqrt()).round() as u8;
            buf.data[idx + 2] = (256.0 * color[2].sqrt()).round() as u8;
        }
    }
}

fn ray_color<T: RayCollidable>(
    ray: &Ray,
    scene: &T,
    min_clip: f64,
    max_depth: i64,
) -> Vector3<f64> {
    if max_depth < 0 {
        return vec3(0.0, 0.0, 0.0);
    }
    match scene.will_intersect(&ray, min_clip, f64::INFINITY) {
        Option::None => {
            // do nothing
        }
        Option::Some(collision) => {
            return match collision.material.scatter(ray, &collision) {
                Option::None => vec3(0.0, 0.0, 0.0),
                Option::Some((attenuation, scatter_ray)) => {
                    return attenuation.mul_element_wise(ray_color(
                        &scatter_ray,
                        scene,
                        min_clip,
                        max_depth - 1,
                    ));
                }
            };
        }
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}
