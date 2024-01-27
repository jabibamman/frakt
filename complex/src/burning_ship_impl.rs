use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::BurningShipDescriptor;
use shared::types::pixel_intensity::PixelIntensity;

/// Provides operations specific to the BurningShip fractal.
pub trait BurningShipOperations {
    /// Constructs a new `BurningShipDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex, divergence_threshold_square: f64) -> Self;

    /// Returns the square of the divergence threshold.
    fn divergence_threshold_square(&self) -> f64;

    /// Returns a reference to the complex number `c` used in the Julia fractal formula.
    fn c(&self) -> &Complex;
}

impl FractalOperations for BurningShipDescriptor {
    fn compute_pixel_intensity(
        &self,
        complex_point: &Complex,
        max_iteration: u16,
    ) -> PixelIntensity {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut num_iterations = 0;
        while ((x * x + y * y) < self.divergence_threshold_square) && num_iterations < max_iteration
        {
            let x_next = (x * x) - (y * y) + complex_point.re;
            let y_next = 2.0 * (x * y).abs() + complex_point.im;

            x = x_next;
            y = y_next;
            num_iterations = num_iterations + 1;
        }

        //(num_iterations, x * x + y * y)
        PixelIntensity {
            zn: (x * x + y * y) as f32,
            count: num_iterations as f32 / max_iteration as f32,
        }
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

    fn c(&self) -> &Complex {
        &self.c
    }
}
