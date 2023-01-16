//! Helper class for modelling u8 image buffers with width, height, and stride.
//! 
//! Stride defaults to 3 if not provided, for an RGB format. Use 4 to include
//! an alpha channel, for eg RGBA.

#[derive(Debug)]
pub struct ImageBuffer {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub stride: usize
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize, stride: usize) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            stride,
            data: vec!(0u8; width * height * stride)
        }
    }

    pub fn new_rgb(width: usize, height: usize) -> ImageBuffer {
        Self::new(width, height, 3)
    }
}

