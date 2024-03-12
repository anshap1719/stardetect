use image::DynamicImage;

use crate::binarize::binarize_image;
use crate::centroid::find_star_centres_and_size;

pub const MIN_STAR_COUNT: usize = 500;

pub fn optimize_threshold_for_star_count(image: &DynamicImage) -> u8 {
    let mut star_count = 0;
    let mut threshold = u8::MAX;

    while star_count < MIN_STAR_COUNT {
        threshold = (0.95 * threshold as f32) as u8;

        let mut image = image.clone();
        binarize_image(&mut image, threshold);
        star_count = find_star_centres_and_size(&image.to_luma8()).len();

        println!("{star_count}");
    }

    threshold
}
