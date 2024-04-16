use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use geo::{Centroid, Coord, EuclideanDistance, LineString};
use image::GrayImage;
use imageproc::point::Point;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct StarCenter {
    coord: Point<u32>,
    radius: u32,
}

impl StarCenter {
    pub fn coord(&self) -> &Point<u32> {
        &self.coord
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }
}

impl Hash for StarCenter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.coord.x.hash(state);
        self.coord.y.hash(state);
    }
}

pub(crate) fn find_star_centres_and_size(image: &GrayImage) -> Vec<StarCenter> {
    let contours = imageproc::contours::find_contours::<u32>(image);

    contours
        .par_iter()
        .filter_map(|contour| {
            if contour.points.is_empty() {
                return None;
            }

            if contour.points.len() == 1 {
                let center = contour.points.first().unwrap();
                let radius = 1_u32;

                return Some(StarCenter {
                    coord: *center,
                    radius,
                });
            }

            let mut line_string = LineString::from_iter(contour.points.iter().map(|point| Coord {
                x: point.x as f32,
                y: point.y as f32,
            }));

            line_string.close();

            let center = line_string.centroid().unwrap();
            let radius = line_string.points().fold(0., |distance, point| {
                point.euclidean_distance(&center).max(distance)
            });

            if !(1. ..=24.).contains(&radius) {
                return None;
            }

            Some(StarCenter {
                coord: Point {
                    x: center.x() as u32,
                    y: center.y() as u32,
                },
                radius: radius as u32,
            })
        })
        .collect()
}

pub(crate) fn find_rgb_stars(
    red: &GrayImage,
    green: &GrayImage,
    blue: &GrayImage,
) -> Vec<StarCenter> {
    let red_star_centers_and_sizes = find_star_centres_and_size(red)
        .into_iter()
        .collect::<HashSet<StarCenter>>();
    let green_star_centers_and_sizes = find_star_centres_and_size(green)
        .into_iter()
        .collect::<HashSet<StarCenter>>();
    let blue_star_centers_and_sizes = find_star_centres_and_size(blue)
        .into_iter()
        .collect::<HashSet<StarCenter>>();

    red_star_centers_and_sizes
        .intersection(
            &green_star_centers_and_sizes
                .intersection(&blue_star_centers_and_sizes)
                .copied()
                .collect(),
        )
        .copied()
        .collect()
}
