use acap::{Euclidean, EuclideanDistance, NearestNeighbors, Proximity};
use acap::vp::VpTree;

#[derive(Copy, Clone, PartialEq)]
pub struct StarQuadEntry {
    x: f32,
    y: f32,
    distance: EuclideanDistance<f32>,
}

#[derive(Copy, Clone, PartialEq)]
pub struct StarQuad {
    x: f32,
    y: f32,
    closest_points: [StarQuadEntry; 3],
}

impl StarQuad {
    pub fn generate_normalized_hashes(&self) -> [f32; 6] {
        let mut largest_distance = EuclideanDistance::try_from(0.).unwrap();
        let mut hash_codes = [0_f32; 6];

        let closest1 = self.closest_points[0];
        hash_codes[0] = closest1.distance.squared_value();
        if closest1.distance > largest_distance {
            largest_distance = closest1.distance;
        }

        let closest2 = self.closest_points[1];
        hash_codes[1] = closest2.distance.squared_value();
        if closest2.distance > largest_distance {
            largest_distance = closest2.distance;
        }

        let closest3 = self.closest_points[2];
        hash_codes[2] = closest3.distance.squared_value();
        if closest3.distance > largest_distance {
            largest_distance = closest3.distance;
        }

        let distance =
            Euclidean([closest1.x, closest1.y]).distance(&Euclidean([closest2.x, closest2.y]));

        if distance > largest_distance {
            largest_distance = distance;
        }
        hash_codes[3] = distance.squared_value();

        let distance =
            Euclidean([closest1.x, closest1.y]).distance(&Euclidean([closest3.x, closest3.y]));

        if distance > largest_distance {
            largest_distance = distance;
        }
        hash_codes[4] = distance.squared_value();

        let distance =
            Euclidean([closest2.x, closest2.y]).distance(&Euclidean([closest3.x, closest3.y]));

        if distance > largest_distance {
            largest_distance = distance;
        }
        hash_codes[5] = distance.squared_value();

        let largest_distance = largest_distance.squared_value();
        for code in hash_codes.iter_mut() {
            *code /= largest_distance;
        }

        hash_codes
    }
}

pub struct StarQuads(pub Vec<StarQuad>);

impl StarQuads {
    pub fn iter(&self) -> impl Iterator<Item = &StarQuad> {
        self.0.iter()
    }
}

impl From<Vec<Euclidean<[f32; 2]>>> for StarQuads {
    fn from(value: Vec<Euclidean<[f32; 2]>>) -> Self {
        let tree = VpTree::balanced(value);
        let mut star_quads = vec![];

        for point in &tree {
            let neighbours = tree
                .k_nearest(point, 3)
                .iter()
                .map(|item| StarQuadEntry {
                    x: item.item.0[0],
                    y: item.item.0[1],
                    distance: item.distance,
                })
                .collect::<Vec<_>>();

            star_quads.push(StarQuad {
                x: point.0[0],
                y: point.0[1],
                closest_points: [neighbours[0], neighbours[1], neighbours[2]],
            })
        }

        Self(star_quads)
    }
}
