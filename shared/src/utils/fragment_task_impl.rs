use log::{debug, error};
use serde::ser::Error;
use serde_json::Value;

use crate::types::{
    fractal_descriptor::FractalDescriptor, messages::FragmentTask, range::Range,
    resolution::Resolution, u8data::U8Data,
};

pub trait FragmentTaskOperation {
    fn new(
        id: U8Data,
        fractal: FractalDescriptor,
        max_iteration: u16,
        resolution: Resolution,
        range: Range,
    ) -> Self;

    /// Serializes a `FragmentTask` into a JSON string.
    ///
    /// This function takes a reference to a `FragmentTask` and converts it into a JSON string using `serde_json`. The serialization captures detailed information about the fractal task, including the type of fractal (Julia or IteratedSinZ), various parameters of the fractal, the maximum iteration count, resolution, and the range for the task.

    /// # Arguments
    ///
    /// * `request` - A reference to the `FragmentTask` to be serialized.
    ///
    /// # Returns
    ///
    /// A `Result<String, serde_json::Error>` which is Ok containing the JSON string if serialization is successful,
    /// or an Err containing the `serde_json::Error` if it fails.
    fn serialize(&self) -> Result<String, serde_json::Error>;

    /// Deserializes a JSON string into a `FragmentTask`.
    ///
    /// This function takes a string slice that holds the JSON representation of the `FragmentTask` and converts it into a `FragmentTask` using `serde_json`. The deserialization captures detailed information about the fractal task, including the type of fractal (Julia or IteratedSinZ), various parameters of the fractal, the maximum iteration count, resolution, and the range for the task.
    ///
    /// # Arguments
    ///
    /// * `response` - A string slice that holds the JSON representation of the `FragmentTask`.
    ///
    /// # Returns
    ///
    /// A `Result<FragmentTask, serde_json::Error>` which is Ok containing the `FragmentTask` if deserialization is successful,
    ///
    /// or an Err containing the `serde_json::Error` if it fails.
    ///
    fn deserialize(message: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized;
}

impl FragmentTaskOperation for FragmentTask {
    fn new(
        id: U8Data,
        fractal: FractalDescriptor,
        max_iteration: u16,
        resolution: Resolution,
        range: Range,
    ) -> Self {
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
