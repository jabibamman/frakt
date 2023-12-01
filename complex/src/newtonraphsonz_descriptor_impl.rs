use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::NewtonRaphsonZ3Descriptor;
use shared::types::fractal_descriptor::NewtonRaphsonZ4Descriptor;
use crate::fractal_operations::FractalOperations;

pub trait NewtonRaphsonOperations {
    fn new() -> Self;
    fn pz(&self,z: &Complex) -> Complex;
    fn pz_prime(&self, z:&Complex) -> Complex;
}

impl NewtonRaphsonOperations for NewtonRaphsonZ3Descriptor {
    fn new() -> Self {
        Self {
        }
    }

    fn pz_prime(&self, z:&Complex) -> Complex{
        let z=z.clone();
        let z3 = Complex::new(3.0, 0.0);
        let result = z3 .mul(&z.mul(&z)) ;
        result

    }
    fn pz(&self,z: &Complex) -> Complex{
        let z=z.clone();
        let z0=Complex::new(1.0,0.0);
        let result = z.mul(&z.mul(&z)).sub(&z0);
        result

    }

}
impl NewtonRaphsonOperations for NewtonRaphsonZ4Descriptor {
    fn new() -> Self {
        Self {
        }
    }
    fn pz_prime(&self, z: &Complex) -> Complex{
        let z=z.clone();
        let z3=Complex::new(4.0, 0.0);
        let result = z3 .mul(&z.mul(&z)).mul(&z) ;
        result

    }
    fn pz(&self,z: &Complex) -> Complex{
        let z=z.clone();
        let z0=Complex::new(1.0,0.0);
        let result = z.mul(&z.mul(&z)).mul(&z) .sub(&z0);
        result

    }
}
impl FractalOperations for NewtonRaphsonZ3Descriptor {
    /// Iterates over a complex point.
    ///
    /// This function takes a reference to a `Complex` number and an iteration limit (`_max_iteration`).
    /// It iterates the complex point using a specific mathematical process (likely related to Newton's method
    /// for finding roots of a complex function), and returns the number of iterations it took to converge
    /// or reach the maximum iterations.
    ///
    /// # Parameters
    /// * `complex_point` - A reference to the complex number to iterate over.
    /// * `max_iteration` - The maximum number of iterations to perform.
    ///
    /// # Returns
    /// * `u16` - The number of iterations it took for the process to converge, or the maximum iteration
    /// limit if convergence did not occur.
    ///
    /// # Notes
    /// * The function utilizes a convergence threshold to determine when the iterative process
    /// has sufficiently converged. 
    /// * It uses Newton's method for root-finding of the function `z^3 - 1`.
    fn iterate_complex_point(&self, complex_point: &Complex,  max_iteration: u16) -> u16 {
        let mut z = *complex_point;
        let mut iteration = 0u16;
        let convergence_threshold = 1e-12; // carré de 1e-6 pour la comparaison directe avec magnitude_squared

        while iteration < max_iteration {
            let fz = z.square().mul(&z).sub(&Complex::new(1.0, 0.0)); // z^3 - 1
            let dfz = z.square().mul(&Complex::new(3.0, 0.0)); // 3z^2

            if dfz.magnitude_squared() < convergence_threshold {
                break;
            }

            let fz_over_dfz = fz.div(dfz); // f(z) / f'(z)
            let new_z = z.sub(&fz_over_dfz); // z - f(z) / f'(z)

            if new_z.sub(&z).magnitude_squared() < convergence_threshold {
                break;
            }

            z = new_z; // z = z - f(z) / f'(z)
            iteration += 1;
        }

        iteration
    }
}

impl FractalOperations for NewtonRaphsonZ4Descriptor {
    fn iterate_complex_point(&self, complex_point: &Complex , _max_iteration: u16) -> u16 {
        let mut z0 = Complex::new(1.0, 0.0);
        let mut iterations = 0;
        let mut z1 = z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
        let e: f64 = std::f64::consts::E;
        let max = 256;
        while (z1.sub(&z0).abs()*z1.sub(&z0).abs()>= 1.0 * e - 6.0) && iterations<max  {
            //println!("   {}",(z1.sub(&z0).abs()*z1.sub(&z0).abs()>= 1.0 * e - 6.0));
            z0=z1;
            z1=z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
            //println!("la valeur de z0 est {} {}",z0.im,z0.re);
            //println!("la valeur de z1 est {} {}",z1.im,z1.re);
            iterations += 1;
            println!("{}",iterations);
        }

        println!("la variable es : {}",iterations);
        iterations
    }

}





