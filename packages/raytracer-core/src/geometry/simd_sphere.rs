use std::simd::{f32x4, SimdFloat, StdFloat};

pub struct CollisionSIMD {
    pub point: f32x4,
    pub normal: f32x4,
    pub t: f32,
}

pub struct SIMDSphere {
    pub center: f32x4,
    pub radius: f32,
}

impl SIMDSphere {
    pub fn new(center_x: f32, center_y: f32, center_z: f32, radius: f32) -> SIMDSphere {
        SIMDSphere {
            center: f32x4::from_array([center_x, center_y, center_z, 0.0]),
            radius,
        }
    }

    #[inline(always)]
    pub fn will_intersect_simd(
        &self,
        ray_origin: f32x4,
        ray_direction: f32x4,
        t_min: f32,
        t_max: f32,
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
            let root_vec = f32x4::splat(root);
            let radius = f32x4::splat(self.radius);
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
