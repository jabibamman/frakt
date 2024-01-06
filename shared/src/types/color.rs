/// RGB : Red, Green, Blue
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// HSL : Hue, Saturation, Lightness
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}
