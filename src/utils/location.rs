use std::iter::successors;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num::{one, zero, Bounded, Num, Signed, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Location<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy + PartialOrd> Location<T> {
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

    pub fn iter_range(self, end: Location<T>) -> SquareIterator<T> {
        SquareIterator {
            next: self,
            next_row: self + Location::new(zero(), one()),
            end,
        }
    }

    pub fn iter_ray(self, direction: Location<T>) -> impl Iterator<Item = Location<T>> {
        successors(Some(self), move |&current| Some(current + direction))
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

impl<T: Num + Copy + Signed> Neg for Location<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Location::new(-self.x, -self.y)
    }
}

// macro_rules! impl_location_from {
//     (for $($t:ty),+) => {
//         $(
//             impl<T: From<$t>> From<Location<$t>> for Location<T> {
//                 fn from(location: Location<$t>) -> Self {
//                     Location::new(location.x.into(), location.y.into())
//                 }
//             }
//
//             impl<T: TryFrom<$t>> TryFrom<Location<$t>> for Location<T> {
//                 type Error = <T as TryFrom<$t>>::Error;
//
//                 fn try_from(location: Location<$t>) -> Result<Self, Self::Error> {
//                     Ok(Location::new(
//                         T::try_from(location.x)?,
//                         T::try_from(location.y)?,
//                     ))
//                 }
//             }
//         )+
//     };
// }
//
// impl_location_from!(for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Into<F>, F> From<Location<F>> for Location<T> {
    fn from(location: Location<F>) -> Self {
        Location::new(location.x.into(), location.y.into())
    }
}

// impl<T: TryFrom<F>, F> TryFrom<Location<F>> for Location<T> {
//     type Error = <T as TryFrom<F>>::Error;
//
//     fn try_from(location: Location<F>) -> Result<Self, Self::Error> {
//         Ok(Location::new(
//             T::try_from(location.x)?,
//             T::try_from(location.y)?,
//         ))
//     }
// }

// TODO: move elsewhere
pub trait Access2d<T> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T>;
    fn set_2d(&mut self, loc: Location<i32>, element: T) -> Option<()>;
    fn iter_2d_keys(&self) -> SquareIterator<T>;
}

impl<T> Access2d<T> for Vec<Vec<T>> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T> {
        self.get(usize::try_from(loc.y).ok()?)
            .and_then(|row| row.get(usize::try_from(loc.x).ok()?))
    }

    fn set_2d(&mut self, loc: Location<i32>, element: T) -> Option<()> {
        self.get_mut(usize::try_from(loc.y).ok()?)
            .and_then(|row| {
                row[usize::try_from(loc.x).ok()?] = element;
                Some(())
            })
            .map(|_| ())
    }

    fn iter_2d_keys(&self) -> SquareIterator<usize> {
        Location::new(0, 0).iter_range(Location::new(self[0].len(), self.len()))
    }
}
