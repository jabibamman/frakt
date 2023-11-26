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
impl FractalOperations for NewtonRaphsonZ3Descriptor {
    fn iterate_complex_point(&self, complex_point: &Complex,  max_iteration: u16) -> u16 {
        let mut z0 = Complex::new(1.0, 0.0);
        let mut iterations = 0;
        let mut z1 = z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
        let e: f64 = std::f64::consts::E;
        while z1 .sub(&z0).abs().sqrt()>= 1.0 * e - 6.0  {
            z0=z1;
            z1=z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
            iterations += 1;
        }

        iterations
    }

}
impl FractalOperations for NewtonRaphsonZ4Descriptor {
    fn iterate_complex_point(&self, complex_point: &Complex , max_iteration: u16) -> u16 {
        let mut z0 = Complex::new(1.0, 0.0);
        let mut iterations = 0;
        let mut z1 = z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
        let e: f64 = std::f64::consts::E;
        while z1.sub(&z0).abs().sqrt()>= 1.0 * e - 6.0 {
            z0=z1;
            z1=z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
            iterations += 1;
        }

        iterations
    }

}

impl NewtonRaphsonOperations for NewtonRaphsonZ3Descriptor {
    fn new() -> Self {
        Self {
        }
    }
    fn pz(&self,z: &Complex) -> Complex{
        let mut z=z.clone();
        let mut z0=Complex::new(1.0,0.0);
        let mut result = z.mul(&z.mul(&z)).sub(&z0);
        result

    }
    fn pz_prime(&self, z:&Complex) -> Complex{
        let mut z=z.clone();
        let mut z3=Complex::new(3.0, 0.0);
        let mut result = z3 .mul(&z.mul(&z)) ;
        result

    }

}
impl NewtonRaphsonOperations for NewtonRaphsonZ4Descriptor {
    fn new() -> Self {
        Self {
        }
    }
    fn pz(&self,z: &Complex) -> Complex{
        let mut z=z.clone();
        let mut z0=Complex::new(1.0,0.0);
        let mut result = z.mul(&z.mul(&z)).mul(&z) .sub(&z0);
        result

    }
    fn pz_prime(&self, z: &Complex) -> Complex{
        let mut z=z.clone();
        let mut z3=Complex::new(3.0, 0.0);
        let mut result = z3 .mul(&z.mul(&z)) ;
        result

    }



}





