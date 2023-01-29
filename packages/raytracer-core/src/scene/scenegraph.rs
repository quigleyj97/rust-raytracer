use std::sync::Arc;

use cgmath::{point3, vec3, InnerSpace};

use rand::prelude::*;

use crate::{
    geometry::{sphere::Sphere, Collision, Ray, RayCollidable, Vector},
    shader::{Dielectric, Lambertian, Material, Metallic},
};

#[derive(Clone)]
pub struct SceneGraph {
    objects: Vec<Arc<dyn RayCollidable + Send + Sync>>,
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
}

pub fn new_test_world() -> SceneGraph {
    SceneGraph {
        objects: vec![
            Arc::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)),
            Arc::new(Sphere::new_with_material(
                point3(0.0, -100.5, -1.0),
                100.0,
                Arc::new(Lambertian::new(vec3(0.2, 0.7, 0.1))),
            )),
            Arc::new(Sphere::new_with_material(
                point3(-1.0, 0.0, -1.0),
                0.5,
                Arc::new(Metallic::new(vec3(0.7, 0.7, 1.0), 0.0)),
            )),
            Arc::new(Sphere::new_with_material(
                point3(1.1, 0.0, -1.0),
                0.5,
                Arc::new(Dielectric::new(1.5)),
            )),
        ],
    }
}

pub fn new_random_world() -> SceneGraph {
    let ground = Arc::new(Sphere::new_with_material(
        point3(0.0, -1000.0, -1.0),
        1000.0,
        Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))),
    ));

    let mut objects: Vec<Arc<dyn RayCollidable + Send + Sync>> = vec![ground];

    let mut rand = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rand.gen();
            let center = point3(
                a as f64 + 0.9 * rand.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rand.gen::<f64>(),
            );

            if (center - point3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Arc<dyn Material + Send + Sync>;
                if choose_material < 0.8 {
                    let albedo: Vector = vec3(rand.gen(), rand.gen(), rand.gen());
                    material = Arc::new(Lambertian::new(albedo));
                } else if choose_material < 0.95 {
                    let albedo: Vector = vec3(rand.gen(), rand.gen(), rand.gen());
                    let fuzz = rand.gen::<f64>() / 2.0;
                    material = Arc::new(Metallic::new(albedo, fuzz));
                } else {
                    material = Arc::new(Dielectric::new(1.5));
                };
                let object = Sphere::new_with_material(center, 0.2, material);
                objects.push(Arc::new(object));
            }
        }
    }

    let glass_ball =
        Sphere::new_with_material(point3(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)));
    objects.push(Arc::new(glass_ball));

    let matte_ball = Sphere::new_with_material(
        point3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))),
    );
    objects.push(Arc::new(matte_ball));

    let metal_ball = Sphere::new_with_material(
        point3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metallic::new(vec3(0.7, 0.6, 0.5), 0.0)),
    );
    objects.push(Arc::new(metal_ball));

    SceneGraph { objects }
}
