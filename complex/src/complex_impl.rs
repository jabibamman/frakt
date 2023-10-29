// Les implémentations de  pour les nombres complexes
impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn add(self, other: &Self) -> Self {
        Complex::new(self.re + other.re, self.im + other.im)
    }

    pub fn mul(self, other: &Self) -> Self {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    pub fn square(&self) -> Self {
        self.mul(&self)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

fn main() {}
