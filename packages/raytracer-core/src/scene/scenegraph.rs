use std::sync::Arc;

use cgmath::{point3, vec3, InnerSpace};

use crate::{
    geometry::{
        moving_sphere::MovingSphere, sphere::Sphere, Collision, Geometry, Ray, RayCollidable,
        Vector,
    },
    shader::{Dielectric, Lambertian, Material, Metallic},
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
}

pub fn new_test_world() -> SceneGraph {
    SceneGraph {
        objects: vec![
            Arc::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)).into(),
            Arc::new(Sphere::new_with_material(
                point3(0.0, -100.5, -1.0),
                100.0,
                Arc::new(Lambertian::new(vec3(0.2, 0.7, 0.1))).into(),
            ))
            .into(),
            Arc::new(Sphere::new_with_material(
                point3(-1.0, 0.0, -1.0),
                0.5,
                Arc::new(Metallic::new(vec3(0.7, 0.7, 1.0), 0.0)).into(),
            ))
            .into(),
            Arc::new(Sphere::new_with_material(
                point3(1.1, 0.0, -1.0),
                0.5,
                Arc::new(Dielectric::new(1.5)).into(),
            ))
            .into(),
        ],
    }
}

pub fn new_random_world() -> SceneGraph {
    let ground = Arc::new(Sphere::new_with_material(
        point3(0.0, -1000.0, -1.0),
        1000.0,
        Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))).into(),
    ));

    let mut objects: Vec<Geometry> = vec![ground.into()];

    let rng = fastrand::Rng::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.f64();
            let center = point3(a as f64 + 0.9 * rng.f64(), 0.2, b as f64 + 0.9 * rng.f64());

            if (center - point3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Material;
                let object: Geometry;
                if choose_material < 0.8 {
                    let albedo: Vector = vec3(rng.f64(), rng.f64(), rng.f64());
                    material = Arc::new(Lambertian::new(albedo)).into();
                    object = Arc::new(MovingSphere::new_with_material(
                        center,
                        center + vec3(0.0, fastrand::f64() / 2.0, 0.0),
                        0.2,
                        material,
                    ))
                    .into();
                } else if choose_material < 0.95 {
                    let albedo: Vector = vec3(rng.f64(), rng.f64(), rng.f64());
                    let fuzz = rng.f64() / 2.0;
                    material = Arc::new(Metallic::new(albedo, fuzz)).into();
                    object = Arc::new(Sphere::new_with_material(center, 0.2, material)).into();
                } else {
                    material = Arc::new(Dielectric::new(1.5)).into();
                    object = Arc::new(Sphere::new_with_material(center, 0.2, material)).into();
                };
                objects.push(object);
            }
        }
    }

    let glass_ball = Sphere::new_with_material(
        point3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)).into(),
    );
    objects.push(Arc::new(glass_ball).into());

    let matte_ball = Sphere::new_with_material(
        point3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))).into(),
    );
    objects.push(Arc::new(matte_ball).into());

    let metal_ball = Sphere::new_with_material(
        point3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metallic::new(vec3(0.7, 0.6, 0.5), 0.0)).into(),
    );
    objects.push(Arc::new(metal_ball).into());

    SceneGraph { objects }
}
