pub mod vector {
    use cgmath::vec3;

    use crate::geometry::Vector;

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
}
