use crate::types::complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub enum FractalType {
    Julia(JuliaDescriptor),
    Mandelbrot(MandelbrotDescriptor),
    NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
    NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor)

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
#[derive(Debug, Clone, PartialEq)]
pub struct NewtonRaphsonZ3Descriptor{
}
#[derive(Debug, Clone, PartialEq)]
pub struct NewtonRaphsonZ4Descriptor{
}

