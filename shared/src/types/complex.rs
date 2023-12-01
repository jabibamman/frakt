/// A `Complex` number with real (`re`) and imaginary (`im`) parts.
///
/// # Attributes
/// * `re` - The real part of the complex number, represented as a `f64`.
/// * `im` - The imaginary part of the complex number, also a `f64`.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
