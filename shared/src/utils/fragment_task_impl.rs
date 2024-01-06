use log::{debug, error};
use serde::ser::Error;
use serde_json::Value;

use crate::types::{messages::FragmentTask, fractal_descriptor::FractalDescriptor, resolution::Resolution, range::Range, u8data::U8Data};

pub trait FragmentTaskOperation {
    fn new(id: U8Data, fractal: FractalDescriptor, max_iteration: u16, resolution: Resolution, range: Range) -> Self;

    /// Serializes a `FragmentTask` into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to the `FragmentTask` to be serialized.
    ///
    /// # Returns
    ///
    /// A `Result<String, serde_json::Error>` which is Ok containing the JSON string if serialization is successful,
    /// or an Err containing the `serde_json::Error` if it fails.
    fn serialize(&self) -> Result<String, serde_json::Error>;

    fn deserialize(message: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized;
}

impl FragmentTaskOperation for FragmentTask {
    fn new(id: U8Data, fractal: FractalDescriptor, max_iteration: u16, resolution: Resolution, range: Range) -> Self {
        Self {
            id,
            fractal,
            max_iteration,
            resolution,
            range,
        }
    }

    fn serialize(&self) -> Result<String, serde_json::Error> {
        let fragment_json = serde_json::json!({"FragmentTask": &self});
        serde_json::to_string(&fragment_json)
    }

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> {
        debug!("Deserializing message: {}", message);
        let v: Value = serde_json::from_str(message)?;
        if !v["FragmentTask"].is_object() {
            error!("Invalid format: FragmentTask object not found");
            return Err(serde_json::Error::custom("Invalid format"));
        }
        serde_json::from_value(v["FragmentTask"].clone())
    }
}