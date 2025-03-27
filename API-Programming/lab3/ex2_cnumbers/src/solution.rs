use std::fmt::{Display, Formatter};
use std::ops::Add;

pub struct ComplexNumber {
    real: f64,
    imag: f64
}

pub struct ComplexNumberError{}

impl ComplexNumber {
    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real: real, imag: imag }
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }

    pub fn from_real(real: f64) -> ComplexNumber {
        ComplexNumber { real: real, imag: 0.0 }
    }

    pub fn to_tuple(&self) -> (f64, f64) {
        (self.real, self.imag)
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl<T: Into<ComplexNumber>> Add<T> for ComplexNumber{
    type Output = ComplexNumber;

    fn add(self, other: T) -> Self {
        let cother = other.into();
        ComplexNumber::new(self.real + cother.real, self.imag + cother.imag)
    }
}

impl From<f64> for ComplexNumber {
    fn from(value: f64) -> Self {
        Self::from_real(value)
    }
}
