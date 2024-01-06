use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::NewtonRaphsonZ3Descriptor;
use shared::types::fractal_descriptor::NewtonRaphsonZ4Descriptor;
use shared::types::pixel_intensity::PixelIntensity;

pub trait NewtonRaphsonOperations {
    /// Crée une nouvelle instance du type de fractale.
    fn new() -> Self;
}
impl NewtonRaphsonOperations for NewtonRaphsonZ3Descriptor {
    /// Crée une nouvelle instance du type de fractale Newton-Raphson Z^3.
    fn new() -> Self {
        Self {}
    }
}
impl NewtonRaphsonOperations for NewtonRaphsonZ4Descriptor {
    /// Crée une nouvelle instance du type de fractale Newton-Raphson Z^4.
    fn new() -> Self {
        Self {}
    }
}

impl FractalOperations for NewtonRaphsonZ3Descriptor {
    /// Itère sur un point complexe selon la méthode Newton-Raphson pour Z^3.
    ///
    /// # Arguments
    ///
    /// * `complex_point` - Un point complexe à itérer.
    /// * `max_iteration` - Le nombre maximal d'itérations à effectuer.
    ///
    /// # Returns
    ///
    /// L'intensité du pixel (PixelIntensity {zn, count}).
    ///
    fn compute_pixel_intensity(
        &self,
        complex_point: &Complex,
        max_iteration: u16,
    ) -> PixelIntensity {
        let mut z = *complex_point;
        let mut iterations = 0;

        while iterations < max_iteration {
            let next_z = newton_raphson_step(&z, 3);
            if (next_z.sub(&z)).magnitude_squared() < 1e-6 {
                break;
            }
            z = next_z;
            iterations += 1;
        }

        PixelIntensity {
            zn: (0.5 + z.arg() / (2.0 * std::f64::consts::PI)) as f32,
            count: iterations as f32 / max_iteration as f32,
        }
    }
}
impl FractalOperations for NewtonRaphsonZ4Descriptor {
    /// Itère sur un point complexe selon la méthode Newton-Raphson pour Z^4.
    ///
    /// # Arguments
    ///
    /// * `complex_point` - Un point complexe à itérer.
    /// * `max_iteration` - Le nombre maximal d'itérations à effectuer.
    ///
    /// # Returns
    ///
    /// L'intensité du pixel (PixelIntensity {zn, count}).
    ///
    fn compute_pixel_intensity(
        &self,
        complex_point: &Complex,
        max_iteration: u16,
    ) -> shared::types::pixel_intensity::PixelIntensity {
        let mut z = complex_point.clone();
        let mut iterations = 0;

        while iterations < max_iteration {
            let next_z = newton_raphson_step(&z, 4);
            if (next_z.sub(&z)).magnitude_squared() < 1e-6 {
                break;
            }
            z = next_z;
            iterations += 1;
        }

        PixelIntensity {
            zn: (0.5 + z.arg() / (2.0 * std::f64::consts::PI)) as f32,
            count: iterations as f32 / max_iteration as f32,
        }
    }
}

/// Calculates the next step in the Newton-Raphson method for a polynomial of degree 3 or 4.
///
/// # Arguments
///
/// * `z` - Le point complexe actuel.
/// * `degree` - La puissance du polynôme (3 pour Z^3, 4 pour Z^4).
///
/// # Returns
///
/// Le prochain point complexe dans l'itération pour NewtonRaphsonZ3.
fn newton_raphson_step(z: &Complex, degree: u32) -> Complex {
    match degree {
        3 => {
            // p(z) = z^3 - 1 et p'(z) = 3z^2
            let pz = z.square().mul(z).sub(&Complex::new(1.0, 0.0));
            let dpz = z.square().mul(&Complex::new(3.0, 0.0));
            z.sub(&pz.div(dpz))
        }
        4 => {
            //  p(z) = z^4 - 1 et p'(z) = 4z^3
            let pz = z.square().square().sub(&Complex::new(1.0, 0.0));
            let dpz = z.square().mul(z).mul(&Complex::new(4.0, 0.0));
            z.sub(&pz.div(dpz))
        }
        _ => panic!("Unsupported polynomial degree"),
    }
}
