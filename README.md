# Stardetect – A library to efficiently detect stars positions and sizes in any image

This project provides an implementation of the star detection technique used in astronomical plate solving algorithms.
The implementation uses À Trous Wavelet decomposition under the hood to optimize the image before running star detection
algorithms.

## Why

I'm trying to build a suite of tools in rust that facilitate image processing, primarily deep sky images and data. Star
detection is particularly helpful in plate-solving astronomical images which allows various image processing techniques
to be implemented, such as star correction, noise reduction, etc.

## Usage

```rust
fn detect_stars() {
    let star_detect = StarDetect::try_from("./sample.jpg").unwrap();
    let star_centers = star_detect.compute_star_centers();
}
```

## Installation

To use this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
stardetect = "0.3.0"
