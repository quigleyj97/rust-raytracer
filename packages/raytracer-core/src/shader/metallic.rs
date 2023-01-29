use cgmath::InnerSpace;

use crate::geometry::{util, Collision, Ray, Vector};

use super::Material;

pub struct Metallic {
    /// The 'color' of the metal
    albedo: Vector,
    /// How 'fuzzy' the material's reflections are, to mimic matte finishes
    fuzziness: f64,
}

impl Metallic {
    pub fn new(albedo: Vector, fuzziness: f64) -> Metallic {
        Metallic { albedo, fuzziness }
    }

    fn reflect(vector: Vector, normal: Vector) -> Vector {
        vector - (cgmath::dot(vector, normal) * (2.0 * normal))
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)> {
        let reflection = Metallic::reflect(ray.direction.normalize(), collision.normal);
        return if cgmath::dot(reflection, collision.normal) > 0.0 {
            let reflection_fuzzed = if self.fuzziness != 0.0 {
                reflection + (self.fuzziness * util::vector::random_unit_vector())
            } else {
                reflection
            };
            let scatter_ray = Ray::new(collision.point, reflection_fuzzed);
            Option::Some((self.albedo, scatter_ray))
        } else {
            Option::None
        };
    }
}
