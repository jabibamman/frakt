use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::IteratedSinZDescriptor;
use crate::fractal_operations::FractalOperations;

/// Provides operations specific to the Iterated Sin(z) fractal.
pub trait IteratedSinZOperations {
    /// Constructs a new `IteratedSinZDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex) -> Self;

    fn max_iteration(&self) -> u16;
}


impl FractalOperations for IteratedSinZDescriptor {
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        let mut z = complex_point.clone();
        let mut iterations = 0;
        let max = 256;

        while z.im.abs() < self.max_iteration() as f64 && iterations < max {
            z = z.sin();
            z = z.mul(&self.c);
            iterations += 1;
        }

        iterations
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}

impl IteratedSinZOperations for IteratedSinZDescriptor {
    fn new(c: Complex) -> Self {
        Self {
            c,
        }
    }

    ///Fixed to 50
    fn max_iteration(&self) -> u16 {
        50
    }
}


#[cfg(test)]
mod iterated_sinz_tests {
    use shared::types::complex::Complex;
    use shared::types::fractal_descriptor::IteratedSinZDescriptor;
    use crate::complex_operations::ComplexOperations;
    use crate::iterated_sinz_impl::IteratedSinZOperations;

    #[test]
    fn test_max_iteration_return_50() {
        let c = Complex::new(4.5, -65.7);
        let iterated_sinz = IteratedSinZDescriptor::new(c);

        assert_eq!(iterated_sinz.max_iteration(), 50);
    }
}
