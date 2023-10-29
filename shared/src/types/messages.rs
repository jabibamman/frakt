use crate::types::pixel_data::PixelData;
use crate::types::pixel_intensity::PixelIntensity;
use crate::types::range::Range;
use crate::types::resolution::Resolution;
use crate::types::u8data::U8Data;

#[derive(Debug, Clone, PartialEq)]
pub enum FragmentRequest {
    Request {
        range: Range,
        resolution: Resolution,
        max_iter: u32,
    },
    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentTask {
    pub id: u32,
    pub range: Range,
    pub resolution: Resolution,
    pub max_iter: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentResult {
    pub id: u32,
    pub u8_data: U8Data,
    pub pixel_data: PixelData,
    pub pixel_intensity: Vec<PixelIntensity>,
}
