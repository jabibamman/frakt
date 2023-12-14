use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::MandelbrotDescriptor;
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
    /// Itère sur un point complexe selon la méthode de la fractale de Mandelbrot.
    ///
    /// Cette fonction prend un point complexe et un nombre maximal d'itérations.
    /// Elle itère sur le point complexe en utilisant la méthode de la fractale de Mandelbrot
    /// et retourne le nombre d'itérations nécessaires pour converger ou atteindre le nombre maximal d'itérations.
    ///
    /// # Arguments
    ///
    /// * `complex_point` - Un point complexe à itérer.
    /// * `max_iteration` - Le nombre maximal d'itérations à effectuer.
    ///
    /// # Returns
    ///
    /// Le nombre d'itérations nécessaires pour converger ou atteindre le nombre maximal d'itérations.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;
        while z.abs() * &z.abs() <= 4.0 && iterations < max_iteration {
            z = z.square().add(complex_point);
            iterations += 1;
        }

        iterations
    }
}
