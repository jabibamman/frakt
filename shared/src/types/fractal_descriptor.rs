use crate::types::complex::Complex;

/// Represents the type of fractal to be generated.
///
/// Variants:
/// - `Julia(JuliaDescriptor)`: Represents a Julia fractal with its specific descriptor.
/// - `Mandelbrot(MandelbrotDescriptor)`: Represents a Mandelbrot fractal (currently commented out).
/// - `BurningShip(BurningShipDescriptor)`: Represents a BurningShip fractal with its specific descriptor.
/// - `...`: Placeholder for additional fractal types.
#[derive(Debug, Clone, PartialEq)]
pub enum FractalType {
    Julia(JuliaDescriptor),
    IteratedSinZ(IteratedSinZDescriptor),
    BurningShip(BurningShipDescriptor),
    //Mandelbrot(MandelbrotDescriptor),
    //...
}

/// Describes parameters specific to a Julia fractal.
///
/// Attributes:
/// - `c`: A `Complex` number representing the constant parameter of the Julia set.
/// - `divergence_threshold_square`: The square of the divergence threshold. Points whose magnitude square exceeds this threshold are considered to diverge.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

/// Describes parameters specific to a Mandelbrot fractal.
///
/// Attributes:
/// - `divergence_threshold_square`: The square of the divergence threshold. Points whose magnitude square exceeds this threshold are considered to diverge.
/// - `max_iteration`: Maximum number of iterations to determine whether a point diverges.
#[derive(Debug, Clone, PartialEq)]
pub struct IteratedSinZDescriptor {
    pub c: Complex,
}

/// Describes parameters specific to a BurningShip fractal.
///
/// Attributes:
/// - `c`: A `Complex` number representing the constant parameter of the BurningShip set.
/// - `escape_time`: Maximum number of iterations to determine whether a point escapes.
/// - `divergence_threshold_square`: The square of the divergence threshold. Points whose magnitude square exceeds this threshold are considered to diverge.
#[derive(Debug, Clone, PartialEq)]
pub struct BurningShipDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MandelbrotDescriptor {
    pub divergence_threshold_square: f64,
    pub max_iteration: u16,
}

/// General descriptor for a fractal, encompassing different fractal types.
///
/// Attributes:
/// - `fractal_type`: A variant of `FractalType` specifying the type of fractal and its parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct FractalDescriptor {
    pub fractal_type: FractalType,
}
