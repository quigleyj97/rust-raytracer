pub struct PixelIterator {
    width: usize,
    #[allow(dead_code)]
    height: usize,
    idx: usize,
    max_size: usize
}

impl PixelIterator {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, idx: 0, max_size: width * height }
    }
}

pub struct Pixel {
    pub x: usize,
    pub y: usize
}

impl Iterator for PixelIterator {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        return if idx < self.max_size { Some(Pixel {
            x: idx % self.width,
            y: idx / self.width
        }) } else { 
            self.idx = 0;
            None
        }
    }
}

pub struct ChunkedPixelIterator {
    width: usize,
    height: usize,
    chunks: usize,
    current_chunk: usize,
}

impl ChunkedPixelIterator {
    pub fn with_chunks(width: usize, height: usize, n_chunks: usize) -> ChunkedPixelIterator {
        ChunkedPixelIterator { width, height, chunks: n_chunks, current_chunk: 0 }
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
                max_size: idx + chunk_size
            })
        } else {
            self.current_chunk = 0;
            Option::None
        }
    }
}