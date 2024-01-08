use crate::types::fractal_descriptor::FractalDescriptor;
use crate::types::messages::FragmentResult;
use crate::types::pixel_data::PixelData;
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;
use crate::utils::fragment_request_impl::FragmentRequestOperation;

pub trait FragmentResultOperation {
    fn new(id: U8Data,resolution: Resolution,range: Range,pixels: PixelData,) -> Self;
    fn serialize(&self) -> Result<String, serde_json::Error>;

    fn deserialize(message: &str) -> Result<Self, serde_json::Error>
        where
            Self: Sized;
}
impl FragmentResultOperation for FragmentResult {
    fn new(id: U8Data,resolution: Resolution,range: Range,pixels: PixelData,) -> Self {
        Self {
            id,resolution,range,pixels,
        }
    }

    fn serialize(&self) -> Result<String, serde_json::Error> {
        let fragment_json = serde_json::json!({"FragmentResult": &self});
        serde_json::to_string(&fragment_json)
    }

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(message)
    }
}