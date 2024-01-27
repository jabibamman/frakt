use shared::types::complex::Complex;

/// Provides a set of operations for complex number arithmetic.
pub trait ComplexOperations {
    /// Constructs a new complex number.
    fn new(re: f64, im: f64) -> Self;

    /// Adds two complex numbers and returns the result.
    fn add(&self, other: &Self) -> Self;

    /// Subtracts another complex number from this one and returns the result.
    fn sub(&self, other: &Self) -> Self;

    /// Multiplies two complex numbers and returns the result.
    fn mul(&self, other: &Self) -> Self;

    /// Squares the complex number and returns the result.
    fn square(&self) -> Self;

    /// Returns the squared magnitude of the complex number.
    fn magnitude_squared(&self) -> f64;

    /// Returns the Euclidean norm (magnitude) of the complex number.
    fn norm(&self) -> f64;

    /// Divides two complex numbers and returns the result.
    fn div(&self, other: Self) -> Self;

    /// Returns the absolute value of the complex number.
    fn abs(&self) -> f64;

    /// Returns the sine of the complex number.
    fn sin(&self) -> Self;

    /// Returns the exponential of the complex number.
    fn exp(&self) -> Self;

    /// Returns the argument of the complex number in radians.
    fn arg(&self) -> f64;
}

impl ComplexOperations for Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    fn add(&self, other: &Self) -> Self {
        Complex::new(self.re + other.re, self.im + other.im)
    }

    fn sub(&self, other: &Self) -> Self {
        Complex::new(self.re - other.re, self.im - other.im)
    }

    fn mul(&self, other: &Self) -> Self {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    fn square(&self) -> Self {
        self.clone().mul(&self)
    }

    fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    fn norm(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }
    fn div(&self, other: Self) -> Self {
        let denominator = other.magnitude_squared();
        Complex::new(
            (self.re * other.re + self.im * other.im) / denominator,
            (self.im * other.re - self.re * other.im) / denominator,
        )
    }
    fn abs(&self) -> f64 {
        self.norm()
    }

    ///Re(Sin(z)) = Sin(a) * Cosh(b)
    ///Im(Sin(z)) = Cos(a) * Sinh(b)
    fn sin(&self) -> Self {
        Complex::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }

    ///exp(a+ib) = exp(a) * exp(ib) = exp(a) * (cos(b) + isin(b))
    fn exp(&self) -> Self {
        let exp_re = Complex::new(self.re.exp(), 0.0);
        let exp_im = Complex::new(self.im.cos(), self.im.sin());
        exp_re.mul(&exp_im)
    }

    /// Returns the argument of the complex number in radians.
    fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }
}

#[cfg(test)]
mod complex_tests {
    use super::*;
    use std::f64::consts::{E, PI};

    // Series of tests for each operation, verifying the correctness
    // of complex number arithmetic such as addition, subtraction,
    // multiplication, squaring, and calculation of magnitude and norm.

    fn round(value: &f64) -> f64 {
        let accuracy = 10000000.0;
        (value * accuracy).round() / accuracy
    }

    #[test]
    fn test_add() {
        let a = Complex::new(-2.0, 5.0);
        let b = Complex::new(1.0, -3.0);
        let result = a.add(&b);

        assert_eq!(result.re, -1.0);
        assert_eq!(result.im, 2.0);
    }

    #[test]
    fn test_add_with_wikipedia_example() {
        let a = Complex::new(-2.0, 5.0);
        let b = Complex::new(1.0, -3.0);
        let result = a.add(&b);

        assert_eq!(result.re, -1.0);
        assert_eq!(result.im, 2.0);
    }

    #[test]
    fn test_sub_with_wikipedia_example() {
        let a = Complex::new(-2.0, 5.0);
        let b = Complex::new(1.0, -3.0);
        let result = a.sub(&b);

        assert_eq!(result.re, -3.0);
        assert_eq!(result.im, 8.0);
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(2.0, 3.0);
        let result = a.mul(&b);

        assert_eq!(result.re, -4.0);
        assert_eq!(result.im, 7.0);
    }

    #[test]
    fn test_mul_with_wikipedia_example() {
        let a = Complex::new(-2.0, 5.0);
        let b = Complex::new(1.0, -3.0);
        let result = a.mul(&b);

        assert_eq!(result.re, 13.0);
        assert_eq!(result.im, 11.0);
    }

    #[test]
    fn test_square() {
        let a = Complex::new(1.0, 2.0);
        let result = a.square();

        assert_eq!(result.re, -3.0);
        assert_eq!(result.im, 4.0);
    }

