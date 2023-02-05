use crate::geometry::{Collision, Ray, Vector};
use cgmath::{vec3, InnerSpace};

use super::MaterialTrait;

pub struct Dielectric {
    /// The refractive index for this material as given by Snell's Law
    refraction_index: f64,
}

const AIR_REFRACTIVE_INDEX: f64 = 1.0;

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cos_theta: f64, reflective_index: f64) -> f64 {
        // https://en.wikipedia.org/wiki/Schlick%27s_approximation
        let r0 = ((AIR_REFRACTIVE_INDEX - reflective_index)
            / (AIR_REFRACTIVE_INDEX + reflective_index))
            .powi(2);
        return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
    }
}

impl MaterialTrait for Dielectric {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)> {
        let is_front_face = cgmath::dot(ray.direction, collision.normal) < 0.0;
        let face_normal = if is_front_face {
            collision.normal
        } else {
            -collision.normal
        };
        // TODO: this needs a way of accepting the other media's refractive index, for eg water-to-glass transitions
        let refractive_ratio = if is_front_face {
            AIR_REFRACTIVE_INDEX / self.refraction_index
        } else {
            self.refraction_index /* / AIR_REFRACTIVE_INDEX */
        };

        let cos_theta = f64::min(
            cgmath::dot(-ray.direction.normalize(), face_normal.normalize()),
            1.0,
        );
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let can_refract = refractive_ratio * sin_theta <= 1.0;
        let should_reflect = Dielectric::reflectance(cos_theta, refractive_ratio) > fastrand::f64();
        let refracted_ray_direction = if can_refract && !should_reflect {
            refract_hack(
                ray.direction.normalize(),
                face_normal,
                refractive_ratio,
                cos_theta,
            )
        } else {
            reflect(ray.direction, face_normal)
        };
        return Option::Some((
            vec3(1.0, 1.0, 1.0),
            Ray::new(collision.point, refracted_ray_direction),
        ));
    }
}

/// Refract a ray according to Snell's Law
///
/// refractive_ratio is the ratio between the refractive indices of the materials forming the optical interface.
/// Eg, for an air-to-glass transition, the ratio would be something like (1.0 / 1.5).
///
/// cos_theta is a perf hack that I probably don't need anyway *shrug*
/// but it comes from |A| * |B| * cos (angle btw A and B)
fn refract_hack(
    ray_direction: Vector,
    normal: Vector,
    refractive_ratio: f64,
    cos_theta: f64,
) -> Vector {
    // the math here is a bit inscruitable in code form, but this page explains
    // it: https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw
    let refraction_perpendicular = refractive_ratio * (ray_direction + cos_theta * normal);
    let refraction_parallel =
        -((1.0 - refraction_perpendicular.magnitude2()).abs().sqrt()) * normal;
    refraction_parallel + refraction_perpendicular
}

// fn refract(ray_direction: Vector, normal: Vector, refractive_ratio: f64) -> Vector {
//     let cos_theta = f64::min(cgmath::dot(-ray_direction.normalize(), normal.normalize()), 1.0);
//     refract_hack(ray_direction, normal, refractive_ratio, cos_theta)
// }

fn reflect(vector: Vector, normal: Vector) -> Vector {
    vector - (cgmath::dot(vector, normal) * (2.0 * normal))
}
