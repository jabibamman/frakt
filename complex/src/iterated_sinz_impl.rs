use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::IteratedSinZDescriptor;
use shared::types::resolution::Resolution;

pub trait IteratedSinZOperations {
    fn new(c: Complex) -> Self;
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex;
    fn iterate_complex_point(&self, complex_point: &Complex) -> u16;
    fn max_iteration(&self) -> u16;
    fn c(&self) -> &Complex;
}

impl IteratedSinZOperations for IteratedSinZDescriptor {
    fn new(c: Complex) -> Self {
        Self {
            c,
        }
    }

    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        let re = (x as f64 / resolution.nx as f64) * 4.0 - 2.0;
        let im = (y as f64 / resolution.ny as f64) * 4.0 - 2.0;
        Complex::new(re, im)
    }

    fn iterate_complex_point(&self, complex_point: &Complex) -> u16 {
        let mut z = complex_point.clone();
        let mut iterations = 0;
        let max = 256;

        while complex_point.norm().powi(2) < self.max_iteration() as f64 && iterations < max {
            z = z.sin().mul(&self.c);
            iterations += 1;
        }

        iterations
    }

    ///Fixed to 50
    fn max_iteration(&self) -> u16 {
        50
    }

    ///Return the complex stored in IteratedSinZ
    fn c(&self) -> &Complex {
        &self.c
    }
}
