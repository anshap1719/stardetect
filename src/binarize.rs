use image::DynamicImage;

pub fn binarize_image(image: &mut DynamicImage, threshold: u8) {
    match image {
        DynamicImage::ImageLuma8(image) => {
            let scaled_threshold = threshold;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageLumaA8(image) => {
            let scaled_threshold = threshold;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgb8(image) => {
            let scaled_threshold = threshold;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgba8(image) => {
            let scaled_threshold = threshold;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageLuma16(image) => {
            let scale_factor = u16::MAX / u8::MAX as u16;
            let scaled_threshold = threshold as u16 * scale_factor;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u16::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageLumaA16(image) => {
            let scale_factor = u16::MAX / u8::MAX as u16;
            let scaled_threshold = threshold as u16 * scale_factor;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u16::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgb16(image) => {
            let scale_factor = u16::MAX / u8::MAX as u16;
            let scaled_threshold = threshold as u16 * scale_factor;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u16::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgba16(image) => {
            let scale_factor = u16::MAX / u8::MAX as u16;
            let scaled_threshold = threshold as u16 * scale_factor;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u16::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgb32F(image) => {
            let scaled_threshold = threshold as f32 / u8::MAX as f32;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = 1.;
                } else {
                    *pixel = 0.;
                }
            }
        }
        DynamicImage::ImageRgba32F(image) => {
            let scaled_threshold = threshold as f32 / u8::MAX as f32;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = 1.;
                } else {
                    *pixel = 0.;
                }
            }
        }
        _ => {}
    }
}
