use crate::centroid::{find_rgb_stars, StarCenter};
use crate::image_extensions::ImageExtensions;
use crate::io::read_image;
use crate::threshold::ThresholdingExtensions;
pub use error::Error;
use image::{DynamicImage, ImageBuffer, Rgb};
use image_dwt::kernels::LinearInterpolationKernel;
use image_dwt::rescale::RescalableImage;
use image_dwt::transform::ATrousTransform;
use imageproc::drawing::Canvas;
use std::path::Path;

pub mod centroid;
mod error;
mod image_extensions;
mod io;
mod threshold;

pub struct StarDetect {
    source: DynamicImage,
}

impl TryFrom<&Path> for StarDetect {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let image = read_image(value)?;

        Ok(Self { source: image })
    }
}

impl TryFrom<&str> for StarDetect {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let image = read_image(Path::new(value))?;

        Ok(Self { source: image })
    }
}

impl TryFrom<String> for StarDetect {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let image = read_image(Path::new(&value))?;

        Ok(Self { source: image })
    }
}

impl TryFrom<DynamicImage> for StarDetect {
    type Error = Error;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        Ok(Self { source: value })
    }
}

impl<'a> TryFrom<&'a DynamicImage> for StarDetect {
    type Error = Error;

    fn try_from(value: &'a DynamicImage) -> Result<Self, Self::Error> {
        Ok(Self {
            source: value.clone(),
        })
    }
}

impl StarDetect {
    pub fn compute_star_centers(&self) -> Vec<StarCenter> {
        let (width, height) = self.source.dimensions();
        let smallest_side = width.min(height);

        let decomposition_levels = ((smallest_side as f32).ln() as usize).min(10);

        let transform = ATrousTransform::new(
            &self.source,
            decomposition_levels,
            LinearInterpolationKernel,
        );

        let mut filtered_image: ImageBuffer<Rgb<f32>, Vec<f32>> = ImageBuffer::new(width, height);
        for layer in transform {
            match layer.pixel_scale {
                Some(scale) if scale < decomposition_levels / 2 => {
                    for (x, y, pixel) in layer.image_buffer.enumerate_pixels() {
                        let [r, g, b] = pixel.0;
                        let pixel = filtered_image.get_pixel_mut(x, y);

                        let [r_existing, g_existing, b_existing] = pixel.0;

                        *pixel = Rgb([r + r_existing, g + g_existing, b + b_existing]);
                    }
                }
                None | Some(_) => {}
            }
        }

        filtered_image.channel_wise_rescale();
        let mut filtered_image = DynamicImage::ImageRgb32F(filtered_image);

        let threshold = filtered_image.optimize_threshold_for_star_count::<500>();
        filtered_image.binarize(threshold);

        let (red, green, blue) = filtered_image.channel_wise_split();
        find_rgb_stars(&red, &green, &blue)
    }
}
