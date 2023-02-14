use super::{Point, Vector};

/// A ray that can be cast into a scene.
pub struct Ray {
    /// The point at which this ray originates
    pub origin: Point,
    /// The direction in which this ray points
    pub direction: Vector,
    /// The time that this ray was cast, in scene-time
    pub time: f64,
}

impl Ray {
    pub fn point_at(&self, distance: f64) -> Point {
        return self.origin + distance * self.direction;
    }

    pub fn new(origin: Point, direction: Vector, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }
}
