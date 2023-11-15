use shared::types::complex::Complex;
use shared::types::resolution::Resolution;
use crate::complex_operations::ComplexOperations;

pub trait FractalOperations {
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        let re = (x as f64 / resolution.nx as f64) * 4.0 - 2.0;
        let im = (y as f64 / resolution.ny as f64) * 4.0 - 2.0;
        Complex::new(re, im)
    }
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16;

    ///Return the complex stored in the fractal descriptor
    fn c(&self) -> &Complex;
}
