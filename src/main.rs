use image::{DynamicImage, Rgba};

use stardetect::centroid::{find_rgb_stars, StarCenter};
use stardetect::io::read_image;
use stardetect::{binarize_image, filter_stars};

fn main() {
    let mut image = read_image("./sample.jpg").unwrap();
    let mut original_image = image.clone();

    binarize_image(&mut image);

    let (red, green, blue) = filter_stars(&image);

    image.save("./sample-binary.jpg").unwrap();

    let rgb_stars = find_rgb_stars(&red, &green, &blue);
    println!("{}", rgb_stars.len());

    for StarCenter { coord, radius } in rgb_stars {
        println!("{coord:?}, {radius}");
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
