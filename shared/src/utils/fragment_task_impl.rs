use crate::types::fractal_descriptor::FractalDescriptor;
use crate::types::messages::{FragmentTask};
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;
use crate::utils::fragment_request_impl::FragmentRequestOperation;

pub trait FragmentTaskOperation {
    fn new(id: U8Data,fractal: FractalDescriptor,max_iteration: u16,resolution: Resolution, range: Range) -> Self;
    fn serialize(&self) -> Result<String, serde_json::Error>;

    fn deserialize(message: &str) -> Result<Self, serde_json::Error>
        where
            Self: Sized;
}
impl FragmentTaskOperation for FragmentTask {
    fn new(id: U8Data,fractal: FractalDescriptor,max_iteration: u16,resolution: Resolution, range: Range) -> Self {
        Self {
            id,
            fractal,
            max_iteration,
            resolution,
            range
        }
    }

    fn serialize(&self) -> Result<String, serde_json::Error> {
        let fragment_json = serde_json::json!({"FragmentTask": &self});
        serde_json::to_string(&fragment_json)
    }

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(message)
    }
}