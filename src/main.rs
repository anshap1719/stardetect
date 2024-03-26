use stardetect::StarDetect;

fn main() {
    let star_detect = StarDetect::try_from("./5bef9d1cc91f5635e4274f8df62f6906.jpg").unwrap();
    let star_centers = star_detect.compute_star_centers();

    dbg!(star_centers.len());
}
