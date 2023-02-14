use super::buffer::ImageBuffer;

/// Helpers for iterating through images that are stored as flat buffers,
/// starting from the top left corner to the bottom right corner.
#[derive(Clone, PartialEq, Debug)]
pub struct PixelIterator {
    /// The width of the buffer being iterated over
    pub width: usize,
    /// The height of the buffer being iterated over
    pub height: usize,
    /// The index of the pixel this iterator points to
    pub idx: usize,
    /// The total size of the buffer, in pixels
    pub max_size: usize,
}

impl PixelIterator {
    /// Given a buffer, return a PixelIterator to iterate through that buffer
    pub fn new_from_buffer(buffer: &ImageBuffer) -> Self {
        Self::new_from_dimensions(buffer.width, buffer.height)
    }

    /// Given a width and height, return a pixel iterator over those dimensions
    pub fn new_from_dimensions(width: usize, height: usize) -> Self {
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

impl Iterator for PixelIterator {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_given_valid_dimensions_returns_iterator() {
        let iterator = PixelIterator::new_from_dimensions(5, 5);
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

/// A variant of the pixel iterator that chunks the iterator.
///
/// !!deprecated
#[deprecated = "Use Iterator::take instead"]
pub struct ChunkedPixelIterator {
    width: usize,
    height: usize,
    chunks: usize,
    current_chunk: usize,
}

impl ChunkedPixelIterator {
    pub fn with_chunks(width: usize, height: usize, n_chunks: usize) -> ChunkedPixelIterator {
        ChunkedPixelIterator {
            width,
            height,
            chunks: n_chunks,
            current_chunk: 0,
        }
    }
}

impl Iterator for ChunkedPixelIterator {
    type Item = PixelIterator;

    fn next(&mut self) -> Option<Self::Item> {
        let current_chunk = self.current_chunk;
        self.current_chunk += 1;
        let max_size = self.width * self.height;
        let chunk_size = max_size / self.chunks;

        let idx = chunk_size * current_chunk;
        return if idx < max_size {
            Option::Some(PixelIterator {
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
