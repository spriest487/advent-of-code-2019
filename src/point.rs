#![allow(unused)]

use std::ops::{AddAssign, Sub, Add, Mul, SubAssign, MulAssign};
use std::fmt;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn normalize(&self) -> Option<Self> {
        fn sign(x: i64) -> i64 {
            if x < 0 {
                -1
            } else if x > 0 {
                1
            } else {
                0
            }
        }

        if self.x != 0 && self.y == 0 {
            Some(Self { x: sign(self.x), y: 0 })
        } else if self.x == 0 && self.y != 0 {
            Some(Self { x: sign(self.x), y: 0 })
        } else {
            None
        }
    }

    pub fn manhattan_len(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        (*self - *other).manhattan_len()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<i64> for Point {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs
    }
}

pub fn visualize_points<F: Fn(&Point) -> char>(w: i64, h: i64, f: F) {
    for y in 0..h {
        for x in 0..w {
            let point = Point::new(x, y);
            let vis = f(&point);
            print!("{}", vis);
        }
        println!();
    }
}