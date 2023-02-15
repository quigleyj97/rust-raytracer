use cgmath::{point3, vec3, Deg};
use log::debug;

use raytracer_core::{
    image::{buffer::ImageBuffer, iter::PixelIterator},
    render::{camera::Camera, renderer::Renderer},
    scene::util::new_random_world,
};
use sha2::{Digest, Sha256};

const TEST_IMAGE_SNAPSHOT: &str =
    "7bd64840fcf3f790fc83228ece97429cf141b80bf7e5bf079b5f58802782104e";

#[test]
pub fn renders_helloworld() {
    fastrand::seed(42);
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

    let camera = Camera::new(
        point3(0.0, 0.0, 0.0),
        point3(0.0, 0.0, -1.0),
        vec3(0.0, 1.0, 0.0),
        WIDTH as f64 / HEIGHT as f64,
        Deg(45.0),
        2.0,
        1.0,
        0.0,
        0.0,
    );

    let renderer = Renderer::new(WIDTH, HEIGHT, 1, 4, camera);

    debug!("Output dimensions: {} x {}", WIDTH, HEIGHT);

    let scene = new_random_world();

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;

    renderer.render_to_buffer(
        &scene,
        &mut buf,
        PixelIterator::with_dimensions(width, height),
    );

    let mut hasher = Sha256::new();
    hasher.update(buf.data);
    let result = hasher.finalize();

    assert_eq!(
        format!("{:x}", result),
        TEST_IMAGE_SNAPSHOT,
        "Snapshot test failed, inspect results and update snapshot if necessary",
    );
}
