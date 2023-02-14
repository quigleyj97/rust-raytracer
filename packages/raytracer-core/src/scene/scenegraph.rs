use crate::geometry::{
    aabb::AABB,
    ray::Ray,
    raycollidable::{Collision, RayCollidable},
    Geometry,
};

#[derive(Clone)]
pub struct SceneGraph {
    objects: Vec<Geometry>,
}

impl RayCollidable for SceneGraph {
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let mut collision: Option<Collision> = None;
        let mut closest_hit = t_max;

        for object in &self.objects {
            match object.will_intersect(ray, t_min, closest_hit) {
                None => {}
                Some(i_collision) => {
                    closest_hit = i_collision.t;
                    collision = Some(i_collision);
                }
            }
        }

        collision
    }

    fn get_bounds(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return Option::None;
        }
        let first_object_bounds = self.objects[0].get_bounds(time_start, time_end);
        let mut bounds: AABB;
        if let Option::Some(first_obj_bounds) = first_object_bounds {
            bounds = first_obj_bounds
        } else {
            return Option::None;
        }
        for object in &self.objects[1..] {
            let object_bound = object.get_bounds(time_start, time_end);
            if let Option::Some(aabb) = object_bound {
                bounds = bounds.bounding_box(&aabb);
            } else {
                return Option::None;
            }
        }
        return Option::Some(bounds);
    }
}
