use crate::types::fractal_descriptor::FractalDescriptor;
use crate::types::pixel_data::PixelData;
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentRequest {
    worker_name: String,
    maximal_work_load: u32
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentResult {
    id: U8Data,
    resolution: Resolution,
    range: Range,
    pixels: PixelData
}
