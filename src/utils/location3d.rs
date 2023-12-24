use std::iter::successors;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num::{one, zero, Bounded, Num, Signed, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location3d<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Location3d<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn map<U: Num, F: Fn(T) -> U>(self, f: F) -> Location3d<U> {
        Location3d::new(f(self.x), f(self.y), f(self.z))
    }

    pub fn try_map<U: Num, E, F: Fn(T) -> Result<U, E>>(self, f: F) -> Result<Location3d<U>, E> {
        Ok(Location3d::new(f(self.x)?, f(self.y)?, f(self.z)?))
    }
}

impl<T: Num + Copy + Signed> Location3d<T> {
    pub fn manhattan_distance(self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn iter_adjacent(self) -> impl IntoIterator<Item = Location3d<T>> {
        [
            Location3d::new(zero(), one::<T>(), zero()),
            Location3d::new(one::<T>(), zero(), zero()),
            Location3d::new(zero(), -one::<T>(), zero()),
            Location3d::new(-one::<T>(), zero(), zero()),
            Location3d::new(zero(), zero(), one::<T>()),
            Location3d::new(zero(), zero(), -one::<T>()),
        ]
        .map(move |direction| self + direction)
    }
}

impl<T: Num + Copy + PartialOrd> Location3d<T> {
    pub fn iter_ray(self, direction: Location3d<T>) -> impl Iterator<Item = Location3d<T>> {
        successors(Some(self), move |&current| Some(current + direction))
    }
}

impl<T: Num> Zero for Location3d<T> {
    fn zero() -> Self {
        Location3d::new(zero(), zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T: Num> Add<Self> for Location3d<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Location3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Num> Sub<Self> for Location3d<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Location3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Num + Copy> Mul<T> for Location3d<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Location3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Num + Copy> Div<T> for Location3d<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Location3d::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Num + Bounded> Bounded for Location3d<T> {
    fn min_value() -> Self {
        Location3d::new(T::min_value(), T::min_value(), T::min_value())
    }

    fn max_value() -> Self {
        Location3d::new(T::max_value(), T::max_value(), T::max_value())
    }
}

impl<T: Num + Copy + Signed> Neg for Location3d<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Location3d::new(-self.x, -self.y, -self.z)
    }
}

pub mod direction3d {
    use super::Location3d;

    pub const LEFT: Location3d<i32> = Location3d::new(-1, 0, 0);
    pub const RIGHT: Location3d<i32> = Location3d::new(1, 0, 0);
    pub const FORWARD: Location3d<i32> = Location3d::new(0, -1, 0);
    pub const BACKWARDS: Location3d<i32> = Location3d::new(0, 1, 0);
    pub const UP: Location3d<i32> = Location3d::new(0, 0, 1);
    pub const DOWN: Location3d<i32> = Location3d::new(0, 0, -1);
}
