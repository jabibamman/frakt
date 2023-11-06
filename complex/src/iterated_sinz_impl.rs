use crate::complex_operations::ComplexOperations;
use shared::types::complex::Complex;
use shared::types::iterated_sinz::IteratedSinZ;
use shared::types::resolution::Resolution;

pub trait IteratedSinZOperations {
    fn new(c: Complex, divergence_threshold_square: f64, max_iteration: u16) -> Self;
    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex;
    fn iterate_complex_point(&self, complex_point: &Complex) -> u16;
    fn divergence_threshold_square(&self) -> f64;
    fn max_iteration(&self) -> u16;
    fn c(&self) -> &Complex;
}

impl IteratedSinZOperations for IteratedSinZ {
    fn new(c: Complex, divergence_threshold_square: f64, max_iteration: u16) -> Self {
        todo!()
    }

    fn to_complex(&self, x: u16, y: u16, resolution: &Resolution) -> Complex {
        todo!()
    }

    fn iterate_complex_point(&self, complex_point: &Complex) -> u16 {
        todo!()
    }

    fn divergence_threshold_square(&self) -> f64 {
        todo!()
    }

    fn max_iteration(&self) -> u16 {
        todo!()
    }

    fn c(&self) -> &Complex {
        todo!()
    }
}