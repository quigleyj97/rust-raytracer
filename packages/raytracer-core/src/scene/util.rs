use cgmath::{point3, vec3, InnerSpace};

use crate::{
    geometry::{moving_sphere::MovingSphere, sphere::Sphere, Geometry, Vector},
    render::renderable::RenderableGeometry,
    shader::prelude::*,
};

use super::scenegraph::SceneGraph;

/// Generates a randomly arranged scene
pub fn new_random_world() -> SceneGraph {
    let ground = RenderableGeometry::new(
        Sphere::new(point3(0.0, -1000.0, -1.0), 1000.0).into(),
        Lambertian::new(vec3(0.5, 0.5, 0.5)).into(),
    );

    let mut objects: Vec<RenderableGeometry> = vec![ground.into()];

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
                    material = Lambertian::new(albedo).into();
                    object = MovingSphere::new(
                        center,
                        center + vec3(0.0, fastrand::f64() / 2.0, 0.0),
                        0.2,
                    )
                    .into();
                } else if choose_material < 0.95 {
                    let albedo: Vector = vec3(rng.f64(), rng.f64(), rng.f64());
                    let fuzz = rng.f64() / 2.0;
                    material = (Metallic::new(albedo, fuzz)).into();
                    object = Sphere::new(center, 0.2).into();
                } else {
                    material = (Dielectric::new(1.5)).into();
                    object = Sphere::new(center, 0.2).into();
                };
                objects.push(RenderableGeometry::new(object, material));
            }
        }
    }

    let glass_ball = RenderableGeometry::new(
        Sphere::new(point3(0.0, 1.0, 0.0), 1.0).into(),
        Dielectric::new(1.5).into(),
    );
    objects.push(glass_ball);

    let matte_ball = RenderableGeometry::new(
        Sphere::new(point3(-4.0, 1.0, 0.0), 1.0).into(),
        Lambertian::new(vec3(0.4, 0.2, 0.1)).into(),
    );
    objects.push(matte_ball);

    let metal_ball = RenderableGeometry::new(
        Sphere::new(point3(4.0, 1.0, 0.0), 1.0).into(),
        Metallic::new(vec3(0.7, 0.6, 0.5), 0.0).into(),
    );
    objects.push(metal_ball);

    SceneGraph { objects }
}
