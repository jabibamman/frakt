use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::MandelbrotDescriptor;
use shared::types::pixel_intensity::PixelIntensity;

/// Trait définissant les opérations de fractale de Mandelbrot.
pub trait MandelbrotOperations {
    /// Crée une nouvelle instance du type de fractale de Mandelbrot.
    fn new() -> Self;
}
impl MandelbrotOperations for MandelbrotDescriptor {
    /// Crée une nouvelle instance du type de fractale de Mandelbrot.
    fn new() -> Self {
        Self {}
    }
}
impl FractalOperations for MandelbrotDescriptor {
    /// Calcule l'intensité d'un pixel selon la méthode de la fractale de Mandelbrot.
    /// 
    /// Cette fonction prend un point complexe et un nombre maximal d'itérations.
    /// 
    /// # Arguments
    /// 
    /// * `complex_point` - Un point complexe à itérer.
    /// * `max_iteration` - Le nombre maximal d'itérations à effectuer.
    /// 
    /// # Returns
    /// 
    /// L'intensité du pixel.
    /// 
    fn compute_pixel_intensity(&self, complex_point: &Complex, max_iteration: u16) -> PixelIntensity {
        let (mut z, mut i) = (Complex::new(0.0, 0.0), 0);

        while z.magnitude_squared() <= 4.0 && i < max_iteration {
            z = z.square().add(complex_point);
            i += 1;
        }

        PixelIntensity {
            zn: (z.magnitude_squared() / 4.0) as f32, 
            count: i as f32 / max_iteration as f32, 
        }
    }
}