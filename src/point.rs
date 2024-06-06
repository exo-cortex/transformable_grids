use std::cmp::PartialEq;

use derive_more::{Add, AddAssign, Mul, Sub};

#[derive(Add, AddAssign, Mul, Sub, Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let expected = Point { x: 1.0, y: 1.0 };
        assert_eq!(Point::new(1.0, 1.0), expected);
    }

    #[test]
    fn add() {
        let expected = Point { x: 1.0, y: 1.0 };
        assert_eq!(Point::new(2.0, 2.0), expected + expected);
    }

    #[test]
    fn mul() {
        let expected = Point { x: 1.0, y: 1.0 };
        assert_eq!(Point::new(2.0, 2.0), expected * 2.0);
    }
}
