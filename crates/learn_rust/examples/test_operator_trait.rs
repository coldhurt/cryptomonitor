use std::{ops, fmt::Display};

struct ComplexNumber {
    real: f64,
    imag: f64,
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl ops::Add for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

fn main(){

  let complex1 = ComplexNumber{real: 1.0, imag: 2.0};
  let complex2 = ComplexNumber{real: 3.0, imag: 4.0};

  let result = complex1 + complex2;
  println!("result: {}", result);

}