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
    fn iterate_complex_point(&self, complex_point: &Complex,  _max_iteration: u16) -> u16 {
        let mut z0 = Complex::new(1.0, 0.0);
        let mut iterations = 0;
        let mut z1 = z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
        let e: f64 = std::f64::consts::E;
        let max = 256;
        while ((z1 .sub(&z0)).abs()*(z1 .sub(&z0)).abs())>= 1.0 * e - 6.0 && iterations<max {
            z0=z1;
            z1=z0.sub(&self.pz(&z0).div(self.pz_prime(&z0))).add(complex_point);
            iterations += 1;
        }
        //println!("la variable es : {}",iterations);

        iterations
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
            //println!("{}",iterations);
        }

        iterations
    }

}





