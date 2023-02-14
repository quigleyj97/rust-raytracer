//! Helper classes for working with ray collisions

use cgmath::{Point3, Vector3};

pub use self::_geometry::Geometry;

pub type Vector = Vector3<f64>;
pub type Point = Point3<f64>;

mod _geometry;
pub mod aabb;
pub mod moving_sphere;
pub mod ray;
pub mod raycollidable;
pub mod sphere;
pub mod util;
