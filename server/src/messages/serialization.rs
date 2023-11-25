use serde_json::json;
use shared::types::{messages::{FragmentRequest, Message, FragmentTask}, fractal_descriptor::FractalType};
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

pub fn serialize_task(task: &FragmentTask) -> Result<String, serde_json::Error> {
    let fractal_details = match &task.fractal.fractal_type {
        FractalType::Julia(julia_descriptor) => {
            json!({
                "Julia": {
                    "c": {
                        "im": julia_descriptor.c.im,
                        "re": julia_descriptor.c.re
                    },
                    "divergence_threshold_square": julia_descriptor.divergence_threshold_square
                }
            })
        }
        FractalType::IteratedSinZ(iterated_sin_z_descriptor) => {
            json!({
                "IteratedSinZ": {
                    "c": {
                        "im": iterated_sin_z_descriptor.c.im,
                        "re": iterated_sin_z_descriptor.c.re
                    }
                }
            })
        }
    };

    let task_json = json!({
        "id": {
            "offset": task.id.offset,
            "count": task.id.count
        },
        "fractal": fractal_details,
        "max_iteration": task.max_iteration,
        "resolution": {
            "nx": task.resolution.nx,
            "ny": task.resolution.ny
        },
        "range": {
            "min": {
                "x": task.range.min.x,
                "y": task.range.min.y
            },
            "max": {
                "x": task.range.max.x,
                "y": task.range.max.y
            }
        }
    });

    let request = json!({
        "FragmentTask": task_json
    });

    serde_json::to_string(&request)
}


/* 
// Un dispatcher qui décide quel builder utiliser en fonction du nom du fractal
fn dispatch_fractal_builder(fractal_name: &str, fractal_value: &serde_json::Value) -> serde_json::Result<FractalType> {
    //println!("Fractal name: {}, {}", fractal_name, max_iteration);
    match fractal_name {
        "Julia" => build_julia_descriptor(fractal_value),
        // "Mandelbrot" => build_mandelbrot_descriptor(fractal_value, max_iteration),
        _ => Err(SerdeError::custom(format!("Unsupported fractal type: {}", fractal_name))),
    }
}*/