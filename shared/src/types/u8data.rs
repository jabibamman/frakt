use serde::{Deserialize, Serialize};

/// Represents a segment of data, typically used for handling parts of a byte stream.
///
/// Attributes:
/// - `offset`: A `u32` indicating the starting position in a byte stream or array.
/// - `count`: A `u32` denoting the length or the number of elements in the segment starting from `offset`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}
