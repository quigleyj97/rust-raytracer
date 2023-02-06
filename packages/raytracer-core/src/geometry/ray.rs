use cgmath::{Point3, Vector3};

pub type Vector = Vector3<f64>;
pub type Point = Point3<f64>;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
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
