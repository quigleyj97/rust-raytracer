use crate::{
    geometry::{
        aabb::AABB,
        ray::Ray,
        raycollidable::{Collision, RayCollidable},
        Geometry,
    },
    shader::{prelude::*, Color},
};

/// A Geometry that can be rendered
#[derive(Clone, PartialEq, Debug)]
pub struct RenderableGeometry {
    pub geometry: Geometry,
    pub material: Material,
}

pub enum RayColorResult {
    /// No collision with this object occurred
    NoHit,
    Bounce(
        /// The apparent color of this object to apply to the bounce ray
        Color,
        /// The reflected (or refracted) bounce ray to cast next
        Ray,
    ),
    Absorb(
        /// The apparent color of this object at the collision site
        Color,
    ),
}
pub trait Renderable {
    fn ray_color(&self, ray: &Ray, min_clip: f64, max_clip: f64) -> RayColorResult;
}

impl Renderable for RenderableGeometry {
    fn ray_color(&self, ray: &Ray, min_clip: f64, max_clip: f64) -> RayColorResult {
        return match self.will_intersect(ray, min_clip, max_clip) {
            None => RayColorResult::NoHit,
            Some(collision) => match self.material.scatter(ray, &collision) {
                ScatterResult::Bounce(color, scatter_ray) => {
                    RayColorResult::Bounce(color, scatter_ray)
                }
                ScatterResult::Absorb(color) => RayColorResult::Absorb(color),
            },
        };
    }
}

impl RayCollidable for RenderableGeometry {
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        self.geometry.will_intersect(ray, t_min, t_max)
    }

    fn get_bounds(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        self.geometry.get_bounds(time_start, time_end)
    }
}

impl RenderableGeometry {
    pub fn new(geometry: Geometry, material: Material) -> Self {
        Self { geometry, material }
    }
}
