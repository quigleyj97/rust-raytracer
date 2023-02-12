/// TODO: I don't like this abstraction, but it's easier for hacking at it
use std::sync::Arc;

use cgmath::{vec3, ElementWise, InnerSpace};

use crate::shader::{Lambertian, Material};

use super::{
    aabb::AABB,
    ray::{Point, Ray},
    Collision, RayCollidable,
};

/// Almost identical to Sphere, but it moves from start a distance of (end - start) every 1 screen second
pub struct MovingSphere {
    pub center_start: Point,
    pub center_end: Point,
    pub radius: f64,
    pub material: Material,
}

impl RayCollidable for MovingSphere {
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc_segment = ray.origin - self.center(ray.time);
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
            let normal = (point - self.center(ray.time)) / self.radius;
            let material = self.material.clone();
            Option::Some(Collision {
                t: root,
                point,
                normal,
                material,
            })
        }
    }

    fn get_bounds(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        let box_at_t0: AABB;
        let box_at_t1: AABB;

        let box_start_at_t0 = self.center(time_start).sub_element_wise(self.radius);
        let box_end_at_t0 = self.center(time_start).add_element_wise(self.radius);
        box_at_t0 = AABB::new(box_start_at_t0, box_end_at_t0);

        let box_start_at_t1 = self.center(time_end).sub_element_wise(self.radius);
        let box_end_at_t1 = self.center(time_end).add_element_wise(self.radius);
        box_at_t1 = AABB::new(box_start_at_t1, box_end_at_t1);
        return Option::Some(AABB::bounding_box(&box_at_t0, &box_at_t1));
    }
}

impl MovingSphere {
    #[inline(always)]
    fn center(&self, time: f64) -> Point {
        self.center_start + ((self.center_end - self.center_start) * time)
    }

    pub fn new(center_start: Point, center_end: Point, radius: f64) -> Self {
        let material = Lambertian::new(vec3(1.0, 0.0, 0.0));
        Self::new_with_material(center_start, center_end, radius, Arc::new(material).into())
    }

    pub fn new_with_material(
        center_start: Point,
        center_end: Point,
        radius: f64,
        material: Material,
    ) -> Self {
        Self {
            center_start,
            center_end,
            radius,
            material,
        }
    }
}
