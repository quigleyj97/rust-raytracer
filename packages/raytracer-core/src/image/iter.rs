//! # Pixel Indexers
//!
//! ## Overview
//!
//! Pixel indexers are utilities for iterating over flat buffers without having
//! to convert between X/Y coordinates and indices. Indexers implement Iterator
//! and can be used in Rust for loops.
//!
//! ### `PixelIndexer`
//!
//! The simplest case of pixel indexer starts at `idx` and advances until
//! `max_size`. In screen space, it starts from the top left and moves left
//! to right, top to bottom.
//!
//! ```no_run
//! use raytracer_core::image::iter::PixelIndexer;
//!
//! let indexer = PixelIndexer::with_dimensions(64, 64);
//! for pixel in indexer {
//!     println!("X: {}, Y: {}, idx: {}", pixel.x, pixel.y, pixel.idx);
//! }
//! ```
//!
//! ### `ChunkedPixelIndexer`
//!
//! This method creates an Iterator of chunks breaks up the screen into N
//! chunks, and yields PixelIndexers that have starts and ends corresponding to
//! chunk boundaries.
//!
//! This is primarily useful for multithreading- chunks can be generated
//! independently, and can be moved to the owning thread.
//!
//! ```
//! use raytracer_core::image::iter::ChunkedPixelIndexer;
//!
//! for chunk in ChunkedPixelIndexer::with_chunks(200, 200, 8) {
//!     println!("Start: {} / End: {}", chunk.idx, chunk.max_size);
//! }
//! ```

use super::buffer::ImageBuffer;

#[derive(Clone, PartialEq, Debug)]
pub struct PixelIndexer {
    /// The width of the buffer being iterated over
    pub width: usize,
    /// The height of the buffer being iterated over
    pub height: usize,
    /// The index of the pixel this iterator points to
    pub idx: usize,
    /// The total size of the buffer, in pixels
    pub max_size: usize,
}

impl PixelIndexer {
    /// Given a buffer, return a PixelIterator to iterate through that buffer
    pub fn new_from_buffer(buffer: &ImageBuffer) -> Self {
        Self::with_dimensions(buffer.width, buffer.height)
    }

    /// Given a width and height, return a pixel iterator over those dimensions
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            idx: 0,
            max_size: width * height,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Pixel {
    /// The x-coordinate of this pixel, from the top left
    pub x: usize,
    /// The y-coordinate of this pixel, from the top left
    pub y: usize,
    /// The index of this pixel in the underlying buffer
    pub idx: usize,
}

impl Iterator for PixelIndexer {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        return if idx < self.max_size {
            Some(Pixel {
                x: idx % self.width,
                y: idx / self.width,
                idx,
            })
        } else {
            self.idx = 0;
            None
        };
    }
}

/// A iterator that yields chunked PixelIndexers
#[derive(Clone, PartialEq, Debug)]
pub struct ChunkedPixelIndexer {
    pub width: usize,
    pub height: usize,
    pub chunks: usize,
    pub current_chunk: usize,
}

impl ChunkedPixelIndexer {
    pub fn with_chunks(width: usize, height: usize, n_chunks: usize) -> ChunkedPixelIndexer {
        ChunkedPixelIndexer {
            width,
            height,
            chunks: n_chunks,
            current_chunk: 0,
        }
    }
}

impl Iterator for ChunkedPixelIndexer {
    type Item = PixelIndexer;

    fn next(&mut self) -> Option<Self::Item> {
        let current_chunk = self.current_chunk;
        self.current_chunk += 1;
        let max_size = self.width * self.height;
        let chunk_size = max_size / self.chunks;

        let idx = chunk_size * current_chunk;
        return if idx < max_size {
            Option::Some(PixelIndexer {
                width: self.width,
                height: self.height,
                idx,
                max_size: idx + chunk_size,
            })
        } else {
            self.current_chunk = 0;
            Option::None
        };
    }
}

#[cfg(test)]
mod pixelindexer_tests {
    use super::*;

    #[test]
    fn new_given_valid_dimensions_returns_iterator() {
        let iterator = PixelIndexer::with_dimensions(5, 5);
        let pixels: Vec<Pixel> = iterator.take(25).collect();
        assert_eq!(pixels[4], Pixel { x: 4, y: 0, idx: 4 });
        assert_eq!(
            pixels[20],
            Pixel {
                x: 0,
                y: 4,
                idx: 20
            }
        );
    }
}

#[cfg(test)]
mod chunked_tests {
    use super::*;

    #[test]
    fn with_chunks_returns_correct_number_of_chunks() {
        let indexer = ChunkedPixelIndexer::with_chunks(200, 200, 10);
        assert_eq!(indexer.collect::<Vec<_>>().len(), 10);
    }

    #[test]
    fn with_chunks_splits_at_correct_boundaries() {
        let chunks: Vec<_> = ChunkedPixelIndexer::with_chunks(200, 200, 10).collect();

        assert_eq!(chunks[0].idx, 0);
        assert_eq!(chunks[0].max_size, 4000);
        assert_eq!(chunks[1].idx, 4000);
        assert_eq!(chunks[1].max_size, 8000);
        assert_eq!(chunks[9].idx, 36000);
        assert_eq!(chunks[9].max_size, 40000);
    }
}
