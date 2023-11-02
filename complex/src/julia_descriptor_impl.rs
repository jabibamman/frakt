use shared::types::julia_descriptor::{JuliaDescriptor};
use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::resolution::Resolution;


pub trait JuliaOperations {
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex;
    fn iterate_complex_point(&self, complex_point: &Complex) -> u16;
    fn new(c: Complex, divergence_threshold_square: f64, max_iteration: u16) -> Self;
    fn divergence_threshold_square(&self) -> f64;
    fn max_iteration(&self) -> u16;
    fn c(&self) -> &Complex;
}

impl JuliaOperations for JuliaDescriptor {
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        let re = (x as f64 / resolution.nx as f64) * 4.0 - 2.0;
        let im = (y as f64 / resolution.ny as f64) * 4.0 - 2.0;
        Complex::new(re, im)
    }

    fn iterate_complex_point(&self, complex_point: &Complex) -> u16 {
        let mut z = complex_point.clone();
        let mut iterations = 0;

        while z.norm() <= self.divergence_threshold_square && iterations < self.max_iteration {
            z = z.square().add(&self.c);
            iterations += 1;
        }

        iterations
    }

    fn new(c: Complex, divergence_threshold_square: f64, max_iteration: u16) -> Self {
        Self {
            c,
            divergence_threshold_square,
            max_iteration,
        }
    }

    fn divergence_threshold_square(&self) -> f64 {
        self.divergence_threshold_square
    }


    fn max_iteration(&self) -> u16 {
        self.max_iteration
    }

    fn c(&self) -> &Complex {
        &self.c
    }
}
