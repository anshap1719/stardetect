use image::{DynamicImage, GenericImageView, GrayImage};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct GrayscaleHistogram(pub [usize; 255]);

pub(crate) trait ImageExtensions {
    fn channel_wise_split(&self) -> (GrayImage, GrayImage, GrayImage);
    fn get_histogram(&self) -> GrayscaleHistogram;
}

impl ImageExtensions for DynamicImage {
    fn channel_wise_split(&self) -> (GrayImage, GrayImage, GrayImage) {
        match self {
            DynamicImage::ImageRgb8(image) => {
                let image_buffer_size = (image.width() * image.height()) as usize;

                let mut red = Vec::with_capacity(image_buffer_size);
                let mut green = Vec::with_capacity(image_buffer_size);
                let mut blue = Vec::with_capacity(image_buffer_size);

                for pixel in image.pixels() {
                    red.push(pixel.0[0]);
                    green.push(pixel.0[1]);
                    blue.push(pixel.0[2]);
                }

                let red = GrayImage::from_raw(image.width(), image.height(), red).unwrap();
                let green = GrayImage::from_raw(image.width(), image.height(), green).unwrap();
                let blue = GrayImage::from_raw(image.width(), image.height(), blue).unwrap();

                (red, green, blue)
            }
            DynamicImage::ImageRgb32F(image) => {
                let image_buffer_size = (image.width() * image.height()) as usize;

                let mut red = Vec::with_capacity(image_buffer_size);
                let mut green = Vec::with_capacity(image_buffer_size);
                let mut blue = Vec::with_capacity(image_buffer_size);

                for pixel in image.pixels() {
                    red.push((u8::MAX as f32 * pixel.0[0]) as u8);
                    green.push((u8::MAX as f32 * pixel.0[1]) as u8);
                    blue.push((u8::MAX as f32 * pixel.0[2]) as u8);
                }

                let red = GrayImage::from_raw(image.width(), image.height(), red).unwrap();
                let green = GrayImage::from_raw(image.width(), image.height(), green).unwrap();
                let blue = GrayImage::from_raw(image.width(), image.height(), blue).unwrap();

                (red, green, blue)
            }
            image => {
                let image_buffer_size = (image.width() * image.height()) as usize;
                let mut red = Vec::with_capacity(image_buffer_size);
                let mut green = Vec::with_capacity(image_buffer_size);
                let mut blue = Vec::with_capacity(image_buffer_size);

                for pixel in image.pixels() {
                    red.push(pixel.2[0]);
                    green.push(pixel.2[1]);
                    blue.push(pixel.2[2]);
                }

                let red = GrayImage::from_raw(image.width(), image.height(), red).unwrap();
                let green = GrayImage::from_raw(image.width(), image.height(), green).unwrap();
                let blue = GrayImage::from_raw(image.width(), image.height(), blue).unwrap();

                (red, green, blue)
            }
        }
    }

    fn get_histogram(&self) -> GrayscaleHistogram {
        let binary_image = self.to_luma8();
        let mut histogram_data = [0usize; 255];

        for pixel in binary_image.iter() {
            histogram_data[*pixel as usize] += 1;
        }

        GrayscaleHistogram(histogram_data)
    }
}
