use crate::types::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}