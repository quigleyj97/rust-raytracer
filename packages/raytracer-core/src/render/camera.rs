//! Projects rays into a space that correspond to UV screen coordinates

use cgmath::{Angle, Deg, InnerSpace};

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

    pub fn new(
        camera_position: Point,
        look_at: Point,
        local_up: Vector,
        aspect_ratio: f64,
        field_of_view: Deg<f64>,
    ) -> Camera {
        let height = -2.0 * (field_of_view / 2.0).tan();
        let width = aspect_ratio * height;

        let inverse_camera_direction = (camera_position - look_at).normalize();
        let screen_u = local_up.cross(inverse_camera_direction).normalize();
        let screen_v = inverse_camera_direction.cross(screen_u);

        let _focal_length = 1.0;

        let origin = camera_position;
        let horizontal = width * screen_u;
        let vertical = height * screen_v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - inverse_camera_direction;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
