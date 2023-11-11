use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::MandelbrotDescriptor;

pub trait MandelbrotOperations {
    fn new() -> Self;
    fn iterate_complex_point(&self, complex_point: &Complex) -> u16;
}

impl MandelbrotOperations for MandelbrotDescriptor {
    fn new() -> Self {
        Self {
        }
    }
    fn iterate_complex_point(&self, complex_point: &Complex) -> u16 {
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;
        while z.norm() <= 4.0 {
            z = z.square().add(complex_point);
            iterations += 1;
        }
        iterations
    }


}
