use serde::{Deserialize, Serialize};

/// Represents data associated with a set of pixels in an image or a fragment of an image.
///
/// Attributes:
/// - `offset`: A `u32` indicating the starting point or offset in a larger array or buffer where the pixel data begins.
/// - `count`: A `u32` representing the number of pixels (or data points) that are included starting from the `offset`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
} 

impl PixelData {
    pub fn new(offset: u32, count: u32) -> Self {
        Self { offset, count }
    }    

}
