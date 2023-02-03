use std::simd::{f64x4, SimdFloat, StdFloat};

pub struct CollisionSIMD {
    point: f64x4,
    normal: f64x4,
    t: f64,
}

pub struct SIMDSphere {
    pub center: f64x4,
    pub radius: f64,
}

impl SIMDSphere {
    pub fn will_intersect_simd(
        &self,
        ray_origin: f64x4,
        ray_direction: f64x4,
        t_min: f64,
        t_max: f64,
    ) -> Option<CollisionSIMD> {
        let oc_segment = ray_origin - self.center;
        let a = (ray_direction * ray_direction).reduce_sum();
        let half_b = (oc_segment * ray_direction).reduce_sum();
        let c = (oc_segment * oc_segment).reduce_sum() - self.radius * self.radius;
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
            let root_vec = f64x4::splat(root);
            let radius = f64x4::splat(self.radius);
            let point = ray_direction.mul_add(root_vec, ray_origin);
            let normal = (point - self.center) / radius;
            Option::Some(CollisionSIMD {
                t: root,
                point,
                normal,
            })
        }
    }
}
