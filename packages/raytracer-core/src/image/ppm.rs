//! PPM is a simplistic, textual bitmap image format
//! 
//! eg,
//! ```text
//! P3
//! 3   2
//! 255
//! 255 0   0   0   255 0   0   0   255
//! 255 255 0   255 255 255 0   0   0
//! ```
//! 
//! is a valid 3pxx2px image of ASCII triplets read left-to-right, top-to-
//! bottom.

const PPM_HEADER: &str = "P3";
const PPM_BITDEPTH: usize = 255;
const IMG_STRIDE: usize = 3;

pub fn make_image(bitmap: &Vec<u8>, width: usize, height: usize) -> String {
    let header = format!("{}\n{}\t{}\n{}", PPM_HEADER, width, height, PPM_BITDEPTH);
    let mut outputs = vec!(header);
    let length = width * height;
    for i in 0..length {
        let idx = i * IMG_STRIDE;
        let r = bitmap[idx + 0];
        let g = bitmap[idx + 1];
        let b = bitmap[idx + 2];
        outputs.push(format!("{}\t{}\t{}", r, g, b));
    }
    let file = outputs.join("\n");
    return file;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IMAGE: &str = "P3
3\t2
255
255\t0\t0
0\t255\t0
0\t0\t255
255\t255\t0
255\t255\t255
0\t0\t0";

    #[test]
    fn writes_simple_image() {
        let test_bitmap = vec!(
            255, 0,   0,   0,   255, 0,   0,   0,   255,
            255, 255, 0,   255, 255, 255, 0,   0,   0,
        );
        let result = make_image(&test_bitmap, 3, 2);
        assert_eq!(result, TEST_IMAGE.to_string());
    }
}

