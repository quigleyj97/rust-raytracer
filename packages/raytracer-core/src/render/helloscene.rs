use log::{debug, info};

use crate::{image::buffer::ImageBuffer, scene::new_test_world, render::{iter::ChunkedPixelIterator, renderer::Renderer}};

pub fn render_helloworld() -> ImageBuffer {
    const WIDTH: usize = 720;
    const HEIGHT: usize = 405;

    let renderer = Renderer::new_from_defaults(WIDTH, HEIGHT);

    debug!("Output dimensions: {} x {}", WIDTH, HEIGHT);

    let scene = new_test_world();

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;
    let mut i_chunk = 0;

    for chunk in ChunkedPixelIterator::with_chunks(width, height, 10){
        info!("Rendering chunk {} of {}", i_chunk, 10);
        i_chunk += 1;
        if i_chunk % 2 == 0 {
            continue;
        }
        renderer.render_to_buffer(&scene, &mut buf, chunk);
    }

    buf
}
