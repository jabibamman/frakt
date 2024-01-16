use log::debug;
use serde::de::Error as SerdeError;

use shared::{
    types::messages::{FragmentRequest, Message},
    utils::fragment_request_impl::FragmentRequestOperation,
};

/// Deserializes a JSON string into a `Message` enum variant.
///
/// # Arguments
///
/// * `response` - A string slice that holds the JSON representation of the message.
///
/// # Returns
///
/// A `Result<Message, serde_json::Error>` which is Ok containing the `Message` enum variant if deserialization is successful,
/// or an Err containing the `serde_json::Error` if it fails.
///
/// # Note
///
/// Currently, it only supports deserializing `FragmentRequest`. Other types like `FragmentTask` and `FragmentResult` are marked with `todo!()`.
pub fn deserialize_message(response: &str) -> serde_json::Result<Message> {
    let response_value: serde_json::Value = serde_json::from_str(response)?;
    debug!("Response value: {:?}", response_value);
    match response_value.as_object().and_then(|obj| obj.keys().next()) {
        Some(key) if key == "FragmentRequest" => {
            debug!("Deserializing FragmentRequest");
            FragmentRequest::deserialize(response).map(Message::FragmentRequest)
        }
        Some(key) if key == "FragmentTask" => {
            todo!()
        }
        Some(key) if key == "FragmentResult" => {
            todo!()
        }
        _ => Err(serde_json::Error::custom(
            "No recognizable message type found",
        )),
    }
}
