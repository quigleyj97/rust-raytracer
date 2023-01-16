use cgmath::{Vector3, Point3};

pub type Vector = Vector3<f64>;
pub type Point = Point3<f64>;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn point_at(&self, distance: f64) -> Point {
        return self.origin + distance * self.direction;
    }

    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction
        }
    }
}