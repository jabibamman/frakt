use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::resolution::Resolution;

pub trait FractalOperations {
    /// Converts pixel coordinates to a complex number based on the resolution.
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        let re = (x as f64 / resolution.nx as f64) * 4.0 - 2.0;
        let im = (y as f64 / resolution.ny as f64) * 4.0 - 2.0;
        Complex::new(re, im)
    }

    /// Iterates over a complex point and returns the number of iterations before it diverges.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16;
}
