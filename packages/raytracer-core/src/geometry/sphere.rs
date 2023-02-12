use std::sync::Arc;

use cgmath::{vec3, ElementWise, InnerSpace};

use crate::shader::{Lambertian, Material};

use super::{
    aabb::AxisAlignedBoundingBox,
    ray::{Point, Ray},
    Collision, RayCollidable,
};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
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
            let material = self.material.clone();
            Option::Some(Collision {
                t: root,
                point,
                normal,
                material,
            })
        }
    }

    fn get_bounds(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        return Option::Some(self.into());
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        let material = Lambertian::new(vec3(1.0, 0.0, 0.0));
        Sphere::new_with_material(center, radius, Arc::new(material).into())
    }

    pub fn new_with_material(center: Point, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
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
