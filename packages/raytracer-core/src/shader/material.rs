use super::prelude::*;
use super::Color;
use crate::geometry::{ray::Ray, raycollidable::Collision};
use crate::macros::make_from;

pub trait MaterialTrait {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> ScatterResult;
}

#[derive(Clone, PartialEq, Debug)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metallic(Metallic),
}

pub enum ScatterResult {
    /// The scatter ray was bounced off this material at the collision site
    Bounce(
        /// The apparent color of this object to apply to the bounce ray
        Color,
        /// The reflected (or refracted) bounce ray to cast next
        Ray,
    ),
    /// The scatter ray was absorbed by this material at the collision site
    Absorb(
        /// The apparent color of this object at the collision site
        Color,
    ),
}

impl MaterialTrait for Material {
    #[inline(always)]
    fn scatter(&self, ray: &Ray, collision: &Collision) -> ScatterResult {
        match self {
            Material::Dielectric(dielectric) => dielectric.scatter(ray, collision),
            Material::Lambertian(lambertian) => lambertian.scatter(ray, collision),
            Material::Metallic(metallic) => metallic.scatter(ray, collision),
        }
    }
}

make_from!(Dielectric, Material);
make_from!(Lambertian, Material);
make_from!(Metallic, Material);
