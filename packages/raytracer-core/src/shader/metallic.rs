use cgmath::InnerSpace;

use crate::geometry::{Ray, Collision, Vector};

use super::Material;

pub struct Metallic {
    /// The 'color' of the metal
    albedo: Vector,
}

impl Metallic {
    pub fn new(albedo: Vector) -> Metallic {
        Metallic { albedo } 
    }

    fn reflect(vector: Vector, normal: Vector) -> Vector {
        vector - (cgmath::dot(vector, normal) * (2.0 * normal))
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)> {
        let reflection = Metallic::reflect(ray.direction.normalize(), collision.normal);
        return if cgmath::dot(reflection, collision.normal) > 0.0 {
            Option::Some((self.albedo, Ray::new(collision.point, reflection)))
        } else {
            Option::None
        }
    }
}