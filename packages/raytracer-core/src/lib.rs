pub mod image;
pub mod render;
pub mod geometry;
pub mod scene;
pub mod shader;

#[cfg(feature = "wasm")]
mod wasm_util;

#[cfg(target = "wasm32")]
extern crate wasm_bindgen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
