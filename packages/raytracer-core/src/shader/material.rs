use crate::geometry::{Collision, Ray, Vector};

pub trait Material {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)>;
}
