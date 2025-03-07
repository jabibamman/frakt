use serde::{Deserialize, Serialize};

use crate::types::point::Point;

/// Defines a rectangular range in a two-dimensional space, represented by minimum and maximum points.
///
/// Attributes:
/// - `min`: A `Point` defining the minimum (bottom-left) corner of the range.
/// - `max`: A `Point` defining the maximum (top-right) corner of the range.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

impl Range {
    /// Creates a new `Range` with the specified minimum and maximum points.
    ///
    /// # Arguments
    ///
    /// * `min` - A `Point` defining the minimum (bottom-left) corner of the range.
    /// * `max` - A `Point` defining the maximum (top-right) corner of the range.
    ///
    /// # Returns
    ///
    /// A new `Range` with the specified minimum and maximum points.
    pub fn new(min: Point, max: Point) -> Self {
        Range { min, max }
    }
}
