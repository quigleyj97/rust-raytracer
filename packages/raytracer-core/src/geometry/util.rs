pub mod vector {
    use cgmath::vec3;

    use crate::geometry::{Vector, Ray};

    /// Returns a vector of 3 random values in [0, 1)
    pub fn random_vector() -> Vector {
        vec3(rand::random(), rand::random(), rand::random())
    }

    pub fn random_unit_vector() -> Vector {
        let mut direction: Vector;
        loop {
            direction = random_vector();
            if cgmath::dot(direction, direction) < 1.0 {
                break;
            }
        }
        unit_vector(direction)
    }

    pub fn unit_vector(vector: Vector) -> Vector {
        vector / cgmath::dot(vector, vector).sqrt()
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
        return if is_front_face { outward_normal } else { -outward_normal };
    }
}
