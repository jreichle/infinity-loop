use quickcheck::{Arbitrary, Gen};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

/// Holds position for x and y axis and offers basic arithmetic operators
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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

impl<A> Coordinate<A> {
    pub const fn of(value: A) -> Self
    where
        A: Copy,
    {
        Self {
            row: value,
            column: value,
        }
    }

    pub fn map<B, F: Fn(A) -> B>(self, transform: F) -> Coordinate<B> {
        Coordinate {
            row: transform(self.row),
            column: transform(self.column),
        }
    }

    /// applicative liftA2 combinator
    pub fn combine<B, C, F: Fn(A, B) -> C>(
        self,
        other: Coordinate<B>,
        transform: F,
    ) -> Coordinate<C> {
        Coordinate {
            row: transform(self.row, other.row),
            column: transform(self.column, other.column),
        }
    }
}

impl<A> Coordinate<Option<A>> {
    pub fn sequence(self) -> Option<Coordinate<A>> {
        self.row
            .and_then(|r| self.column.map(|c| Coordinate { row: r, column: c }))
    }
}

impl<A: Display> Display for Coordinate<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row: {:04}, column: {:04})", self.row, self.column)
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
