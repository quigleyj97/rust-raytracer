use std::{sync::Arc, thread::JoinHandle, time::SystemTime};

use cgmath::{point3, vec3, Deg, InnerSpace};
use log::{debug, info};
use raytracer_core::{
    image::{
        blend::{self, BlendingMode},
        buffer::ImageBuffer,
        ppm,
    },
    render::{camera::Camera, iter::ChunkedPixelIterator, renderer::Renderer},
    scene,
};

fn main() {
    pretty_env_logger::init();
    const THREADS: usize = 8;
    const WIDTH: usize = 1280;
    // const WIDTH: usize = 720;
    const HEIGHT: usize = 720;
    // const HEIGHT: usize = 405;
    const SAMPLES_PER_PIXEL: usize = 1;
    const MAX_RAY_DEPTH: i64 = 4;

    debug!("Output dimensions: {} x {}", WIDTH, HEIGHT);

    info!("Rendering image...");
    let start = SystemTime::now();

    let mut threadpool = Vec::<JoinHandle<ImageBuffer>>::new();

    let scene = Arc::new(scene::new_random_world());

    for chunk in ChunkedPixelIterator::with_chunks(WIDTH, HEIGHT, THREADS) {
        info!("Spawning thread...");
        // make a copy of the world specific to each thread
        // this helps the borrow checker see the move into the thread, without
        // having it try to move the top-level object.
        let local_scene = scene.clone();
        threadpool.push(std::thread::spawn(move || -> ImageBuffer {
            let camera_position = point3(13.0, 2.0, 3.0);
            let look_at = point3(0.0, 0.0, 0.0);
            let camera = Camera::new(
                camera_position,
                look_at,
                vec3(0.0, 1.0, 0.0),
                WIDTH as f64 / HEIGHT as f64,
                Deg(20.0),
                22.0,
                (look_at - camera_position).magnitude(),
            );
            let renderer = Renderer::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_RAY_DEPTH, camera);
            let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);
            renderer.render_to_buffer(&local_scene, &mut buf, chunk);
            buf
        }));
    }

    let images = threadpool
        .into_iter()
        .map(move |i| {
            let buf = i.join();
            buf.unwrap()
        })
        .collect();
    let result_image = blend::blend_images(images, BlendingMode::Add);

    let end = SystemTime::now();
    let result = ppm::make_image(&result_image.data, result_image.width, result_image.height);
    info!(
        "Rendering took {} ms",
        end.duration_since(start).expect("you doltz").as_millis()
    );
    println!("{}", result);
}
