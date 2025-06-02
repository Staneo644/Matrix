use std::ops::{Add, Sub, Mul, Div, Neg, SubAssign, MulAssign, DivAssign};
use num_traits::{Zero, One};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl One for Complex {
    fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Complex::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.re == 0.0 && self.im == 0.0
    }
}


impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn conj(&self) -> Self {
        Complex { re: self.re, im: -self.im }
    }

    pub fn modulus_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn modulus(&self) -> f64 {
        self.modulus_squared().sqrt()
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denom = other.modulus_squared();
        Complex::new(
            (self.re * other.re + self.im * other.im) / denom,
            (self.im * other.re - self.re * other.im) / denom,
        )
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Complex) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Complex) {
        let temp = *self;
        self.re = temp.re * other.re - temp.im * other.im;
        self.im = temp.re * other.im + temp.im * other.re;
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, other: Complex) {
        let denom = other.modulus_squared();
        let temp = *self;
        self.re = (temp.re * other.re + temp.im * other.im) / denom;
        self.im = (temp.im * other.re - temp.re * other.im) / denom;
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im > 0.0 {
            write!(f, "{} + {}i", self.re, self.im)
        } else if self.im < 0.0 {
            write!(f, "{} - {}i", self.re, -self.im)
        } else {
            write!(f, "{}", self.re)
        }
    }
}
