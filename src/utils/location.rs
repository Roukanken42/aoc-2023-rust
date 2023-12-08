use std::ops::{Add, Div, Mul, Sub};

use num::{one, zero, Bounded, Num, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location<T: Num> {
    x: T,
    y: T,
}

impl<T: Num> Location<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// TODO: wrong implementation
impl<T: Num + Bounded + Copy> Location<T> {
    #[allow(dead_code)]
    fn neighborhood(&self) -> Vec<Self> {
        vec![
            *self + Location::new(zero(), one()),
            *self - Location::new(zero(), one()),
            *self + Location::new(one(), zero()),
            *self - Location::new(one(), zero()),
        ]
    }
}

impl<T: Num> Zero for Location<T> {
    fn zero() -> Self {
        Location::new(zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Num> Add<Self> for Location<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Location::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num> Sub<Self> for Location<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Location::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Num + Copy> Mul<T> for Location<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Location::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Num + Copy> Div<T> for Location<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Location::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Num + Bounded> Bounded for Location<T> {
    fn min_value() -> Self {
        Location::new(T::min_value(), T::min_value())
    }

    fn max_value() -> Self {
        Location::new(T::max_value(), T::max_value())
    }
}

// TODO: move elsewhere
pub trait Utils2d<T> {
    fn get_2d(&self, loc: Location<usize>) -> Option<&T>;
}

impl<T> Utils2d<T> for Vec<Vec<T>> {
    fn get_2d(&self, loc: Location<usize>) -> Option<&T> {
        self.get(loc.y).and_then(|row| row.get(loc.x))
    }
}
