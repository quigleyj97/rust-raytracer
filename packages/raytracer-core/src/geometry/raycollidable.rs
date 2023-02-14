use super::{aabb::AxisAlignedBoundingBox, ray::Ray, Point, Vector};

/** An object representing a collision between a ray and a `RayCollidable`

The `point` is the point at which the collision occurred, `normal` is the
outward surface normal at the point of collision, and `t` is the distance
along the ray that the collision occurred.
 */
#[derive(Debug, PartialEq, Clone)]
pub struct Collision {
    /// The point at which the collision occurred
    pub point: Point,
    /// The normal vector to the point of collision
    pub normal: Vector,
    /// The distance along the input ray that the collision occurred.
    pub t: f64,
}

/// A trait for objects that are intersectable with Rays
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

    /// Return a bounding box for this object between the specified time intervals.
    fn get_bounds(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox>;
}
