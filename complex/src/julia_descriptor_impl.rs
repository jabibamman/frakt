use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::JuliaDescriptor;
use shared::types::pixel_intensity::PixelIntensity;

/// Provides operations specific to the Julia fractal.
pub trait JuliaOperations {
    /// Constructs a new `JuliaDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex, divergence_threshold_square: f64) -> Self;

    /// Returns the square of the divergence threshold.
    fn divergence_threshold_square(&self) -> f64;

    /// Returns a reference to the complex number `c` used in the Julia fractal formula.
    fn c(&self) -> &Complex;
}

impl FractalOperations for JuliaDescriptor {
    /// Computes the pixel intensity of a complex point.
    fn compute_pixel_intensity(
        &self,
        complex_point: &Complex,
        max_iteration: u16,
    ) -> PixelIntensity {
        let (mut z, mut i) = (*complex_point, 0);

        while z.magnitude_squared() < self.divergence_threshold_square && i < max_iteration {
            z = z.square().add(&self.c);
            i += 1;
        }

        PixelIntensity {
            zn: z.norm() as f32,
            count: i as f32 / max_iteration as f32,
        }
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

    fn c(&self) -> &Complex {
        &self.c
    }
}
