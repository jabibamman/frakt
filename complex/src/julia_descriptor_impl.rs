use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::JuliaDescriptor;
use shared::types::resolution::Resolution;

/// Provides operations specific to the Julia fractal.
pub trait JuliaOperations {
    /// Constructs a new `JuliaDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex, divergence_threshold_square: f64) -> Self;

    /// Converts pixel coordinates to a complex number based on the resolution.
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex;

    /// Iterates over a complex point and returns the number of iterations before it diverges.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16;

    /// Returns the square of the divergence threshold.
    fn divergence_threshold_square(&self) -> f64;

    /// Returns a reference to the complex number `c` used in the Julia fractal formula.
    fn c(&self) -> &Complex;
}

impl JuliaOperations for JuliaDescriptor {
    fn new(c: Complex, divergence_threshold_square: f64) -> Self {
        Self {
            c,
            divergence_threshold_square,
        }
    }

    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        let re = (x as f64 / resolution.nx as f64) * 4.0 - 2.0;
        let im = (y as f64 / resolution.ny as f64) * 4.0 - 2.0;
        Complex::new(re, im)
    }

    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        fn recursive_iterate(z: Complex, c: &Complex, divergence_threshold_square: f64, max_iteration: u16, current_iteration: u16) -> u16 {
            if z.norm() > divergence_threshold_square || current_iteration == max_iteration {
                current_iteration
            } else {
                let next_z = z.square().add(c);
                recursive_iterate(next_z, c, divergence_threshold_square, max_iteration, current_iteration + 1)
            }
        }

        recursive_iterate(complex_point.clone(), &self.c, self.divergence_threshold_square, max_iteration, 0)
    }

    fn divergence_threshold_square(&self) -> f64 {
        self.divergence_threshold_square
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}
