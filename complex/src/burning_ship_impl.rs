use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::BurningShipDescriptor;

/// Provides operations specific to the BurningShip fractal.
pub trait BurningShipOperations {
    /// Constructs a new `BurningShipDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex, divergence_threshold_square: f64) -> Self;

    /// Returns the square of the divergence threshold.
    fn divergence_threshold_square(&self) -> f64;
}

impl FractalOperations for BurningShipDescriptor {
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        fn recursive_iterate(
            z: Complex,
            c: &Complex,
            divergence_threshold_square: f64,
            max_iteration: u16,
            current_iteration: u16,
        ) -> u16 {
            if z.norm() > divergence_threshold_square || current_iteration == max_iteration {
                current_iteration
            } else {
                // next_z = (|(z.re)| - |(z.im)|)^2 + c
                let next_z = Complex::new(z.re.abs(), z.im.abs()).square().add(c);
                recursive_iterate(
                    next_z,
                    c,
                    divergence_threshold_square,
                    max_iteration,
                    current_iteration + 1,
                )
            }
        }

        recursive_iterate(
            complex_point.clone(),
            &self.c,
            self.divergence_threshold_square,
            max_iteration,
            0,
        )
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}

impl BurningShipOperations for BurningShipDescriptor {
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
