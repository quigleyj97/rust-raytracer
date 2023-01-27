use std::rc::Rc;

use cgmath::{point3, vec3};

use crate::{geometry::{sphere::Sphere, RayCollidable, Ray, Collision}, shader::{Lambertian, Metallic}};

pub struct SceneGraph {
    objects: Vec<Box<dyn RayCollidable>>
}

impl RayCollidable for SceneGraph {
    fn will_intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let mut collision: Option<Collision> = None;
        let mut closest_hit = t_max;

        for object in &self.objects {
            match object.will_intersect(ray, t_min, closest_hit) {
                None => {},
                Some(i_collision) => {
                    closest_hit = i_collision.t;
                    collision = Some(i_collision);
                }
            }
        }

        collision
    }
}

pub fn new_test_world() -> SceneGraph {
    SceneGraph {
        objects: vec![
            Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new_with_material(
                point3(0.0, -100.5, -1.0),
                100.0,
                Rc::new(Lambertian::new(vec3(0.0, 1.0, 0.0)))
            )),
            Box::new(Sphere::new_with_material(
                point3(-1.0, 0.0, -1.0),
                0.5, 
                Rc::new(Metallic::new(vec3(0.7, 0.7, 1.0), 0.3))
            )),
            Box::new(Sphere::new_with_material(
                point3(1.1, 0.0, -1.0),
                0.5,
                Rc::new(Metallic::new(vec3(0.4, 0.4, 0.4), 0.0))
            ))
        ]
    }
}

