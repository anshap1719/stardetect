use image::DynamicImage;

use crate::binarize::binarize_image;
use crate::centroid::find_star_centres_and_size;

pub const MIN_STAR_COUNT: usize = 1000;

pub fn optimize_threshold_for_star_count(image: &DynamicImage) -> u8 {
    let mut star_count = 0;
    let mut threshold = 255;

    while star_count < MIN_STAR_COUNT {
        if threshold == 0 {
            panic!("Maximum iteration count reached");
        }

        threshold = (0.95 * threshold as f32) as u8;

        let mut image = image.clone();
        binarize_image(&mut image, threshold);

        image.to_rgb8().save(format!("{star_count}.jpg")).unwrap();

        star_count = find_star_centres_and_size(&image.to_luma8()).len();
    }

    threshold
}
