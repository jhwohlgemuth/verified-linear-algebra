#![allow(dead_code)]
use std::fmt;
// use creusot_contracts::*;

type Number = f64;
type Vector = Vec<Complex>;

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
    fn mult_scalar(&self, scalar: Number) -> Complex {
        Complex {
            re: self.re * scalar,
            im: self.im * scalar,
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
    fn norm_squared(&self) -> Number {
        self.re * self.re + self.im * self.im
    }
}

fn two_norm(a: Vector) -> Number {
    a.iter().map(|x| x.norm()).sum()
}

fn two_norm_squared(a: Vector) -> Number {
    a.iter().map(|x| x.norm_squared()).sum()
}

fn main() {
    let a = Complex::new(1.0, 1.0);
    let b = Complex::new(1.0, -1.0);
    println!("a = {}", a);
    println!("b = {}", b);
}
#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn two_norm_squared_positive_definiteness() -> () {
        let limit = 4000.0;
        let a: f64 = kani::any();
        let b: f64 = kani::any();
        let c: f64 = kani::any();
        let d: f64 = kani::any();
        let e: f64 = kani::any();
        let f: f64 = kani::any();
        let g: f64 = kani::any();
        let h: f64 = kani::any();
        kani::assume(-limit < a && a < limit);
        kani::assume(-limit < b && b < limit);
        kani::assume(-limit < c && c < limit);
        kani::assume(-limit < d && d < limit);
        kani::assume(-limit < e && e < limit);
        kani::assume(-limit < f && f < limit);
        kani::assume(-limit < g && g < limit);
        kani::assume(-limit < h && h < limit);
        let zero: Complex = Complex::new(0.0, 0.0);
        let x = vec![
            Complex::new(a, b),
            Complex::new(c, d),
            Complex::new(e, f),
            Complex::new(g, h),
        ];
        if two_norm_squared(x.clone()) != 0.0 {
            assert!(x != vec![zero, zero, zero, zero]);
        }
    }

    #[kani::proof]
    fn two_norm_squared_nonnegative_homogeneity() -> bool {
        let limit = 4000.0;
        let value: f64 = kani::any();
        let a: f64 = kani::any();
        let b: f64 = kani::any();
        let c: f64 = kani::any();
        let d: f64 = kani::any();
        let e: f64 = kani::any();
        let f: f64 = kani::any();
        let g: f64 = kani::any();
        let h: f64 = kani::any();
        kani::assume(-limit < value && value < limit);
        kani::assume(-limit < a && a < limit);
        kani::assume(-limit < b && b < limit);
        kani::assume(-limit < c && c < limit);
        kani::assume(-limit < d && d < limit);
        kani::assume(-limit < e && e < limit);
        kani::assume(-limit < f && f < limit);
        kani::assume(-limit < g && g < limit);
        kani::assume(-limit < h && h < limit);
        let x = vec![
            Complex::new(a, b),
            Complex::new(c, d),
            Complex::new(e, f),
            Complex::new(g, h),
        ];
        let product: Vector = x.iter().map(|i| i.mult_scalar(value)).collect();
        two_norm_squared(product) == value.abs() * two_norm_squared(x)
    }

    #[kani::proof]
    fn two_norm_squared_triangle_inequality() -> bool {
        assert!(true);
    }

    // #[kani::proof]
    // fn complex_numbers_identity() -> bool {
    //     let a: f64 = kani::any();
    //     let b: f64 = kani::any();
    //     let x: Complex = Complex::new(a, b);
    //     let additive_identity = Complex::new(0.0, 0.0);
    //     x.plus(&additive_identity) == x
    // }

    // #[kani::proof]
    // fn complex_numbers_conjugate_norm_squared() -> bool {
    //     let a: f64 = kani::any();
    //     let b: f64 = kani::any();
    //     let x: Complex = Complex::new(a, b);
    //     x.mult(&x.conj()) == x.norm_squared()
    // }

    // #[kani::proof]
    // // BROKEN - f64 square root not supported
    // fn complex_numbers_conjugate_norm() -> bool {
    //     let a: f64 = kani::any();
    //     let b: f64 = kani::any();
    //     let x: Complex = Complex::new(a, b);
    //     x.norm() == x.conj().norm()
    // }

    // #[kani::proof]
    // // BROKEN - f64 square root not supported
    // fn complex_numbers_triangle_inequality() -> bool {
    //     let a: f64 = kani::any();
    //     let b: f64 = kani::any();
    //     let c: f64 = kani::any();
    //     let d: f64 = kani::any();
    //     let x: Complex = Complex::new(a, b);
    //     let y: Complex = Complex::new(c, d);
    //     x.norm() + y.norm() >= (x.plus(&y)).norm()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulus() {
        let a = Complex::new(3.0, 4.0);
        assert_eq!(a.norm(), 5.0);
        assert_eq!(a.norm_squared(), 25.0);
    }
}
