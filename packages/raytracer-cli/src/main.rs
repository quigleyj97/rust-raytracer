use raytracer_core::{image::ppm, render};

fn main() {
    eprintln!("Hello, world!");
    let test_image = render::render_helloworld();
    let result = ppm::make_image(&test_image.data, test_image.width, test_image.height);
    println!("{}", result);
}
