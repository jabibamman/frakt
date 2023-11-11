use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::NewtonRaphsonZ3Descriptor;
use shared::types::fractal_descriptor::NewtonRaphsonZ4Descriptor;

pub trait NewtonRaphsonOperations {
    fn new() -> Self;
    fn iterate_complex_point(&self, complex_point: &Complex) -> u16;
    fn pz(&self,z: Complex) -> Complex;
    fn pz_prime(&self, z:Complex) -> Complex;
}

impl NewtonRaphsonOperations for NewtonRaphsonZ3Descriptor {
    fn new() -> Self {
        Self {
        }
    }
    fn pz(&self,z: Complex) -> Complex{
        let mut result = z.mul(&z.mul((&z)))-1;
        result

    }
    fn pz_prime(&self, z:Complex) -> Complex{
        let mut z3=Complex::new(3.0, 0.0);
        let mut result = z3 * z.mul(&z);
        result

    }


    fn iterate_complex_point(&self, complex_point: &Complex, ) -> u16 {
   //     let mut z0 = Complex::new(1.0, 0.0);
       let mut iterations = 0;
 //      let mut z1 = z0.sub(&self.pz(z0).div(&self.pz_prime(z0)));
 /* pas encore demain je continue ça        while z.norm() <= 4.0 {
            z = z.square().add(complex_point);
            iterations += 1;
        }
        */
        iterations
    }





}
/*impl NewtonRaphsonOperations for NewtonRaphsonZ4Descriptor {
    fn new() -> Self {
        Self {
        }
    }

    fn iterate_complex_point(&self, complex_point: &Complex) -> u16 {
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;
        while z.norm() <= 4.0 {
            z = z.square().add(complex_point);
            iterations += 1;
        }
        iterations
    }


}
*/


