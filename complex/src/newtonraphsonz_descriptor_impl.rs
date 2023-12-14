use crate::complex_operations::ComplexOperations;
use crate::fractal_operations::FractalOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::NewtonRaphsonZ3Descriptor;
use shared::types::fractal_descriptor::NewtonRaphsonZ4Descriptor;

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
/// Fonction générique pour itérer sur un point complexe selon la méthode Newton-Raphson.
///
/// Cette fonction prend un point complexe, un nombre maximal d'itérations et la puissance du polynôme.
/// Elle itère sur le point complexe en utilisant la méthode Newton-Raphson et retourne le nombre
/// d'itérations nécessaires pour converger ou atteindre le nombre maximal d'itérations.
///
/// # Arguments
///
/// * `complex_point` - Un point complexe à itérer.
/// * `max_iteration` - Le nombre maximal d'itérations à effectuer.
/// * `power` - La puissance du polynôme (3 pour Z^3, 4 pour Z^4).
///
/// # Returns
///
/// Le nombre d'itérations nécessaires pour converger ou atteindre le nombre maximal d'itérations.

fn iterate_common(complex_point: &Complex, max_iteration: u16, power: u32) -> Result<u16, String> {
    let mut z = *complex_point;
    let mut iteration = 0u16;
    let convergence_threshold = 1e-12; // carré de 1e-6 pour la comparaison directe avec magnitude_squared
    while iteration < max_iteration {
        let (fz, dfz) = match power {
            3 => (
                z.square().mul(&z).sub(&Complex::new(1.0, 0.0)), // z^3 - 1
                z.square().mul(&Complex::new(3.0, 0.0)),         // 3z^2
            ),
            4 => (
                z.square().mul(&z).mul(&z).sub(&Complex::new(1.0, 0.0)), // z^4 - 1
                z.square().mul(&z).mul(&Complex::new(4.0, 0.0)),         // 4z^3
            ),
            _ => return Err(format!("Unsupported power: {}", power)),
        };

        if dfz.magnitude_squared() < convergence_threshold {
            break;
        }
        let fz_over_dfz = fz.div(dfz);
        let new_z = z.sub(&fz_over_dfz);

        if new_z.sub(&z).magnitude_squared() < convergence_threshold {
            break;
        }
        z = new_z;
        iteration += 1;
    }
    Ok(iteration)
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
    /// Le nombre d'itérations nécessaires pour converger ou atteindre le nombre maximal d'itérations.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        match iterate_common(complex_point, max_iteration, 3) {
            Ok(iteration) => iteration,
            Err(e) => {
                eprintln!("{}", e);
                0
            }
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
    /// Le nombre d'itérations nécessaires pour converger ou atteindre le nombre maximal d'itérations.
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        match iterate_common(complex_point, max_iteration, 4) {
            Ok(iteration) => iteration,
            Err(e) => {
                eprintln!("{}", e);
                0
            }
        }
    }
}
