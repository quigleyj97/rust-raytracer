use cgmath::{point3, vec3, Deg};
use log::{debug, info};

use raytracer_core::{
    image::{buffer::ImageBuffer, iter::ChunkedPixelIterator},
    render::{camera::Camera, renderer::Renderer},
};
use sha2::{Digest, Sha256};

#[test]
pub fn renders_helloworld() {
    fastrand::seed(42);
    const WIDTH: usize = 720;
    const HEIGHT: usize = 405;

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

    let renderer = Renderer::new_from_defaults(WIDTH, HEIGHT, camera);

    debug!("Output dimensions: {} x {}", WIDTH, HEIGHT);

    // let scene = new_test_world();

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;
    let mut i_chunk = 0;

    for chunk in ChunkedPixelIterator::with_chunks(width, height, 10) {
        info!("Rendering chunk {} of {}", i_chunk, 10);
        i_chunk += 1;
        if i_chunk % 2 == 0 {
            continue;
        }
        // renderer.render_to_buffer(&scene, &mut buf, chunk);
    }

    let mut hasher = Sha256::new();
    hasher.update(buf.data);
    let result = hasher.finalize();

    panic!("{:x}", result)
}
