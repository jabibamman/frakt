use crate::types::complex::Complex;
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
    pub max_iteration: u16,
}
