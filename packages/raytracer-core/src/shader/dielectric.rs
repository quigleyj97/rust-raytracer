use cgmath::{InnerSpace, vec3};
use crate::geometry::{Vector, Ray, Collision};

use super::Material;

pub struct Dielectric {
    /// The refractive index for this material as given by Snell's Law
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)> {
        let is_front_face = cgmath::dot(ray.direction, collision.normal) < 0.0;
        let face_normal = if is_front_face { collision.normal } else { -collision.normal };
        // TODO: this needs a way of accepting the other media's refractive index, for eg water-to-glass transitions
        let refractive_ratio = if is_front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        // let refractive_ratio = 1.0 / self.refraction_index;
        let refracted_ray_direction = refract(
            ray.direction.normalize(),
            face_normal,
            refractive_ratio
        );
        // let refracted_ray_direction = ray.direction * -1.0;
        return Option::Some((
            vec3(1.0, 1.0, 1.0),
            Ray::new(collision.point, refracted_ray_direction)
        ))
    }
}

/// Refract a ray according to Snell's Law
/// 
/// refractive_ratio is the ratio between the refractive indices of the materials forming the optical interface.
/// Eg, for an air-to-glass transition, the ratio would be something like (1.0 / 1.5).
fn refract(ray_direction: Vector, normal: Vector, refractive_ratio: f64) -> Vector {
    // the math here is a bit inscruitable in code form, but this page explains
    // it: https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw
    let cos_theta = f64::min(cgmath::dot(-ray_direction.normalize(), normal.normalize()), 1.0);
    let refraction_perpendicular = refractive_ratio * (ray_direction + cos_theta * normal);
    let refraction_parallel = -((1.0 - refraction_perpendicular.magnitude2()).abs().sqrt()) * normal;
    refraction_parallel + refraction_perpendicular
}