use crate::centroid::{find_rgb_stars, StarCenter};
use crate::image_extensions::ImageExtensions;
use crate::io::read_image;
use crate::threshold::ThresholdingExtensions;
pub use error::Error;
use image::DynamicImage;
use image_dwt::kernels::LinearInterpolationKernel;
use image_dwt::recompose::{OutputLayer, RecomposableWaveletLayers};
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
    minimum_star_count: usize,
    minimum_star_size: usize,
    maximum_star_size: usize,
}

impl TryFrom<&Path> for StarDetect {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let image = read_image(value)?;

        Ok(Self {
            source: image,
            minimum_star_count: 1000,
            minimum_star_size: 1,
            maximum_star_size: 24,
        })
    }
}

impl TryFrom<&str> for StarDetect {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let image = read_image(Path::new(value))?;

        Ok(Self {
            source: image,
            minimum_star_count: 1000,
            minimum_star_size: 1,
            maximum_star_size: 24,
        })
    }
}

impl TryFrom<String> for StarDetect {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let image = read_image(Path::new(&value))?;

        Ok(Self {
            source: image,
            minimum_star_count: 1000,
            minimum_star_size: 1,
            maximum_star_size: 24,
        })
    }
}

impl TryFrom<DynamicImage> for StarDetect {
    type Error = Error;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        Ok(Self {
            source: value,
            minimum_star_count: 1000,
            minimum_star_size: 1,
            maximum_star_size: 24,
        })
    }
}

impl<'a> TryFrom<&'a DynamicImage> for StarDetect {
    type Error = Error;

    fn try_from(value: &'a DynamicImage) -> Result<Self, Self::Error> {
        Ok(Self {
            source: value.clone(),
            minimum_star_count: 1000,
            minimum_star_size: 1,
            maximum_star_size: 24,
        })
    }
}

impl StarDetect {
    pub fn with_minimum_star_count(mut self, minimum_star_count: usize) -> Self {
        self.minimum_star_count = minimum_star_count;
        self
    }

    pub fn with_minimum_star_size(mut self, minimum_star_size: usize) -> Self {
        self.minimum_star_size = minimum_star_size;
        self
    }

    pub fn with_maximum_star_size(mut self, maximum_star_size: usize) -> Self {
        self.maximum_star_size = maximum_star_size;
        self
    }

    pub fn compute_star_centers(&self) -> Vec<StarCenter> {
        let (width, height) = self.source.dimensions();
        let smallest_side = width.min(height);

        let decomposition_levels = ((smallest_side as f32).ln() as usize).min(20);

        let mut filtered_image = ATrousTransform::new(
            &self.source,
            decomposition_levels,
            LinearInterpolationKernel,
        )
        .filter(|item| {
            item.pixel_scale
                .is_some_and(|scale| scale < decomposition_levels / 2)
        })
        .recompose_into_image(width as usize, height as usize, OutputLayer::Rgb);

        let threshold = filtered_image.optimize_threshold_for_star_count(
            self.minimum_star_count,
            self.minimum_star_size,
            self.maximum_star_size,
        );
        filtered_image.binarize(threshold);

        let (red, green, blue) = filtered_image.channel_wise_split();
        find_rgb_stars(
            &red,
            &green,
            &blue,
            self.minimum_star_size,
            self.maximum_star_size,
        )
    }
}
