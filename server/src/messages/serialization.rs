use serde_json::json;
use shared::types::messages::{FragmentRequest, Message, FragmentTask};
use serde::de::Error as SerdeError;


/// Serializes a `FragmentRequest` into a JSON string.
///
/// # Arguments
///
/// * `request` - A reference to the `FragmentRequest` to be serialized.
///
/// # Returns
///
/// A `Result<String, serde_json::Error>` which is Ok containing the JSON string if serialization is successful,
/// or an Err containing the `serde_json::Error` if it fails.
pub fn serialize_request(request: &FragmentRequest) -> Result<String, serde_json::Error> {
    let request_details = json!({
        "worker_name": &request.worker_name,
        "maximal_work_load": request.maximal_work_load
    });

    let request = json!({
        "FragmentRequest": request_details
    });

    serde_json::to_string(&request)
}

/// Deserializes a JSON string into a `FragmentRequest`.
///
/// # Arguments
///
/// * `response` - A string slice that holds the JSON representation of the `FragmentRequest`.
///
/// # Returns
///
/// A `Result<FragmentRequest, serde_json::Error>` which is Ok containing the `FragmentRequest` if deserialization is successful,
/// or an Err containing the `serde_json::Error` if it fails.
pub fn deserialize_request(response: &str) -> serde_json::Result<FragmentRequest> {
    let response_value: serde_json::Value = serde_json::from_str(response)?;
    let response_details = response_value.get("FragmentRequest").ok_or_else(|| SerdeError::custom("FragmentRequest not found"))?;

    let worker_name = response_details.get("worker_name").and_then(|c| c.as_str()).ok_or_else(|| SerdeError::custom("Invalid worker name"))?;
    let maximal_work_load = response_details.get("maximal_work_load").and_then(|c| c.as_u64()).ok_or_else(|| SerdeError::custom("Invalid maximal work load"))?;

    let response = FragmentRequest {
        worker_name: worker_name.to_string(),
        maximal_work_load: maximal_work_load as u32,
    };

    Ok(response)
}

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
    match response_value.as_object().and_then(|obj| obj.keys().next()) {
        Some(key) if key == "FragmentRequest" => {
            deserialize_request(response).map(Message::FragmentRequest)
        }
        Some(key) if key == "FragmentTask" => {
            todo!()
        }
        Some(key) if key == "FragmentResult" => {
            todo!()
        }
        _ => Err(serde_json::Error::custom("No recognizable message type found"))
    }
}

pub fn serialize_task(_task: &FragmentTask) -> Result<String, serde_json::Error> {
    todo!()
}