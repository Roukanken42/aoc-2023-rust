use std::ops::{Add, Div, Mul, Sub};

use num::{one, zero, Bounded, Num, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Location<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy + PartialOrd> Location<T> {
    #[allow(dead_code)]
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            *self + Location::new(zero(), one()),
            *self + Location::new(one(), one()),
            *self + Location::new(one(), zero()),
            *self + Location::new(one(), zero()) - Location::new(zero(), one()),
            *self - Location::new(zero(), one()),
            *self - Location::new(one(), one()),
            *self - Location::new(one(), zero()),
            *self - Location::new(one(), zero()) + Location::new(zero(), one()),
        ]
    }

    pub fn iter_range(self, end: Location<T>) -> impl Iterator<Item = Location<T>> {
        SquareIterator {
            next: self,
            next_row: self + Location::new(zero(), one()),
            end,
        }
    }
}

struct SquareIterator<T: Num + Copy + PartialOrd> {
    next: Location<T>,
    next_row: Location<T>,
    end: Location<T>,
}

impl<T: Num + Copy + PartialOrd> Iterator for SquareIterator<T> {
    type Item = Location<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.x < self.end.x {
            let result = self.next;
            self.next = self.next + Location::new(one(), zero());

            return Some(result);
        }

        if self.next_row.y < self.end.y {
            self.next = self.next_row;
            self.next_row = self.next_row + Location::new(zero(), one());

            return self.next();
        }

        None
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
