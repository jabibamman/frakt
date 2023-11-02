use shared::types::complex::Complex;

pub trait ComplexOperations {
    fn new(re: f64, im: f64) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn square(&self) -> Self;
    fn magnitude_squared(&self) -> f64;
    fn norm(&self) -> f64;
}

impl ComplexOperations for Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    fn add(&self, other: &Self) -> Self {
        Complex::new(self.re + other.re, self.im + other.im)
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
}
