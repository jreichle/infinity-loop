use quickcheck::{Arbitrary, Gen};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

/// Holds position for x and y axis and offers basic arithmetic operators
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coordinate<A> {
    pub row: A,
    pub column: A,
}

// it is currently impossible to generically cast number literals in a constant expression outside of macros
impl Coordinate<u8> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<u8> = Self::of(0);
}

impl Coordinate<u16> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<u16> = Self::of(0);
}

impl Coordinate<u32> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<u32> = Self::of(0);
}

impl Coordinate<u64> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<u64> = Self::of(0);
}

impl Coordinate<u128> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<u128> = Self::of(0);
}

impl Coordinate<usize> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<usize> = Self::of(0);
}

impl Coordinate<i8> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<i8> = Self::of(0);
}

impl Coordinate<i16> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<i16> = Self::of(0);
}

impl Coordinate<i32> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<i32> = Self::of(0);
}

impl Coordinate<i64> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<i64> = Self::of(0);
}

impl Coordinate<i128> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<i128> = Self::of(0);
}

impl Coordinate<isize> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Coordinate<isize> = Self::of(0);
}

impl<A> Coordinate<A> {
    #[inline(always)]
    pub const fn of(value: A) -> Self
    where
        A: Copy,
    {
        Self::new(value, value)
    }

    #[inline(always)]
    pub const fn new(row: A, column: A) -> Self {
        Coordinate { row, column }
    }

    #[inline(always)]
    pub fn map<B, F: Fn(A) -> B>(self, transform: F) -> Coordinate<B> {
        Coordinate {
            row: transform(self.row),
            column: transform(self.column),
        }
    }

    /// applicative liftA2 combinator
    #[inline(always)]
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

    #[inline(always)]
    pub fn sum(self) -> A::Output
    where
        A: Add,
    {
        self.row + self.column
    }

    #[inline(always)]
    pub fn product(self) -> A::Output
    where
        A: Mul,
    {
        self.row * self.column
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
