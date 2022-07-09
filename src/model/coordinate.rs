use quickcheck::{Arbitrary, Gen};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

/// Holds position for x and y axis and offers basic arithmetic operators
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coordinate<A> {
    pub row: A,
    pub column: A,
}

// how to implement for generic numeric value?
impl Coordinate<isize> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<isize> = Self { row: 0, column: 0 };
}

impl Coordinate<usize> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<usize> = Self { row: 0, column: 0 };
}

impl<A: Display> Display for Coordinate<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {:04}, y: {:04})", self.row, self.column)
    }
}

impl<A: Add + Add<Output = A>> Add for Coordinate<A> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl<A: AddAssign + Add<Output = A> + Copy> AddAssign for Coordinate<A> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            row: self.row + other.row,
            column: self.column + other.column,
        };
    }
}

impl<A: Sub + Sub<Output = A>> Sub for Coordinate<A> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl<A: SubAssign + Sub<Output = A> + Copy> SubAssign for Coordinate<A> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            row: self.row - other.row,
            column: self.column - other.column,
        };
    }
}

impl<A: Neg + Neg<Output = A>> Neg for Coordinate<A> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            row: -self.row,
            column: -self.column,
        }
    }
}

impl<A: Arbitrary> Arbitrary for Coordinate<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            row: A::arbitrary(g),
            column: A::arbitrary(g),
        }
    }
}

// omit testing
