use cgmath::{relative_eq, Zero};

use crate::geometry::{ray::Ray, raycollidable::Collision, util::random_unit_vector, Vector};

use super::material::{MaterialTrait, ScatterResult};

#[derive(PartialEq, Debug, Clone)]
pub struct Lambertian {
    albedo: Vector,
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> ScatterResult {
        let mut scatter_direction = collision.normal + random_unit_vector();

        if relative_eq!(scatter_direction, Zero::zero()) {
            scatter_direction = collision.normal;
        }

        let scatter = Ray::new(collision.point, scatter_direction, ray.time);
        return ScatterResult::Bounce(self.albedo, scatter);
    }
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Lambertian {
        Lambertian { albedo }
    }
}