    #[test]
    fn test_magnitude_squared() {
        let a = Complex::new(1.0, 2.0);
        let result = a.magnitude_squared();

        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_norm() {
        let a = Complex::new(1.0, 2.0);
        let result = a.norm();

        assert_eq!(result, 5.0f64.sqrt());
    }

    #[test]
    fn test_exp_where_im_equal_0() {
        let a = Complex::new(0.0, 0.0);
        let result = a.exp();

        assert_eq!(result.re, 1.0);
        assert_eq!(result.im, 0.0);
    }

    #[test]
    fn test_exp_where_im_equal_half_pi() {
        let a = Complex::new(0.0, PI / 2.0);
        let result = a.exp();

        assert_eq!(round(&result.re), 0.0);
        assert_eq!(round(&result.im), 1.0);
    }

    #[test]
    fn test_exp_where_im_equal_minus_half_pi() {
        let a = Complex::new(0.0, -PI / 2.0);
        let result = a.exp();

        assert_eq!(round(&result.re), 0.0);
        assert_eq!(round(&result.im), -1.0);
    }

    #[test]
    fn test_exp_where_im_equal_pi() {
        let a = Complex::new(0.0, PI);
        let result = a.exp();

        assert_eq!(round(&result.re), -1.0);
        assert_eq!(round(&result.im), 0.0);
    }

    #[test]
    fn test_exp_where_im_equal_minus_pi() {
        let a = Complex::new(0.0, -PI);
        let result = a.exp();

        assert_eq!(round(&result.re), -1.0);
        assert_eq!(round(&result.im), 0.0);
    }

    #[test]
    fn test_exp_z_mul_exp_minus_z_equal_1() {
        let z = Complex::new(353.5, -4520.3);
        let left = z.exp();
        let right = z.mul(&Complex::new(-1.0, 0.0)).exp();
        let result = left.mul(&right);

        assert_eq!(round(&result.re), 1.0);
        assert_eq!(round(&result.im), 0.0);
    }

    #[test]
    fn test_exp_with_reel_1() {
        let z = Complex::new(1.0, PI);
        let result = z.exp();

        let minus_exp = -E;
        assert_eq!(round(&result.re), round(&minus_exp));
        assert_eq!(round(&result.im), 0.0);
    }

    #[test]
    fn test_exp_with_reel_2() {
        let z = Complex::new(2.0, PI / 4.0);
        let result = z.exp();

        let exp_value = E.powi(2) / 2.0_f64.sqrt();
        assert_eq!(round(&result.re), round(&result.im));
        assert_eq!(round(&result.re), round(&exp_value));
    }

    #[test]
    fn test_new() {
        let a = Complex::new(1.0, 2.0);

        assert_eq!(a.re, 1.0);
        assert_eq!(a.im, 2.0);
    }

    #[test]
    fn test_new_with_zero() {
        let a = Complex::new(0.0, 0.0);

        assert_eq!(a.re, 0.0);
        assert_eq!(a.im, 0.0);
    }

    #[test]
    fn test_new_with_negative() {
        let a = Complex::new(-1.0, -2.0);

        assert_eq!(a.re, -1.0);
        assert_eq!(a.im, -2.0);
    }

    #[test]
    fn test_complex_arg() {
        let tolerance = 1e-6;

        // nombre complexe dans le premier quadrant
        let complex = Complex::new(1.0, 1.0);
        let expected = std::f64::consts::FRAC_PI_4; // 45 degrés en radians
        let difference = (complex.arg() - expected).abs();
        assert!(difference < tolerance, "Failed at first quadrant");

        // nombre complexe dans le deuxième quadrant
        let complex = Complex::new(-1.0, 1.0);
        let expected = 3.0 * std::f64::consts::FRAC_PI_4; // 135 degrés en radians
        let difference = (complex.arg() - expected).abs();
        assert!(difference < tolerance, "Failed at second quadrant");

        // nombre complexe dans le troisième quadrant
        let complex = Complex::new(-1.0, -1.0);
        let expected = -3.0 * std::f64::consts::FRAC_PI_4; // -135 degrés en radians
        let difference = (complex.arg() - expected).abs();
        assert!(difference < tolerance, "Failed at third quadrant");

        // nombre complexe dans le quatrième quadrant
        let complex = Complex::new(1.0, -1.0);
        let expected = -std::f64::consts::FRAC_PI_4; // -45 degrés en radians
        let difference = (complex.arg() - expected).abs();
        assert!(difference < tolerance, "Failed at fourth quadrant");
    }
}
