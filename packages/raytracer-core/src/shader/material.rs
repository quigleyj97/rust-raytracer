use crate::geometry::{Ray, Collision, Vector};

pub trait Material {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector, Ray)>;
}
