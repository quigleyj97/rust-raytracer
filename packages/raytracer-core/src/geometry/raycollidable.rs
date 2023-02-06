use std::sync::Arc;

use crate::shader::Material;

use super::{moving_sphere::MovingSphere, sphere::Sphere, Point, Ray, Vector};

/** An object representing a collision between a ray and a `RayCollidable`

The `point` is the point at which the collision occurred, `normal` is the
outward surface normal at the point of collision, and `t` is the distance
along the ray that the collision occurred.
 */
pub struct Collision {
    pub point: Point,
    pub normal: Vector,
    pub t: f64,
    pub material: Material,
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

#[derive(Clone)]
pub enum Geometry {
    Sphere(Arc<Sphere>),
    MovingSphere(Arc<MovingSphere>),
}

impl RayCollidable for Geometry {
    #[inline(always)]
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        match self {
            Self::Sphere(sphere) => sphere.will_intersect(ray, t_min, t_max),
            Self::MovingSphere(sphere) => sphere.will_intersect(ray, t_min, t_max),
        }
    }
}

macro_rules! make_from {
    ($geoType:ident) => {
        impl From<Arc<$geoType>> for Geometry {
            fn from(value: Arc<$geoType>) -> Self {
                Self::$geoType(value)
            }
        }
    };
}

make_from!(Sphere);
make_from!(MovingSphere);
