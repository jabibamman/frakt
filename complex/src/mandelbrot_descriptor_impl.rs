use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::MandelbrotDescriptor;
use crate::fractal_operations::FractalOperations;

pub trait MandelbrotOperations {
    fn new() -> Self;
}

impl MandelbrotOperations for MandelbrotDescriptor {
    fn new() -> Self {
        Self {
        }
    }

}
impl FractalOperations for MandelbrotDescriptor {
    fn iterate_complex_point(&self, complex_point: &Complex, _max_iteration: u16) -> u16 {
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;
        while z.abs().sqrt()<= 4.0{
            z = z.square().add(complex_point);
            iterations += 1;
        }
        iterations
    }

}

