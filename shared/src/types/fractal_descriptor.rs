use crate::types::complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub enum FractalType {
    Julia(JuliaDescriptor),
    //Mandelbrot(MandelbrotDescriptor),
    //...
}

#[derive(Debug, Clone, PartialEq)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MandelbrotDescriptor {
}

#[derive(Debug, Clone, PartialEq)]
pub struct FractalDescriptor {
    pub fractal_type: FractalType,
}
