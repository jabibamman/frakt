/// Represents the intensity of a pixel in fractal rendering, particularly in the context of iterations.
///
/// Attributes:
/// - `zn`: A `f32` representing the magnitude at the end of the iteration process.
///   This is typically used to determine the color or intensity of a pixel in fractal images.
/// - `count`: A `f32` reflecting the number of iterations performed, normalized by the maximum number of iterations.
///   This value is often used to apply color gradients based on the iteration depth.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}