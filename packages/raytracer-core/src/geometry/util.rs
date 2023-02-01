pub mod vector {
    use cgmath::{vec3, InnerSpace};

    use crate::geometry::{Ray, Vector};

    /// Returns a vector of 3 random values in [0, 1)
    #[inline(always)]
    pub fn random_vector() -> Vector {
        let rng = fastrand::Rng::new();
        vec3(rng.f64(), rng.f64(), rng.f64())
    }

    #[inline(always)]
    pub fn random_vector_in_unit_sphere() -> Vector {
        let mut direction: Vector;
        loop {
            direction = random_vector();
            if direction.magnitude2() < 1.0 {
                break;
            }
        }
        direction
    }

    #[inline(always)]
    pub fn random_unit_vector() -> Vector {
        random_vector_in_unit_sphere().normalize()
    }

    #[inline(always)]
    pub fn random_vector_in_disk() -> Vector {
        let rng = fastrand::Rng::new();
        let mut x: f64;
        let mut y: f64;
        loop {
            x = rng.f64();
            y = rng.f64();
            let mag = x * x + y * y;
            if mag < 1.0 {
                break;
            }
        }
        vec3(x, y, 0.0)
    }

    #[inline(always)]
    pub fn near_zero(vector: Vector) -> bool {
        const EPSILON: f64 = 1e-8;
        return cgmath::dot(vector, vector) < EPSILON;
    }

    /// Given an outward normal, return a corrected face normal.
    ///
    /// The raytracing engine by default does not calculate face normals, and
    /// the normals on the Collision record are outward normals.
    #[inline(always)]
    pub fn to_face_normal(ray: Ray, outward_normal: Vector) -> Vector {
        let is_front_face = cgmath::dot(ray.direction, outward_normal) < 0.0;
        return if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
