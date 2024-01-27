use serde::{Deserialize, Serialize};

use crate::types::fractal_descriptor::FractalDescriptor;
use crate::types::pixel_data::PixelData;
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;

use super::fractal_descriptor::BurningShipFractalDescriptor;

/// Represents a request for a fragment of work from a worker.
///
/// Attributes:
/// - `worker_name`: A `String` representing the name of the worker making the request.
/// - `maximal_work_load`: An `u32` indicating the maximum workload (in terms of pixels) the worker can handle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

/// Describes a task assigned to a worker for fractal computation by a Server.
///
/// Attributes:
/// - `id`: An `U8Data` structure, typically representing an identifier for the task.
/// - `fractal`: A `FractalDescriptor` detailing the type and parameters of the fractal to be computed.
/// - `max_iteration`: A `u16` specifying the maximum number of iterations for the fractal computation.
/// - `resolution`: A `Resolution` specifying the resolution of the fragment to be computed.
/// - `range`: A `Range` defining the physical space coordinates for the fragment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BurningShipFragmentTask {
    pub id: U8Data,
    pub fractal: BurningShipFractalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
}

/// Represents the result of a fragment computation by a worker.
///
/// Attributes:
/// - `id`: An `U8Data` structure, typically representing the identifier of the task for which this is the result.
/// - `resolution`: A `Resolution` specifying the resolution of the computed fragment.
/// - `range`: A `Range` defining the physical space coordinates for the computed fragment.
/// - `pixels`: A `PixelData` containing the computed pixel data for the fragment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}

/// An enumeration representing different types of messages in the system.
///
/// This enum categorizes messages into three distinct types, each associated with a specific data structure:
/// `FragmentRequest`, `FragmentTask`, and `FragmentResult`.
///
/// # Variants
///
/// - `FragmentRequest(FragmentRequest)`: Represents a request for a fragment operation, containing the details
///   required to process this request.
/// - `FragmentTask(FragmentTask)`: Represents a task associated with a fragment, typically used to describe
///   the work to be done or in progress.
/// - `FragmentResult(FragmentResult)`: Represents the result of a fragment operation, detailing the outcome
///   of the task or request.
///
/// # Traits
///
/// - `Debug`: Allows for formatting the enum using the `{:?}` formatter. Useful for debugging purposes.
/// - `Clone`: Enables the creation of a copy of a value in this enum. Each variant of the enum will also need
///   to implement `Clone`.
/// - `PartialEq`: Enables comparison for equality between two enum values. Each variant's associated type
///   must also implement `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    FragmentRequest(FragmentRequest),
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
}
