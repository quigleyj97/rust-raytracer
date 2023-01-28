//! Helper classes for working with ray collisions

use std::{sync::Arc};

use crate::shader::Material;
pub use self::ray::{Ray, Point, Vector};

mod ray;
pub mod sphere;
pub mod util;

/** An object representing a collision between a ray and a `RayCollidable`

The `point` is the point at which the collision occurred, `normal` is the
outward surface normal at the point of collision, and `t` is the distance
along the ray that the collision occurred.
 */
pub struct Collision {
    pub point: Point,
    pub normal: Vector,
    pub t: f64,
    pub material: Arc<dyn Material>
}

// note to self: remember we aren't calculating face side here, we need to do
// that at raster time

pub trait RayCollidable {
    //! Tests if the given ray will intersect self between t_min and t_max, where
    //! t is the distance along the ray from it's origin.
    //! 
    //! If a collision will happen, return a struct with the collision point,
    //! the distance along the ray it happened, and the normal to self at that
    //! point.
    //! 
    //! If a collision will not happen, return None
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}
