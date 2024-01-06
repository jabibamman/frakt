use serde::{Deserialize, Serialize};

use crate::types::complex::Complex;

/// Represents the type of fractal to be generated.
///
/// Variants:
/// - `Julia(JuliaDescriptor)`: Represents a Julia fractal with its specific descriptor.
/// - `Mandelbrot(MandelbrotDescriptor)`: Represents a Mandelbrot fractal (currently commented out).
/// - `...`: Placeholder for additional fractal types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FractalType {
    Julia(JuliaDescriptor),
    IteratedSinZ(IteratedSinZDescriptor),
    Mandelbrot(MandelbrotDescriptor),
    NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
    NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor),
}

/// Describes parameters specific to a Julia fractal.
///
/// Attributes:
/// - `c`: A `Complex` number representing the constant parameter of the Julia set.
/// - `divergence_threshold_square`: The square of the divergence threshold. Points whose magnitude square exceeds this threshold are considered to diverge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

/// Describes parameters specific to a Mandelbrot fractal.
///
/// Attributes:
/// - `divergence_threshold_square`: The square of the divergence threshold. Points whose magnitude square exceeds this threshold are considered to diverge.
/// - `max_iteration`: Maximum number of iterations to determine whether a point diverges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IteratedSinZDescriptor {
    pub c: Complex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MandelbrotDescriptor {}

/// General descriptor for a fractal, encompassing different fractal types.
///
/// Attributes:
/// - `fractal_type`: A variant of `FractalType` specifying the type of fractal and its parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FractalDescriptor {
    pub fractal_type: FractalType,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewtonRaphsonZ3Descriptor {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewtonRaphsonZ4Descriptor {}
