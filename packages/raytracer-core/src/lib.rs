#![feature(portable_simd)]

pub mod geometry;
pub mod image;
pub mod render;
pub mod scene;
pub mod shader;

#[cfg(feature = "wasm")]
mod wasm_util;

#[cfg(target = "wasm32")]
extern crate wasm_bindgen;
