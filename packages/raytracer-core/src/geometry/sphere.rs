use super::{ray::{Point, Ray}, RayCollidable, Collision};

pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl RayCollidable for Sphere {
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc_segment = ray.origin - self.center;
        let a = cgmath::dot(ray.direction, ray.direction);
        let half_b = cgmath::dot(oc_segment, ray.direction);
        let c = cgmath::dot(oc_segment, oc_segment) - self.radius * self.radius;
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
                    return Option::None
                }
            }
            let point = ray.point_at(root);
            let normal = (point - self.center) / self.radius;
            Option::Some(Collision {
                t: root,
                point,
                normal
            })
        }
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}