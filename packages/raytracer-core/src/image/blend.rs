use super::buffer::ImageBuffer;

pub enum BlendingMode {
    Add,
}

pub fn blend_images(images: Vec<ImageBuffer>, blending_mode: BlendingMode) -> ImageBuffer {
    let width = images[0].width;
    let height = images[0].height;
    let stride = images[0].format.stride;
    let total_size = width * height * stride;
    // todo: generic buffer support
    let mut output = ImageBuffer::new_rgb(width, height);

    for i in 0..total_size {
        output.data[i] = match blending_mode {
            BlendingMode::Add => {
                let mut sum = 0;
                for image in &images {
                    sum += image.data[i];
                }
                sum
            }
        }
    }

    output
}
