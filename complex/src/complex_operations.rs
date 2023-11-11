use shared::types::complex::Complex;

pub trait ComplexOperations {
    fn new(re: f64, im: f64) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn square(&self) -> Self;
    fn magnitude_squared(&self) -> f64;
    fn norm(&self) -> f64;
    fn div(&self,other:&self) -> self;
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
    fn div(&self,other:&self) -> self{
        let denominator = other.magnitude_squared();
        Complex::new(
            (self.re * other.re + self.im * other.im) / denominator,
            (self.im * other.re - self.re * other.im) / denominator,
        )

    }

}

#[cfg(test)]
mod complex_tests {
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
