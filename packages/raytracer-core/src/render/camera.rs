//! Projects rays into a space that correspond to UV screen coordinates

use cgmath::{Angle, Deg, InnerSpace};

use crate::geometry::{ray::Ray, util, Point, Vector};

/// A camera that projects sample rays into a scene
pub struct Camera {
    origin: Point,
    /// The time in scene-seconds to start casting rays from
    ///
    /// In other words, the time the shutter opened
    ///
    /// In concert with time_end, this forms the time interval across which rays
    /// will be cast into the scene. This simulates effects like motion blur.
    time_start: f64,
    /// The time in scene-seconds to stop casting rays at.
    time_end: f64,
    // The below props are all calculated
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    /// Radius of the lens used in the thin-lens bokeh
    lens_radius: f64,
    /// The horizontal basis vector in film plane (or screen) space
    screen_u: Vector,
    /// The vertical basis vector in film plane (or screen) space
    screen_v: Vector,
    /// A vector pointing away from the camera that is normal to the film plane
    _inverse_camera_direction: Vector,
}

impl Camera {
    /// Project a ray into space from a UV screenspace coordinate
    pub fn project_ray(&self, u: f64, v: f64) -> Ray {
        let Vector { x, y, z: _ } = self.lens_radius * util::random_vector_in_disk();
        let offset = self.screen_u * x + self.screen_v * y;
        let time = fastrand::f64() * (self.time_end - self.time_start) + self.time_start;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            time,
        )
    }

    pub fn new(
        camera_position: Point,
        look_at: Point,
        local_up: Vector,
        aspect_ratio: f64,
        field_of_view: Deg<f64>,
        aperture_f_stop: f64,
        focal_length: f64,
        time_start: f64,
        time_end: f64,
    ) -> Camera {
        let height = -2.0 * (field_of_view / 2.0).tan();
        let width = aspect_ratio * height;

        let inverse_camera_direction = (camera_position - look_at).normalize();
        let screen_u = local_up.cross(inverse_camera_direction).normalize();
        let screen_v = inverse_camera_direction.cross(screen_u);

        let origin = camera_position;
        let horizontal = focal_length * width * screen_u;
        let vertical = focal_length * height * screen_v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - focal_length * inverse_camera_direction;

        let lens_radius = 1.0 / (aperture_f_stop);

        Camera {
            origin,
            time_start,
            time_end,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            _inverse_camera_direction: inverse_camera_direction,
            screen_u,
            screen_v,
        }
    }
}
