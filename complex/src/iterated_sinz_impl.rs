use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::IteratedSinZDescriptor;

/// Provides operations specific to the Iterated Sin(z) fractal.
pub trait IteratedSinZOperations {
    /// Constructs a new `IteratedSinZDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex) -> Self;

    ///Fixed to 50
    fn max_iteration(&self) -> u16;

    /// Returns a reference to the complex number `c` used in the IteratedSinZ fractal formula.
    fn c(&self) -> &Complex;
}

impl FractalOperations for IteratedSinZDescriptor {
    fn iterate_complex_point(&self, complex_point: &Complex, _max_iteration: u16) -> u16 {
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
}

impl IteratedSinZOperations for IteratedSinZDescriptor {
    fn new(c: Complex) -> Self {
        Self { c }
    }

    fn max_iteration(&self) -> u16 {
        50
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}

#[cfg(test)]
mod iterated_sinz_tests {
    use crate::complex_operations::ComplexOperations;
    use crate::iterated_sinz_impl::IteratedSinZOperations;
    use shared::types::complex::Complex;
    use shared::types::fractal_descriptor::IteratedSinZDescriptor;

    #[test]
    fn test_max_iteration_return_50() {
        let c = Complex::new(4.5, -65.7);
        let iterated_sinz = IteratedSinZDescriptor::new(c);

        assert_eq!(iterated_sinz.max_iteration(), 50);
    }
}
