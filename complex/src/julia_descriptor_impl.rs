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
                let next_z = z.square().add(c);
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

    /// Computes the pixel intensity of a complex point.
    fn compute_pixel_intensity(&self, complex_point: &Complex, max_iteration: u16) -> PixelIntensity {
        let (mut z, mut i) = (complex_point.clone(), 0);

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
