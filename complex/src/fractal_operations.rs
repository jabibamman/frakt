use shared::types::complex::Complex;

pub trait FractalOperations {
    /// Iterates over a complex point and returns the number of iterations before it diverges.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16;
}
