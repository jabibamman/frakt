use shared::types::{complex::Complex, pixel_intensity::PixelIntensity};

pub trait FractalOperations {
    /// Computes the pixel intensity of a complex point.
    fn compute_pixel_intensity(
        &self,
        complex_point: &Complex,
        max_iteration: u16,
    ) -> PixelIntensity;
}
