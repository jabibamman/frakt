use serde::{Deserialize, Serialize};

/// Represents the resolution of an image or a grid, defined by the number of pixels or cells along the x and y axes.
///
/// Attributes:
/// - `nx`: A `u16` representing the number of pixels or cells along the x-axis (width).
/// - `ny`: A `u16` representing the number of pixels or cells along the y-axis (height).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

impl Resolution {
    /// Creates a new `Resolution` with the specified number of pixels along the x and y axes.
    ///
    /// # Arguments
    ///
    /// * `nx` - A `u16` representing the number of pixels along the x-axis (width).
    /// * `ny` - A `u16` representing the number of pixels along the y-axis (height).
    ///
    /// # Returns
    ///
    /// A new `Resolution` with the specified number of pixels along the x and y axes.
    pub fn new(nx: u16, ny: u16) -> Resolution {
        Resolution { nx, ny }
    }
    
}
