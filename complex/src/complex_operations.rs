use shared::types::complex::Complex;

pub trait ComplexOperations {
    fn new(re: f64, im: f64) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn square(&self) -> Self;
    fn magnitude_squared(&self) -> f64;
    fn norm(&self) -> f64;
    fn sin(&self) -> Self;
    fn exp(&self) -> Self;
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

    ///(exp(iz) - exp(-iz)) / 2i
    fn sin(&self) -> Self {
        let mut mul_pos_i = self.mul(&Complex::new(0.0, 1.0));
        let mut mul_neg_i = self.mul(&Complex::new(0.0, -1.0));

        mul_pos_i = mul_pos_i.exp();
        mul_neg_i = mul_neg_i.exp();

        let calcul_top = mul_pos_i.sub(&mul_neg_i);
        Complex::new(calcul_top.im / 2.0, (-calcul_top.re) / 2.0)
    }

    ///exp(a+ib) = exp(a) * exp(ib) = exp(a) * (cos(b) + isin(b))
    fn exp(&self) -> Self {
        let exp_re = Complex::new(self.re.exp(), 0.0);
        let exp_im = Complex::new(self.im.cos(), self.im.sin());
        exp_re.mul(&exp_im)
    }
}

#[cfg(test)]
mod complex_tests {
    use std::f64::consts::PI;
    use super::*;

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
        let a = Complex::new(0.0, PI/2.0);
        let result = a.exp();

        assert_eq!((result.re * 100000.0).round() / 100000.0, 0.0);
        assert_eq!((result.im * 100000.0).round() / 100000.0, 1.0);
    }

    #[test]
    fn test_exp_where_im_equal_minus_half_pi() {
        let a = Complex::new(0.0, -PI/2.0);
        let result = a.exp();

        assert_eq!((result.re * 100000.0).round() / 100000.0, 0.0);
        assert_eq!((result.im * 100000.0).round() / 100000.0, -1.0);
    }

    #[test]
    fn test_exp_where_im_equal_pi() {
        let a = Complex::new(0.0, PI);
        let result = a.exp();

        assert_eq!((result.re * 100000.0).round() / 100000.0, -1.0);
        assert_eq!((result.im * 100000.0).round() / 100000.0, 0.0);
    }

    #[test]
    fn test_exp_where_im_equal_minus_pi() {
        let a = Complex::new(0.0, -PI);
        let result = a.exp();

        assert_eq!((result.re * 100000.0).round() / 100000.0, -1.0);
        assert_eq!((result.im * 100000.0).round() / 100000.0, 0.0);
    }

    #[test]
    fn test_exp_z_mul_exp_minus_z_equal_1() {
        let z = Complex::new(353.5, -4520.3);
        let left = z.exp();
        let right = z.mul(&Complex::new(-1.0, 0.0)).exp();
        let result = left.mul(&right);

        assert_eq!((result.re * 100000.0).round() / 100000.0, 1.0);
        assert_eq!((result.im * 100000.0).round() / 100000.0, 0.0);
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
}
