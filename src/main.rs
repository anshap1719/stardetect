use stardetect::{binarize_image, filter_stars};
use stardetect::io::read_image;

fn main() {
    let mut image = read_image("./sample.jpg").unwrap();
    binarize_image(&mut image);

    let (red, green, blue) = filter_stars(&image);

    image.save("./sample-binary.jpg").unwrap();
    red.save("./sample-red.jpg").unwrap();
    green.save("./sample-green.jpg").unwrap();
    blue.save("./sample-blue.jpg").unwrap();
}
