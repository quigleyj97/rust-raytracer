//! Helper class for modelling u8 image buffers with width, height, and stride.
//!
//! Stride defaults to 3 if not provided, for an RGB format. Use 4 to include
//! an alpha channel, for eg RGBA.

#[allow(non_snake_case)]
pub mod BufferFormat {
    #[derive(Debug, PartialEq)]
    pub struct Metadata {
        pub stride: usize,
    }

    pub const RGB8: Metadata = Metadata { stride: 3 };
    pub const RGBA8: Metadata = Metadata { stride: 4 };
}

#[derive(Debug, PartialEq)]
pub struct ImageBuffer {
    /* A buffer holding the data in a flat list. Read from top row, left-to-right. */
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub format: BufferFormat::Metadata,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize, format: BufferFormat::Metadata) -> ImageBuffer {
        let stride = format.stride;
        ImageBuffer {
            width,
            height,
            format,
            data: vec![0u8; width * height * stride],
        }
    }

    pub fn new_rgb(width: usize, height: usize) -> ImageBuffer {
        Self::new(width, height, BufferFormat::RGB8)
    }

    pub fn new_rgba(width: usize, height: usize) -> ImageBuffer {
        Self::new(width, height, BufferFormat::RGBA8)
    }
}

pub mod convert {
    use super::{BufferFormat, ImageBuffer};

    pub fn rgb_to_rgba(rgb_buffer: &ImageBuffer, fill: u8) -> ImageBuffer {
        assert!(
            rgb_buffer.format == BufferFormat::RGB8,
            "FROM buffer must be in RGB8"
        );
        let width = rgb_buffer.width;
        let height = rgb_buffer.height;
        let n_pixels = width * height;

        let mut new_buffer = vec![0u8; width * height * 4];

        for pixel in 0..n_pixels {
            let old_idx = pixel * 3;
            let new_idx = pixel * 4;

            new_buffer[new_idx + 0] = rgb_buffer.data[old_idx + 0];
            new_buffer[new_idx + 1] = rgb_buffer.data[old_idx + 1];
            new_buffer[new_idx + 2] = rgb_buffer.data[old_idx + 2];
            new_buffer[new_idx + 3] = fill;
        }

        ImageBuffer {
            data: new_buffer,
            width,
            height,
            format: BufferFormat::RGBA8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{convert, BufferFormat, ImageBuffer};

    #[test]
    fn given_valid_rgb8_image_when_convert_rgba8_then_returns_buffer() {
        let test_image = ImageBuffer {
            width: 3,
            height: 3,
            data: vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25,
                26, 27, 28, 29,
            ],
            format: BufferFormat::RGB8,
        };

        let reference_output = ImageBuffer {
            width: 3,
            height: 3,
            format: BufferFormat::RGBA8,
            data: vec![
                1, 2, 3, 42, 4, 5, 6, 42, 7, 8, 9, 42, 11, 12, 13, 42, 14, 15, 16, 42, 17, 18, 19,
                42, 21, 22, 23, 42, 24, 25, 26, 42, 27, 28, 29, 42,
            ],
        };

        let converted_image = convert::rgb_to_rgba(&test_image, 42);

        assert_eq!(reference_output, converted_image);
    }
}
