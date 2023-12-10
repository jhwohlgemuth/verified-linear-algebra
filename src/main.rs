#![allow(dead_code)]
use std::fmt;
use creusot_contracts::*;

type Number = f64;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Complex {
    re: Number,
    im: Number,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operator = if Number::signum(self.im) == 1.0 {
            "+"
        } else {
            "-"
        };
        write!(f, "{} {} {}i", self.re, operator, self.im.abs())
    }
}

impl Complex {
    fn new(re: Number, im: Number) -> Complex {
        Complex { re, im }
    }
    fn mult(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    fn plus(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    fn conj(&self) -> Complex {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
    fn norm(&self) -> Number {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

fn main() {
    let a = Complex::new(1.0, 1.0);
    let b = Complex::new(1.0, -1.0);
    println!("a = {}", a);
    println!("b = {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 1.0);
        let i = Complex::new(0.0, 0.0);
        assert_eq!(a.plus(&i), a);
    }
}
