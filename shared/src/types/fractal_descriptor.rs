use serde::{Deserialize, Serialize};

use crate::types::complex::Complex;

/// General descriptor for a fractal, encompassing different fractal types.
///
/// Attributes:
/// - `fractal_type`: A variant of `FractalType` specifying the type of fractal and its parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FractalDescriptor {
    #[serde(flatten)]
    pub fractal_type: FractalType,
}

/// Represents the type of fractal to be generated.
///
/// Variants:
/// - `Julia(JuliaDescriptor)`: Represents a Julia fractal with its specific descriptor.
/// - `Mandelbrot(MandelbrotDescriptor)`: Represents a Mandelbrot fractal (currently commented out).
/// - `BurningShip(BurningShipDescriptor)`: Represents a BurningShip fractal with its specific descriptor.
/// - `...`: Placeholder for additional fractal types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FractalType {
    Julia(JuliaDescriptor),
    IteratedSinZ(IteratedSinZDescriptor),
    Mandelbrot(MandelbrotDescriptor),
    NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
    NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor),
    BurningShip(BurningShipDescriptor),
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
/// - `c`: A `Complex` number representing the constant parameter of the IteratedSinZ set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IteratedSinZDescriptor {
    pub c: Complex,
}

/// Describes parameters specific to a Mandelbrot fractal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MandelbrotDescriptor {}
/// Describes parameters specific to a BurningShip fractal.
///
/// Attributes:
/// - `c`: A `Complex` number representing the constant parameter of the BurningShip set.
/// - `divergence_threshold_square`: The square of the divergence threshold. Points whose magnitude square exceeds this threshold are considered to diverge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurningShipDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}


/// Describes parameters specific to a Newton-Raphson z3 fractal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewtonRaphsonZ3Descriptor {}

/// Describes parameters specific to a Newton-Raphson z3 fractal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewtonRaphsonZ4Descriptor {}
