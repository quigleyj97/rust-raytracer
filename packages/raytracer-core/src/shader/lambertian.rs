use crate::geometry::{
    util::vector::{near_zero, random_unit_vector},
    Collision, Ray, Vector,
};

use super::Material;

pub struct Lambertian {
    albedo: Vector,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)> {
        let mut scatter_direction = collision.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = collision.normal;
        }

        let scatter = Ray::new(collision.point, scatter_direction);
        return Option::Some((self.albedo, scatter));
    }
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Lambertian {
        Lambertian { albedo }
    }
}
