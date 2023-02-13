use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    sync::Arc,
    thread::JoinHandle,
    time::SystemTime,
};

use cgmath::{point3, vec3, Deg, InnerSpace};
use clap::Parser;
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

#[derive(Parser)]
#[command(version)]
struct CliArguments {
    /// The number of threads to spawn
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
    /// The width of the image to render
    #[arg(short, long, default_value_t = 720)]
    width: usize,
    /// The height of the image to render
    #[arg(short, long, default_value_t = 405)]
    height: usize,
    /// The number of sample rays cast per pixel
    #[arg(short, long, default_value_t = 4)]
    samples_per_pixel: usize,
    /// The maximum number of ray bounces a sample ray can generate
    #[arg(short, long, default_value_t = 4)]
    max_ray_depth: usize,
    /// The output to write the result to. If not specified, defaults to stdout
    #[arg(short, long)]
    output_file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    let CliArguments {
        threads,
        width,
        height,
        samples_per_pixel,
        max_ray_depth,
        output_file,
    } = CliArguments::parse();

    debug!("Output dimensions: {} x {}", width, height);

    info!("Rendering image...");
    let start = SystemTime::now();

    let mut threadpool = Vec::<JoinHandle<ImageBuffer>>::new();

    let scene = Arc::new(scene::new_random_world());

    for chunk in ChunkedPixelIterator::with_chunks(width, height, threads) {
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
                width as f64 / height as f64,
                Deg(20.0),
                22.0,
                (look_at - camera_position).magnitude(),
                0.0,
                1.0,
            );
            let renderer = Renderer::new(
                width,
                height,
                samples_per_pixel,
                max_ray_depth as i64,
                camera,
            );
            let mut buf = ImageBuffer::new_rgb(width, height);
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
    info!(
        "Rendering took {} ms",
        end.duration_since(start).expect("you doltz").as_millis()
    );

    let result = ppm::make_image(&result_image.data, result_image.width, result_image.height);

    if let Some(filepath) = output_file {
        let mut file = File::create(filepath)?;
        file.write(result.as_bytes())?;
        file.flush()?;
    } else {
        let mut stdout = io::stdout();
        stdout.write(result.as_bytes())?;
        stdout.flush()?;
    };

    Ok(())
}
