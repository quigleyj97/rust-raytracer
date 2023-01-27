use std::{time::SystemTime, thread::JoinHandle};

use log::{info, debug};
use raytracer_core::{image::{ppm, buffer::ImageBuffer, blend::{BlendingMode, self}}, render::{renderer::Renderer, iter::ChunkedPixelIterator}, scene::new_test_world};

fn main() {
    pretty_env_logger::init();
    const THREADS: usize = 8;
    const WIDTH: usize = 720;
    const HEIGHT: usize = 405;

    debug!("Output dimensions: {} x {}", WIDTH, HEIGHT);

    info!("Rendering image...");
    let start = SystemTime::now();

    let mut threadpool = Vec::<JoinHandle<ImageBuffer>>::new();

    for chunk in ChunkedPixelIterator::with_chunks(WIDTH, HEIGHT, THREADS) {
        info!("Spawning thread...");
        threadpool.push(std::thread::spawn(move || -> ImageBuffer {
            let renderer = Renderer::new_from_defaults(WIDTH, HEIGHT);
            let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);
            let scene = new_test_world();
            renderer.render_to_buffer(&scene, &mut buf, chunk);
            buf
        }));
    }

    let images = threadpool.into_iter()
        .map(move |i| {
            let buf = i.join();
            buf.unwrap()
         })
        .collect();
    let result_image = blend::blend_images(images, BlendingMode::Add);

    let end = SystemTime::now();
    let result = ppm::make_image(&result_image.data, result_image.width, result_image.height);
    info!("Rendering took {} ms", end.duration_since(start).expect("you doltz").as_millis());
    println!("{}", result);
}
