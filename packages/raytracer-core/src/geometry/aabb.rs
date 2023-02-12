use std::mem::swap;

use super::{Point, Ray};

#[derive(Clone, Debug)]
pub struct AxisAlignedBoundingBox {
    /// The corner of this AABB with the smallest coordinates in all dimensions
    pub start_point: Point,
    /// The corner of this AABB with the largest coordinates in all dimensions
    pub end_point: Point,
}

pub type AABB = AxisAlignedBoundingBox;

/// 2-tuple struct representing the interval across which a hit has occurred between a ray and an AABB
#[derive(Clone, Default, Debug)]
pub struct AABBCollision(
    /// The distance along the ray where the collision starts
    pub f64,
    /// The same, but for where the collision ends
    pub f64,
);

impl AxisAlignedBoundingBox {
    pub fn new(start_point: Point, end_point: Point) -> Self {
        Self {
            start_point,
            end_point,
        }
    }
}

impl AxisAlignedBoundingBox {
    pub fn will_intersect_aabb(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<AABBCollision> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for axis in 0..3 {
            let direction_recriprocal = 1.0 / ray.direction[axis];
            let origin = ray.origin[axis];
            let min = self.start_point[axis];
            let max = self.end_point[axis];
            let mut t0 = (min - origin) * direction_recriprocal;
            let mut t1 = (max - origin) * direction_recriprocal;

            if direction_recriprocal < 0.0 {
                swap(&mut t0, &mut t1);
            }

            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);

            if t_max <= t_min {
                return Option::None;
            }
        }

        return Option::Some(AABBCollision(t_min, t_max));
    }

    /// Given this AABB and another, return a new AABB that contains them both
    pub fn bounding_box(&self, other: &Self) -> Self {
        let min = self.start_point.zip(other.start_point, f64::min);
        let max = self.end_point.zip(other.end_point, f64::max);

        return Self {
            start_point: min,
            end_point: max,
        };
    }
}

#[cfg(test)]
mod tests {
    use cgmath::{point3, vec3};

    use crate::geometry::Vector;

    use super::*;

    const START_POINT: Point = point3(1.1, 1.1, 1.1);
    const END_POINT: Point = point3(2.2, 2.2, 2.2);
    const ORIGIN: Point = point3(0.0, 0.0, 0.0);
    const RAY_DIRECTION: Vector = vec3(1.0, 1.0, 1.0);
    const Y_BASIS: Vector = vec3(0.0, 1.0, 0.0);

    fn make_aabb() -> AABB {
        let start_point = START_POINT;
        let end_point = END_POINT;
        AABB {
            start_point,
            end_point,
        }
    }

    #[test]
    fn when_new_given_valid_data_returns_aabb() {
        let aabb = make_aabb();
        assert_eq!(aabb.start_point, START_POINT);
        assert_eq!(aabb.end_point, END_POINT);
    }

    #[test]
    fn when_will_intersect_given_colliding_ray_returns_intersection() {
        let aabb = make_aabb();
        let ray = Ray::new(ORIGIN, RAY_DIRECTION, 0.1);
        let result = aabb.will_intersect_aabb(&ray, 0.0, f64::INFINITY);
        assert!(
            result.is_some(),
            "AABB returned no collision for an intersecting ray"
        );
        let value = result.unwrap();
        assert!(
            value.0 < value.1,
            "Collision intervals are not correctly ordered"
        );
        assert_eq!(value.0, 1.1);
        assert_eq!(value.1, 2.2);
    }

    #[test]
    fn when_will_interset_given_nonintersecting_ray_returns_none() {
        let aabb = make_aabb();
        let ray = Ray::new(ORIGIN, Y_BASIS, 0.1);
        let result = aabb.will_intersect_aabb(&ray, 0.0, 1.0);
        assert!(
            result.is_none(),
            "AABB returned collision for a non-intersecting ray: {:?}",
            result.unwrap()
        );
    }

    #[test]
    fn when_bounding_box_given_other_aabb_returns_containing_aabb() {
        let aabb1 = make_aabb();
        let aabb2 = AABB {
            start_point: ORIGIN,
            end_point: START_POINT,
        };
        let result = aabb1.bounding_box(&aabb2);
        assert_eq!(
            result.start_point, ORIGIN,
            "AABB did not start from the correct place"
        );
        assert_eq!(
            result.end_point, END_POINT,
            "AABB did not end at the correct place"
        );
    }
}
