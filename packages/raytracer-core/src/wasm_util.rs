//! Helpers specific to WASM platforms
//!
//! The helpers in this file are intended to be called in JS, and are made
//! available as globals on the WASM binary.
use console_error_panic_hook;
use console_log;
use log::{info, Level};
use std::panic;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
pub fn init_debug_hooks() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Info)
        .expect("Failed to set console hooks, logging may not work");
    info!("Debug hooks set.");
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn draw_scene() -> Vec<u8> {
    // let test_image = render::render_helloworld();
    // let result = buffer::convert::rgb_to_rgba(&test_image, 255);
    // result.data
    // TODO: Move WASM into it's own module
    vec![]
}
