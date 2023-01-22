use std::time::SystemTime;

use log::info;
use raytracer_core::{image::ppm, render};

fn main() {
    pretty_env_logger::init();
    info!("Rendering image...");
    let start = SystemTime::now();
    let test_image = render::render_helloworld();
    let end = SystemTime::now();
    let result = ppm::make_image(&test_image.data, test_image.width, test_image.height);
    info!("Rendering took {} ms", end.duration_since(start).expect("you doltz").as_millis());
    println!("{}", result);
}
