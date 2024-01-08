use std::{io, fmt, error};

use image::ImageError;

/// Enum representing various types of errors that can occur in the application.
#[derive(Debug)]
pub enum FractalError {
    /// Error related to Input/Output operations.
    Io(io::Error),
    /// Error related to image processing.
    Image(ImageError),
    /// Error occurring when a path cannot be converted to a string.
    PathConversion(String),
    /// Error related to data serialization.
    SerializationError(String),
    /// Error indicating that a required item was not found.
    NotFound(String),
    /// Error representing an unsupported operation.
    UnsupportedOperation(String),
    /// Error indicating that a required task was not set.
    TaskNotSet(String),
    /// Error related to network connection.
    ConnectionError(String),
    /// Generic error for other cases.
    Other(String),
}




impl fmt::Display for FractalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FractalError::Io(e) => write!(f, "IO error: {}", e),
            FractalError::Image(e) => write!(f, "Image error: {}", e),
            FractalError::PathConversion(e) => write!(f, "Path conversion error: {}", e),
            FractalError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            FractalError::NotFound(e) => write!(f, "Not found: {}", e),
            FractalError::UnsupportedOperation(e) => write!(f, "Unsupported operation: {}", e),
            FractalError::TaskNotSet(e) => write!(f, "Task not set: {}", e),
            FractalError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            FractalError::Other(e) => write!(f, "An error occurred: {}", e),
        }
    }
}

impl error::Error for FractalError {}

impl From<io::Error> for FractalError {
    fn from(e: io::Error) -> Self {
        FractalError::Io(e)
    }
}

impl From<String> for FractalError {
    fn from(e: String) -> Self {
        FractalError::Other(e)
    }
}

impl From<serde_json::error::Error> for FractalError {
    fn from(error: serde_json::error::Error) -> Self {
        FractalError::SerializationError(error.to_string())
    }
}