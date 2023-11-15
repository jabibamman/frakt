use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::JuliaDescriptor;
use crate::fractal_operations::FractalOperations;

/// Provides operations specific to the Julia fractal.
pub trait JuliaOperations {
    /// Constructs a new `JuliaDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex, divergence_threshold_square: f64) -> Self;

    /// Returns the square of the divergence threshold.
    fn divergence_threshold_square(&self) -> f64;
}


impl FractalOperations for JuliaDescriptor {
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        let mut z = complex_point.clone();
        let mut iterations = 0;

        while z.norm() <= self.divergence_threshold_square && iterations < max_iteration {
            z = z.square().add(&self.c);
            iterations += 1;
        }

        iterations
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}


impl JuliaOperations for JuliaDescriptor {
    fn new(c: Complex, divergence_threshold_square: f64) -> Self {
        Self {
            c,
            divergence_threshold_square,
        }
    }

    fn divergence_threshold_square(&self) -> f64 {
        self.divergence_threshold_square
    }
}