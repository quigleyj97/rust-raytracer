pub mod vector {
    use cgmath::{vec3, InnerSpace};
    use rand::Rng;

    use crate::geometry::{Ray, Vector};

    /// Returns a vector of 3 random values in [0, 1)
    pub fn random_vector() -> Vector {
        vec3(rand::random(), rand::random(), rand::random())
    }

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

    pub fn random_unit_vector() -> Vector {
        random_vector_in_unit_sphere().normalize()
    }

    pub fn random_vector_in_disk() -> Vector {
        let mut direction: Vector;
        let mut rng = rand::thread_rng();
        loop {
            direction = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if direction.magnitude2() < 1.0 {
                break;
            }
        }
        direction
    }

    pub fn near_zero(vector: Vector) -> bool {
        const EPSILON: f64 = 1e-8;
        return cgmath::dot(vector, vector) < EPSILON;
    }

    /// Given an outward normal, return a corrected face normal.
    ///
    /// The raytracing engine by default does not calculate face normals, and
    /// the normals on the Collision record are outward normals.
    pub fn to_face_normal(ray: Ray, outward_normal: Vector) -> Vector {
        let is_front_face = cgmath::dot(ray.direction, outward_normal) < 0.0;
        return if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
