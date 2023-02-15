use cgmath::{vec3, InnerSpace};

use crate::{
    geometry::{ray::Ray, raycollidable::RayCollidable},
    render::renderable::{RayColorResult, Renderable, RenderableGeometry},
    shader::Color,
};

#[derive(Clone, PartialEq, Debug)]
pub struct SceneGraph {
    pub objects: Vec<RenderableGeometry>,
}

impl SceneGraph {
    /// Cast a ray into the scene and render it
    pub fn cast_ray(&self, ray: &Ray, min_clip: f64, max_clip: f64) -> RayColorResult {
        let mut closest_hit = None;
        let mut t_max = max_clip;

        for object in &self.objects {
            match object.will_intersect(ray, min_clip, t_max) {
                None => {}
                Some(i_collision) => {
                    t_max = i_collision.t;
                    closest_hit = Some((object, i_collision));
                }
            }
        }

        closest_hit.map_or_else(
            || RayColorResult::Absorb(Self::color_bg(ray)),
            |(renderable, collision)| renderable.ray_color(ray, &collision),
        )
    }

    fn color_bg(ray: &Ray) -> Color {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
    }

    // fn _get_bounds(&self, time_start: f64, time_end: f64) -> Option<AABB> {
    //     if self.objects.is_empty() {
    //         return Option::None;
    //     }
    //     let first_object_bounds = self.objects[0].get_bounds(time_start, time_end);
    //     let mut bounds: AABB;
    //     if let Option::Some(first_obj_bounds) = first_object_bounds {
    //         bounds = first_obj_bounds
    //     } else {
    //         return Option::None;
    //     }
    //     for object in &self.objects[1..] {
    //         let object_bound = object.get_bounds(time_start, time_end);
    //         if let Option::Some(aabb) = object_bound {
    //             bounds = bounds.bounding_box(&aabb);
    //         } else {
    //             return Option::None;
    //         }
    //     }
    //     return Option::Some(bounds);
    // }
}
