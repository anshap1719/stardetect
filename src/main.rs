use image::{DynamicImage, Rgba};

use stardetect::binarize::binarize_image;
use stardetect::centroid::{find_rgb_stars, StarCenter};
use stardetect::filter_stars;
use stardetect::io::read_image;
use stardetect::star_count::optimize_threshold_for_star_count;

fn main() {
    let mut image = read_image("./sample.jpg").unwrap();
    let mut original_image = image.clone();

    let threshold = optimize_threshold_for_star_count(&image);

    binarize_image(&mut image, threshold);

    let (red, green, blue) = filter_stars(&image);

    image.save("./sample-binary.jpg").unwrap();

    let rgb_stars = find_rgb_stars(&red, &green, &blue);

    for StarCenter { coord, radius } in rgb_stars {
        original_image = DynamicImage::ImageRgba8(imageproc::drawing::draw_hollow_circle(
            &original_image,
            (coord.x as i32, coord.y as i32),
            (radius as i32).max(1),
            Rgba([0, 255, 0, 1]),
        ));
    }

    red.save("./sample-red.jpg").unwrap();
    green.save("./sample-green.jpg").unwrap();
    blue.save("./sample-blue.jpg").unwrap();

    original_image.to_rgb8().save("./final.jpg").unwrap()
}
