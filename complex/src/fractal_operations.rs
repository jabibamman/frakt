use shared::types::{complex::Complex, pixel_intensity::PixelIntensity};

pub trait FractalOperations {
    /// Iterates over a complex point and returns the number of iterations before it diverges.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16;

    /// Computes the pixel intensity of a complex point.
    fn compute_pixel_intensity(&self, complex_point: &Complex, max_iteration: u16) -> PixelIntensity;
}
