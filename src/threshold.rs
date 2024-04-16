use image::DynamicImage;

use crate::centroid::find_star_centres_and_size;

pub(crate) trait ThresholdingExtensions {
    fn optimize_threshold_for_star_count(
        &self,
        minimum_star_count: usize,
        min_size: usize,
        max_size: usize,
    ) -> u8;
    fn binarize(&mut self, threshold: u8);
}

impl ThresholdingExtensions for DynamicImage {
    fn optimize_threshold_for_star_count(
        &self,
        minimum_star_count: usize,
        min_size: usize,
        max_size: usize,
    ) -> u8 {
        let mut star_count = 0;
        let mut threshold = 255;

        while star_count < minimum_star_count {
            if threshold == 0 {
                panic!("Maximum iteration count reached");
            }

            threshold = (0.95 * threshold as f32) as u8;

            let mut image = self.clone();
            ThresholdingExtensions::binarize(&mut image, threshold);

            star_count = find_star_centres_and_size(&image.to_luma8(), min_size, max_size).len();
        }

        threshold
    }

    fn binarize(&mut self, threshold: u8) {
        match self {
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
}
