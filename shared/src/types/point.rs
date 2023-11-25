use serde::{Serialize, Deserialize};

/// Represents a point in a two-dimensional space.
///
/// Attributes:
/// - `x`: A `f64` representing the x-coordinate of the point.
/// - `y`: A `f64` representing the y-coordinate of the point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
