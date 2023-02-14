use std::sync::Arc;

use crate::{
    geometry::{ray::Ray, Geometry},
    shader::{Color, Material},
};

/// A Geometry that can be rendered
pub struct RenderableGeometry {
    pub geometry: Geometry,
    pub material: Arc<Material>,
}

pub trait Renderable {
    fn ray_color(ray: &Ray, min_clip: f64, max_depth: f64) -> Color;
}

impl Renderable for RenderableGeometry {
    fn ray_color(ray: &Ray, min_clip: f64, max_depth: f64) -> Color {
        todo!()
    }
}
