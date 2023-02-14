use cgmath::{ElementWise, InnerSpace};

use super::{
    aabb::AxisAlignedBoundingBox,
    ray::Ray,
    raycollidable::{Collision, RayCollidable},
    Point,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl RayCollidable for Sphere {
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc_segment = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = cgmath::dot(oc_segment, ray.direction);
        let c = oc_segment.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            Option::None
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            // find the nearest root in [t_min..t_max]
            let mut root = (-half_b - sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrt_discriminant) / a;
                if root < t_min || t_max < root {
                    return Option::None;
                }
            }
            let point = ray.point_at(root);
            let normal = (point - self.center) / self.radius;
            Option::Some(Collision {
                t: root,
                point,
                normal,
            })
        }
    }

    fn get_bounds(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        return Option::Some(self.into());
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl From<&Sphere> for AxisAlignedBoundingBox {
    fn from(value: &Sphere) -> Self {
        return Self {
            start_point: value.center.sub_element_wise(value.radius),
            end_point: value.center.add_element_wise(value.radius),
        };
    }
}

#[cfg(test)]
mod tests {
    use cgmath::{assert_relative_eq, point3, vec3};

    use crate::geometry::{aabb::AABB, Vector};

    use super::*;

    const ORIGIN: Point = point3(0.0, 0.0, 0.0);
    const PLUS_X: Vector = vec3(1.0, 0.0, 0.0);

    #[test]
    fn when_new_returns_sphere() {
        let sphere = Sphere::new(point3(0.0, 0.0, 0.0), 1.0);
        assert_eq!(sphere.radius, 1.0);
    }

    #[test]
    fn when_get_bounds_given_valid_sphere_returns_valid_bounds() {
        let start_point = point3(1.0, 1.0, 1.0);
        let end_point = point3(3.0, 3.0, 3.0);
        let sphere = Sphere::new(point3(2.0, 2.0, 2.0), 1.0);
        let bounds: AABB = sphere.get_bounds(0.0, 1.0).unwrap();
        assert_relative_eq!(bounds.start_point, start_point);
        assert_relative_eq!(bounds.end_point, end_point);
    }

    #[test]
    fn when_will_intersect_given_intersecting_ray_returns_intersection() {
        let ray = Ray::new(ORIGIN, PLUS_X, 0.0);
        let sphere = Sphere::new(point3(2.0, 0.0, 0.0), 0.5);
        let intersection = sphere.will_intersect(&ray, 0.0, f64::INFINITY);
        assert!(intersection.is_some());
        let data = intersection.unwrap();
        assert_relative_eq!(data.t, 1.5);
        assert_relative_eq!(data.point, point3(1.5, 0.0, 0.0));
        assert_relative_eq!(data.normal, vec3(-1.0, 0.0, 0.0));
    }

    #[test]
    fn when_will_intersect_given_noncolliding_ray_returns_none() {
        let ray = Ray::new(ORIGIN, PLUS_X, 0.0);
        let sphere = Sphere::new(point3(-2.0, 0.0, 0.0), 0.5);
        let intersection = sphere.will_intersect(&ray, 0.0, f64::INFINITY);
        assert!(intersection.is_none());
    }
}
