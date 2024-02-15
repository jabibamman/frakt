use serde::{Deserialize, Serialize};

/// Represents a point in a two-dimensional space.
///
/// Attributes:
/// - `x`: A `f64` representing the x-coordinate of the point.
/// - `y`: A `f64` representing the y-coordinate of the point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new `Point` with the specified x and y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - A `f64` representing the x-coordinate of the point.
    /// * `y` - A `f64` representing the y-coordinate of the point.
    ///
    /// # Returns
    ///
    /// A new `Point` with the specified x and y coordinates.
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
}
