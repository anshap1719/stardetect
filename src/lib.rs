use image::{DynamicImage, GenericImageView, GrayImage};
use rayon::prelude::*;

pub use error::Error;

pub mod error;
pub mod io;

#[derive(Debug, Eq, PartialEq)]
pub struct GrayscaleHistogram(pub [usize; 255]);

pub fn get_histogram(image: &DynamicImage) -> GrayscaleHistogram {
    let binary_image = image.to_luma8();
    let mut histogram_data = [0usize; 255];

    for pixel in binary_image.iter() {
        histogram_data[*pixel as usize] += 1;
    }

    GrayscaleHistogram(histogram_data)
}

const INITIAL_THRESHOLD: u8 = 230;

pub fn binarize_image(image: &mut DynamicImage) {
    match image {
        DynamicImage::ImageLuma8(image) => {
            let scaled_threshold = INITIAL_THRESHOLD;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageLumaA8(image) => {
            let scaled_threshold = INITIAL_THRESHOLD;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgb8(image) => {
            let scaled_threshold = INITIAL_THRESHOLD;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u8::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgba8(image) => {
            let scaled_threshold = INITIAL_THRESHOLD;

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
            let scaled_threshold = INITIAL_THRESHOLD as u16 * scale_factor;

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
            let scaled_threshold = INITIAL_THRESHOLD as u16 * scale_factor;

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
            let scaled_threshold = INITIAL_THRESHOLD as u16 * scale_factor;

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
            let scaled_threshold = INITIAL_THRESHOLD as u16 * scale_factor;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = u16::MAX;
                } else {
                    *pixel = 0;
                }
            }
        }
        DynamicImage::ImageRgb32F(image) => {
            let scale_factor = u32::MAX / u8::MAX as u32;
            let scaled_threshold = (INITIAL_THRESHOLD as f32 * scale_factor as f32) / f32::MAX;

            for pixel in image.iter_mut() {
                if *pixel > scaled_threshold {
                    *pixel = 1.;
                } else {
                    *pixel = 0.;
                }
            }
        }
        DynamicImage::ImageRgba32F(image) => {
            let scale_factor = u32::MAX / u8::MAX as u32;
            let scaled_threshold = (INITIAL_THRESHOLD as f32 * scale_factor as f32) / f32::MAX;

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

pub fn filter_stars(image: &DynamicImage) -> (GrayImage, GrayImage, GrayImage) {
    let images = match image {
        DynamicImage::ImageLuma8(image) => {
            todo!()
        }
        DynamicImage::ImageLumaA8(image) => {
            todo!()
        }
        DynamicImage::ImageRgb8(image) => {
            let image_buffer_size = (image.width() * image.height()) as usize;
            let mut red_stars = Vec::with_capacity(image_buffer_size);
            let mut green_stars = Vec::with_capacity(image_buffer_size);
            let mut blue_stars = Vec::with_capacity(image_buffer_size);

            for pixel in image.pixels() {
                red_stars.push(pixel.0[0]);
                green_stars.push(pixel.0[1]);
                blue_stars.push(pixel.0[2]);
            }

            let red_stars = GrayImage::from_raw(image.width(), image.height(), red_stars).unwrap();
            let green_stars =
                GrayImage::from_raw(image.width(), image.height(), green_stars).unwrap();
            let blue_stars =
                GrayImage::from_raw(image.width(), image.height(), blue_stars).unwrap();

            (red_stars, green_stars, blue_stars)
        }
        DynamicImage::ImageRgba8(image) => {
            todo!()
        }
        DynamicImage::ImageLuma16(image) => {
            todo!()
        }
        DynamicImage::ImageLumaA16(image) => {
            todo!()
        }
        DynamicImage::ImageRgb16(image) => {
            todo!()
        }
        DynamicImage::ImageRgba16(image) => {
            todo!()
        }
        DynamicImage::ImageRgb32F(image) => {
            todo!()
        }
        DynamicImage::ImageRgba32F(image) => {
            todo!()
        }
        image => {
            let image_buffer_size = (image.width() * image.height()) as usize;
            let mut red_stars = Vec::with_capacity(image_buffer_size);
            let mut green_stars = Vec::with_capacity(image_buffer_size);
            let mut blue_stars = Vec::with_capacity(image_buffer_size);

            for pixel in image.pixels() {
                red_stars.push(pixel.2[0]);
                green_stars.push(pixel.2[1]);
                blue_stars.push(pixel.2[2]);
            }

            let red_stars = GrayImage::from_raw(image.width(), image.height(), red_stars).unwrap();
            let green_stars =
                GrayImage::from_raw(image.width(), image.height(), green_stars).unwrap();
            let blue_stars =
                GrayImage::from_raw(image.width(), image.height(), blue_stars).unwrap();

            (red_stars, green_stars, blue_stars)
        }
    };

    images
}

#[cfg(test)]
mod test {
    use crate::io::read_image;
    use crate::{get_histogram, GrayscaleHistogram};

    #[test]
    fn sample_image_histogram() {
        let image = read_image("./sample.jpg").unwrap();
        assert_eq!(
            get_histogram(&image),
            GrayscaleHistogram([
                0, 0, 1, 6, 15, 25, 51, 102, 185, 377, 635, 975, 1559, 2227, 2988, 3892, 4777,
                5561, 6535, 7874, 8949, 10586, 12474, 14725, 16676, 18934, 20892, 21971, 23433,
                24314, 24957, 25594, 26639, 26752, 27714, 28607, 29465, 31132, 32460, 33860, 35656,
                37144, 37969, 38332, 37178, 35517, 33523, 31812, 29325, 27126, 25006, 22946, 21664,
                20265, 18694, 17153, 16058, 14387, 13529, 12617, 11811, 10938, 10076, 9248, 8565,
                7678, 6912, 6514, 5860, 5470, 5126, 4704, 4323, 4145, 3784, 3512, 3244, 3029, 2911,
                2856, 2651, 2468, 2406, 2321, 2237, 2155, 2081, 1963, 1953, 1925, 1822, 1797, 1732,
                1604, 1619, 1533, 1467, 1424, 1430, 1357, 1310, 1326, 1256, 1223, 1168, 1117, 1170,
                1050, 1039, 1025, 950, 981, 997, 922, 923, 910, 873, 826, 789, 789, 824, 743, 789,
                715, 734, 720, 684, 644, 623, 614, 590, 602, 591, 595, 616, 570, 495, 555, 550,
                530, 452, 490, 464, 472, 467, 463, 460, 434, 418, 416, 394, 367, 389, 384, 402,
                393, 370, 370, 411, 377, 360, 361, 372, 302, 367, 341, 353, 281, 283, 333, 261,
                276, 268, 287, 267, 257, 227, 256, 211, 230, 191, 169, 210, 188, 145, 144, 135,
                128, 107, 123, 126, 95, 99, 104, 97, 85, 93, 78, 94, 70, 56, 55, 67, 51, 61, 64,
                53, 60, 61, 54, 42, 38, 48, 47, 44, 31, 34, 31, 38, 27, 26, 26, 23, 17, 18, 18, 18,
                16, 15, 16, 14, 9, 11, 10, 8, 6, 7, 9, 7, 5, 2, 2, 4, 1, 0, 1, 0, 1, 0, 0, 2, 0, 0,
                0, 0
            ])
        );
    }
}
