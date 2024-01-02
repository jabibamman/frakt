use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::IteratedSinZDescriptor;
use shared::types::pixel_intensity::PixelIntensity;

/// Provides operations specific to the Iterated Sin(z) fractal.
pub trait IteratedSinZOperations {
    /// Constructs a new `IteratedSinZDescriptor` with the specified complex number and divergence threshold.
    fn new(c: Complex) -> Self;

    ///Fixed to 50
    fn max_iteration(&self) -> u16;

    /// Returns a reference to the complex number `c` used in the IteratedSinZ fractal formula.
    fn c(&self) -> &Complex;

    /// Returns the divergence threshold `η` used in the IteratedSinZ fractal formula.
    /// 
    /// # Description
    /// 
    /// In the context of the Iterated SinZ fractal, this divergence threshold (η) is used to determine when
    /// the iterations should stop.
    fn η(&self) -> f64;
}

impl FractalOperations for IteratedSinZDescriptor {
    /// Computes the pixel intensity of a complex point.
    /// 
    /// # Description
    /// 
    /// This function takes a complex point and a maximum number of iterations.
    /// 
    /// # Arguments
    /// 
    /// * `complex_point` - A complex point to iterate.
    /// * `max_iteration` - The maximum number of iterations to perform.
    /// 
    /// # Returns
    /// 
    /// The pixel intensity.
    /// 
    fn compute_pixel_intensity(&self, complex_point: &Complex, max_iteration: u16) -> PixelIntensity {
        let mut z = *complex_point;
        let mut iterations = 0;

        while z.magnitude_squared() < self.η() && iterations < max_iteration {
            z = z.sin().mul(&self.c); // f(z_n) = sin(z_n) * c
            iterations += 1;
        }

        PixelIntensity {
            zn: (z.magnitude_squared() / self.η()) as f32, // η = 50
            count: iterations as f32 / max_iteration as f32, 
        }
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

    fn η(&self) -> f64 {
        50.0
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
