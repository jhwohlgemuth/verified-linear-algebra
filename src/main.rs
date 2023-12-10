#[derive(Debug, PartialEq, Copy, Clone)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
    fn add(&self, other: &Complex) -> Complex {
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
    fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 1.0);
        let i = Complex::new(0.0, 0.0);
        assert_eq!(a.add(&i), a);
    }
}