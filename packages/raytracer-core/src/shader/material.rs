use std::sync::Arc;

use crate::geometry::{Collision, Ray, Vector};

use super::{Dielectric, Lambertian, Metallic};

pub trait MaterialTrait {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)>;
}

#[derive(Clone)]
pub enum Material {
    Dielectric(Arc<Dielectric>),
    Lambertian(Arc<Lambertian>),
    Metallic(Arc<Metallic>),
}

impl MaterialTrait for Material {
    #[inline(always)]
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)> {
        match self {
            Material::Dielectric(dielectric) => dielectric.scatter(ray, collision),
            Material::Lambertian(lambertian) => lambertian.scatter(ray, collision),
            Material::Metallic(metallic) => metallic.scatter(ray, collision),
        }
    }
}

impl From<Arc<Dielectric>> for Material {
    fn from(value: Arc<Dielectric>) -> Self {
        Self::Dielectric(value)
    }
}
impl From<Arc<Lambertian>> for Material {
    fn from(value: Arc<Lambertian>) -> Self {
        Self::Lambertian(value)
    }
}
impl From<Arc<Metallic>> for Material {
    fn from(value: Arc<Metallic>) -> Self {
        Self::Metallic(value)
    }
}
