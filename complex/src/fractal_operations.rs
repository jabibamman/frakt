use shared::types::{complex::Complex, pixel_intensity::PixelIntensity};

pub trait FractalOperations {
    /// Computes the pixel intensity of a complex point.
    fn compute_pixel_intensity(
        &self,
        complex_point: &Complex,
        max_iteration: u16,
    ) -> PixelIntensity;
}

pub trait BurningShipFractalOperations {
    /// Iterates over a complex point and returns the number of iterations before it diverges.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> (u16, f64);

    /// Returns a reference to the complex number `c` used in the Julia fractal formula.
    fn c(&self) -> &Complex;
}
