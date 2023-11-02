use crate::types::complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub struct JuliaDescriptor {
    c: Complex,
    divergence_threshold_square: f64,
    max_iteration: u16,
}

pub trait JuliaDescriptorOperations {
    fn new(c: Complex, divergence_threshold_square: f64, max_iteration: u16) -> Self;
    fn get_c(&self) -> Complex;
    fn get_divergence_threshold_square(&self) -> f64;
    fn get_max_iteration(&self) -> u16;
}

impl JuliaDescriptorOperations for JuliaDescriptor {
    fn new(c: Complex, divergence_threshold_square: f64, max_iteration: u16) -> Self {
        JuliaDescriptor {
            c,
            divergence_threshold_square,
            max_iteration,
        }
    }

    fn get_c(&self) -> Complex {
        self.c.clone()
    }

    fn get_divergence_threshold_square(&self) -> f64 {
        self.divergence_threshold_square
    }

    fn get_max_iteration(&self) -> u16 {
        self.max_iteration
    }
}
