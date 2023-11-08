use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::fractal_descriptor::JuliaDescriptor;
use shared::types::resolution::Resolution;

pub trait JuliaOperations {
    fn new(c: Complex, divergence_threshold_square: f64) -> Self;
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex;
    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16;
    fn divergence_threshold_square(&self) -> f64;
    fn c(&self) -> &Complex;
}

impl JuliaOperations for JuliaDescriptor {
    fn new(c: Complex, divergence_threshold_square: f64) -> Self {
        Self {
            c,
            divergence_threshold_square,
        }
    }

    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        let re = (x as f64 / resolution.nx as f64) * 4.0 - 2.0;
        let im = (y as f64 / resolution.ny as f64) * 4.0 - 2.0;
        Complex::new(re, im)
    }

    fn iterate_complex_point(&self, complex_point: &Complex, max_iteration: u16) -> u16 {
        let mut z = complex_point.clone();
        let mut iterations = 0;

        while z.norm() <= self.divergence_threshold_square && iterations < max_iteration {
            z = z.square().add(&self.c);
            iterations += 1;
        }

        iterations
    }

    fn divergence_threshold_square(&self) -> f64 {
        self.divergence_threshold_square
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}
