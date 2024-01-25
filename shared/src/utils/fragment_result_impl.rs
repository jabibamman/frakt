use log::{debug, error};
use serde::ser::Error;

use crate::types::messages::FragmentResult;
use crate::types::pixel_data::PixelData;
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;
use serde_json::Value;

pub trait FragmentResultOperation {
    fn new(id: U8Data, resolution: Resolution, range: Range, pixels: PixelData) -> Self;
    fn serialize(&self) -> Result<String, serde_json::Error>;

    fn deserialize(message: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized;
}
impl FragmentResultOperation for FragmentResult {
    fn new(id: U8Data, resolution: Resolution, range: Range, pixels: PixelData) -> Self {
        Self {
            id,
            resolution,
            range,
            pixels,
        }
    }

    fn serialize(&self) -> Result<String, serde_json::Error> {
        let fragment_json = serde_json::json!({"FragmentResult": &self});
        debug!(
            "Serialized message: {}",
            serde_json::to_string(&fragment_json)?
        );
        serde_json::to_string(&fragment_json)
    }

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> {
        debug!("Deserializing message: {}", message);

        let v: Value = serde_json::from_str(message)?;
        if !v["FragmentResult"].is_object() {
            error!("Invalid format: FragmentResult object not found");
            return Err(serde_json::Error::custom("Invalid format"));
        }
        serde_json::from_value(v["FragmentResult"].clone())
    }
}

#[cfg(test)]
mod fragment_result_tests {
    use crate::types::point::Point;

    use super::*;

    #[test]
    fn test_serialize() {
        let fragment_result = FragmentResult::new(
            U8Data {
                offset: 0,
                count: 0,
            },
            Resolution { nx: 0, ny: 0 },
            Range {
                min: Point { x: 0.0, y: 0.0 },
                max: Point { x: 0.0, y: 0.0 },
            },
            PixelData {
                offset: 0,
                count: 0,
            },
        );

        let result = fragment_result.serialize();
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_deserialize() {
        let serialized_fragment_result = "{\"FragmentResult\":{\"id\":{\"offset\":0,\"count\":0},\"resolution\":{\"nx\":0,\"ny\":0},\"range\":{\"min\":{\"x\":0.0,\"y\":0.0},\"max\":{\"x\":0.0,\"y\":0.0}},\"pixels\":{\"offset\":0,\"count\":0}}}";
        let result = FragmentResult::deserialize(serialized_fragment_result);
        assert!(result.is_ok());
        let fragment_result = result.unwrap();
        assert_eq!(fragment_result.id.offset, 0);
        assert_eq!(fragment_result.id.count, 0);
        assert_eq!(fragment_result.resolution.nx, 0);
        assert_eq!(fragment_result.resolution.ny, 0);
        assert_eq!(fragment_result.range.min.x, 0.0);
        assert_eq!(fragment_result.range.min.y, 0.0);
        assert_eq!(fragment_result.range.max.x, 0.0);
        assert_eq!(fragment_result.range.max.y, 0.0);
        assert_eq!(fragment_result.pixels.offset, 0);
        assert_eq!(fragment_result.pixels.count, 0);
    }
}
