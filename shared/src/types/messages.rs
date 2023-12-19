use crate::types::fractal_descriptor::FractalDescriptor;
use crate::types::pixel_data::PixelData;
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, PartialEq)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
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
#[derive(Debug, Clone, PartialEq)]
pub struct FragmentResult {
    id: U8Data,
    resolution: Resolution,
    range: Range,
    pixels: PixelData,
}
