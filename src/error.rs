//! Error definitions for the library

use std::fmt::{Debug, Display, Formatter};
use std::io;

use image::ImageError;
use rawloader::RawLoaderError;
use thiserror::Error;

/// Represents error occurred during the raw image decoding pipeline.
#[derive(Error, Debug)]
pub struct RawPipelineError(pub String);

impl Display for RawPipelineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<String> for RawPipelineError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Represents errors that cannot be categorised as any other error types.
#[derive(Error, Debug)]
pub struct UnknownError(pub String);

impl Display for UnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<String> for UnknownError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Error that can be returned by any of the methods in the library.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents image or raw image decoding error
    #[error("Unable to decode raw image")]
    DecodeError(#[from] RawLoaderError),
    /// Represents raw image pipeline error
    #[error("Unable to decode raw image")]
    RawPipeline(#[from] RawPipelineError),
    /// Represents error occurred while reading/writing image files
    #[error("Unable to read file")]
    IoError(#[from] io::Error),
    /// Represents error occurred while converting between different image formats or while writing
    /// buffers from raw/processed pixel data.
    #[error("Unable to process image")]
    ImageError(#[from] ImageError),
    /// Represents error caused by invalid input to the crate's functions
    #[error("Invalid value for {parameter_name:?}: {message:?}")]
    InputError {
        /// Name of the parameter for which error occurred
        parameter_name: String,
        /// A message explaining why parameter is invalid
        message: String,
    },
    /// Represents errors that cannot be categorised as any other error types.
    #[error("{0}")]
    UnknownError(#[from] UnknownError),
}
