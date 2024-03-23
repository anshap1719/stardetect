use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use image_dwt::kernels::LinearInterpolationKernel;
use image_dwt::rescale::RescalableImage;
use image_dwt::transform::ATrousTransform;

use stardetect::binarize::binarize_image;
use stardetect::centroid::{find_rgb_stars, StarCenter};
use stardetect::io::read_image;
use stardetect::star_count::optimize_threshold_for_star_count;
use stardetect::ChannelSplit;

fn main() {
    let image = read_image("./recombined.jpg").unwrap();
    let transform = ATrousTransform::new(&image, 10, LinearInterpolationKernel);

    let mut filtered_image: ImageBuffer<Rgb<f32>, Vec<f32>> =
        ImageBuffer::new(image.width(), image.height());

    for layer in transform {
        match layer.pixel_scale {
            Some(scale) if scale < 3 => {
                DynamicImage::ImageRgb32F(layer.image_buffer.clone())
                    .to_rgb8()
                    .save(format!("{scale}scale.jpg"))
                    .unwrap();

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
    filtered_image.to_rgb8().save("test.jpg").unwrap();

    let threshold = optimize_threshold_for_star_count(&filtered_image);
    binarize_image(&mut filtered_image, threshold);

    let (red, green, blue) = filtered_image.channel_wise_split();

    filtered_image
        .to_rgb8()
        .save("./recombined-binary.jpg")
        .unwrap();

    let rgb_stars = find_rgb_stars(&red, &green, &blue);

    for StarCenter { coord, radius } in rgb_stars {
        filtered_image = DynamicImage::ImageRgba8(imageproc::drawing::draw_hollow_circle(
            &filtered_image,
            (coord.x as i32, coord.y as i32),
            (radius as i32).max(4),
            Rgba([0, 255, 0, 1]),
        ));
    }

    red.save("./recombined-red.jpg").unwrap();
    green.save("./recombined-green.jpg").unwrap();
    blue.save("./recombined-blue.jpg").unwrap();

    filtered_image.to_rgb8().save("./final.jpg").unwrap()
}
