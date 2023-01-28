//! Projects rays into a space that correspond to UV screen coordinates

use cgmath::{point3, vec3, Angle, Deg};

use crate::geometry::{Point, Ray, Vector};
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    /// Project a ray into space from a UV screenspace coordinate
    pub fn project_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn new(aspect_ratio: f64, field_of_view: Deg<f64>) -> Camera {
        let height = -2.0 * (field_of_view / 2.0).cos();
        let width = aspect_ratio * height;

        let focal_length = 1.0;

        let origin = point3(0.0, 0.0, 0.0);
        let horizontal = vec3(width, 0.0, 0.0);
        let vertical = vec3(0.0, height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
