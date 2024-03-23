use acap::Euclidean;
use image::{DynamicImage, ImageBuffer, Rgb};
use image_dwt::kernels::LinearInterpolationKernel;
use image_dwt::rescale::RescalableImage;
use image_dwt::transform::ATrousTransform;

use stardetect::binarize::binarize_image;
use stardetect::centroid::{find_rgb_stars, StarCenters};
use stardetect::ChannelSplit;
use stardetect::io::read_image;
use stardetect::quad::StarQuads;
use stardetect::star_count::optimize_threshold_for_star_count;

fn main() {
    let image = read_image("./5bef9d1cc91f5635e4274f8df62f6906.jpg").unwrap();
    let transform = ATrousTransform::new(&image, 10, LinearInterpolationKernel);

    let mut filtered_image: ImageBuffer<Rgb<f32>, Vec<f32>> =
        ImageBuffer::new(image.width(), image.height());

    for layer in transform {
        match layer.pixel_scale {
            Some(scale) if scale < 3 => {
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

    let threshold = optimize_threshold_for_star_count(&filtered_image);
    binarize_image(&mut filtered_image, threshold);

    let (red, green, blue) = filtered_image.channel_wise_split();
    let stars = StarCenters(find_rgb_stars(&red, &green, &blue));
    let quads = StarQuads::from(Vec::<Euclidean<[f32; 2]>>::from(stars));

    for quad in quads.iter() {
        let hashes = quad.generate_normalized_hashes();
        dbg!(hashes);
    }

    filtered_image.to_rgb8().save("./final.jpg").unwrap()
}
