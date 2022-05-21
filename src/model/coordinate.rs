use quickcheck::{Arbitrary, Gen};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

/// Holds position for x and y axis and offers basic arithmetic operators
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coordinate<A> {
    x: A,
    y: A,
}

// how to implement for generic numeric value?
impl Coordinate<i32> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<i32> = Self { x: 0, y: 0 };
}

impl Coordinate<u32> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<u32> = Self { x: 0, y: 0 };
}

impl<A: Display> Display for Coordinate<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {:04}, y: {:04})", self.x, self.y)
    }
}

impl<A: Add + Add<Output = A>> Add for Coordinate<A> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<A: AddAssign + Add<Output = A> + Copy> AddAssign for Coordinate<A> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<A: Sub + Sub<Output = A>> Sub for Coordinate<A> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<A: SubAssign + Sub<Output = A> + Copy> SubAssign for Coordinate<A> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<A: Neg + Neg<Output = A>> Neg for Coordinate<A> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<A: Arbitrary> Arbitrary for Coordinate<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            x: A::arbitrary(g),
            y: A::arbitrary(g),
        }
    }
}

// omit testing
