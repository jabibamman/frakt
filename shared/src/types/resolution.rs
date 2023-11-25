use serde::{Serialize, Deserialize};

/// Represents the resolution of an image or a grid, defined by the number of pixels or cells along the x and y axes.
///
/// Attributes:
/// - `nx`: A `u16` representing the number of pixels or cells along the x-axis (width).
/// - `ny`: A `u16` representing the number of pixels or cells along the y-axis (height).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}
