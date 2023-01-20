//! Helpers specific to WASM platforms
//! 
//! The helpers in this file are intended to be called in JS, and are made
//! available as globals on the WASM binary.
use console_error_panic_hook;
use console_log;
use log::{info, Level};
use std::panic;
use wasm_bindgen::prelude::*;
use crate::image::ppm;
use crate::render;

#[wasm_bindgen]
pub fn init_debug_hooks() {
    console_log::init_with_level(Level::Info);
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    info!("Debug hooks set.");
}

#[wasm_bindgen]
pub fn draw_scene() -> String {
    let test_image = render::render_helloworld();
    let result = ppm::make_image(&test_image.data, test_image.width, test_image.height);
    result
}