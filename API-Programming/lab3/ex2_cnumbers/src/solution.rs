use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref};
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug)]
pub struct ComplexNumber {
    real: f64,
    imag: f64
}

#[derive(Debug, PartialEq)]
pub enum  ComplexNumberError{
    ImaginaryNotZero
}

impl ComplexNumber {
    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real, imag }
    }
    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }

    pub fn from_real(real: f64) -> ComplexNumber {
        ComplexNumber { real, imag: 0.0 }
    }

    pub fn to_tuple(&self) -> (f64, f64) {
        (self.real, self.imag)
    }
}
impl AsRef<ComplexNumber> for ComplexNumber {
    fn as_ref(&self) -> &ComplexNumber {
        &self
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl<T: Into<ComplexNumber>> Add<T> for ComplexNumber
    {
    type Output = ComplexNumber;

    fn add(self, other: T) -> Self {
        let cother:ComplexNumber = other.into();
        ComplexNumber::new(self.real + cother.real, self.imag + cother.imag)
    }
}

impl Add<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber::new(self.real + other.real, self.imag + other.imag)
    }
}

impl From<f64> for ComplexNumber {
    fn from(value: f64) -> Self {
        Self::from_real(value)
    }
}

impl TryFrom<ComplexNumber> for f64 {
    type Error = ComplexNumberError;
    fn try_from(value: ComplexNumber) -> Result<Self, Self::Error> {
        if value.imag != 0.0{
            Err(ComplexNumberError::ImaginaryNotZero)
        } else {
            Ok(value.real)
        }
    }
}

impl From<&ComplexNumber> for ComplexNumber {
    fn from(value: &ComplexNumber) -> Self {
        Self::new(value.real, value.imag)
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, other: Self) {
        (*self).real += other.real;
        (*self).imag += other.imag;
    }
}

impl Default for ComplexNumber {
    fn default() -> Self {
        ComplexNumber::new(0.0, 0.0)
    }
}


impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl Eq for ComplexNumber {}

// Order only by real part (ignoring imaginary part)
impl PartialOrd for ComplexNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.real.partial_cmp(&other.real)
    }
}

// Implement Ord based on real first, then imaginary
impl Ord for ComplexNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.real.partial_cmp(&other.real) {
            Some(Ordering::Equal) => self.imag.partial_cmp(&other.imag).unwrap(),
            Some(ordering) => ordering,
            None => Ordering::Greater, // Handle NaN by treating it as the largest
        }
    }
}

impl AsRef<f64> for ComplexNumber{
    fn as_ref(&self) -> &f64 {
        &self.real
    }
}

impl AsMut<f64> for ComplexNumber{
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.real
    }
}
