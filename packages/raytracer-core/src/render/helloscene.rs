use crate::image::buffer::ImageBuffer;

pub fn render_helloworld() -> ImageBuffer {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    let mut buf = ImageBuffer::new_rgb(WIDTH, HEIGHT);

    let width = buf.width;
    let height = buf.height;

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width) + x) * 3;
            if ((x / 32) % 2) > 0 {
                buf.data[idx] = 255
            }
            if ((y / 32) % 2) > 0 {
                buf.data[idx + 2] = 255
            }
        }
    }

    buf
}