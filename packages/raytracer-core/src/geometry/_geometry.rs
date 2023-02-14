use super::{
    aabb::AxisAlignedBoundingBox,
    moving_sphere::MovingSphere,
    ray::Ray,
    raycollidable::{Collision, RayCollidable},
    sphere::Sphere,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Geometry {
    Sphere(Box<Sphere>),
    MovingSphere(Box<MovingSphere>),
}

impl RayCollidable for Geometry {
    #[inline(always)]
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        match self {
            Self::Sphere(sphere) => sphere.will_intersect(ray, t_min, t_max),
            Self::MovingSphere(sphere) => sphere.will_intersect(ray, t_min, t_max),
        }
    }

    fn get_bounds(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        match self {
            Self::MovingSphere(sphere) => sphere.get_bounds(time_start, time_end),
            Self::Sphere(sphere) => sphere.get_bounds(time_start, time_end),
        }
    }
}

macro_rules! make_from {
    ($geoType:ident) => {
        impl From<$geoType> for Geometry {
            fn from(value: $geoType) -> Self {
                Self::$geoType(Box::new(value))
            }
        }

        impl From<Box<$geoType>> for Geometry {
            fn from(value: Box<$geoType>) -> Self {
                Self::$geoType(value)
            }
        }
    };
}

make_from!(Sphere);
make_from!(MovingSphere);

#[cfg(test)]
mod tests {
    use cgmath::point3;

    use super::*;

    #[test]
    fn when_from_given_sphere_returns_geometry() {
        let test_sphere = Sphere::new(point3(0.0, 0.0, 0.0), 1.0);
        let test_geometry: Geometry = test_sphere.clone().into();
        assert_eq!(
            test_geometry.get_bounds(0.0, 1.0),
            test_sphere.get_bounds(0.0, 1.0)
        );
    }
}
